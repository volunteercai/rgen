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
