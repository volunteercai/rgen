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
	let args = env::args().collect::<Vec<_>>();
	// 校验命令行参数

	// 解析.rg为entities
	
	// 生成代码
}

#[test]
fn test_parse_reg() {
	let reg = fs::read_to_string("entities.rge").unwrap();
	print!("{}", reg);
	// 解析.rge为entities
	let entities = parse_reg(&reg);
	println!("{:?}", entities);
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

#[test]
fn test_filed() {
	// let filed = EntityFiled {
	// 	name: "name".to_string(),
	// 	type_: EntityFiledType::new(Option::Some("String".to_string())),
	// 	is_required: false,
	// };
	// println!("{:?}", filed);
	let slice = &"Können"[..6];
	println!("{}", slice);
}