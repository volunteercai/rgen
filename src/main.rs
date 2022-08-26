use std::{env, fs::{self, File}, path, io::Write, collections::HashMap};
use rgen::*;
use tera::{Tera, Context};

fn main() {
	// 读取当前目录的 配置文件 rgen.yaml
	let conf = path::Path::new("rgen.yaml");
	if !conf.exists() {
		println!("rgen.yaml not found");
	}
	let conf = fs::read_to_string("rgen.yaml").unwrap();
	let conf: Conf = serde_yaml::from_str(conf.as_str()).unwrap();
	println!("读取配置 {:?}", conf);
	// 校验内容
	
	// 获取命令行参数
	// let args = env::args().collect::<Vec<_>>();
	// 校验命令行参数

	let reg = fs::read_to_string(conf.entity_source.url.unwrap()).unwrap();
	
	// 解析.rge为entities
	let entities = parse_reg(&reg);
	// 生成代码
	let mut tera = Tera::default();
	let mut context = Context::new();

	for key in conf.context.keys() {
		context.insert(key, &conf.context.get(key))
	}

	gen(conf.templates.unwrap().as_str(), conf.out_put.unwrap().as_str(), &mut tera, &mut context, &entities);
}

fn parse_reg(f: &str) -> Vec<Entity> {
	let mut entities = Vec::new();
	let lines = f.split("}");
	for line in lines {
		let line = line.trim();
		println!("分解后 {}", line);
		if line.starts_with("entity") {
			// 截取entity名称
			let entity = Entity::new(line.to_string());
			entities.push(entity);
		}
	}
	entities
}