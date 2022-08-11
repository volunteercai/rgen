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
}

fn parse_reg(f: &str) -> Vec<Entity> {
	let mut entities = Vec::new();
	let lines = f.lines();
	for line in lines {
		let line = line.trim();
		if line.starts_with("entity") {
			// 截取entity名称
			let name = line.split(" ").nth(1).unwrap();
			let mut entity = Entity {
				name: name.to_string(),
				low_case_name: name.to_string().to_lowercase(),
				fileds: Vec::new(),
			};
			entities.push(entity);
		}
	}
	entities
}