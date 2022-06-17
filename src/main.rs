use std::{env, fs::{self, File}, path, io::Write, collections::HashMap};
use rgen::*;
use tera::{Tera, Context};

fn main() {
	// 读取当前目录的 配置文件 rgen.yaml
	let conf = path::Path::new("rgen.yaml");
	if !conf.exists() {
		panic!("rgen.yaml not found");
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Conf {
	out_put: Option<String>,
	templates: Option<String>,
	entity_source: EntitySource,
	context: HashMap<String, String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct EntitySource {
	type_: Option<String>,
	url: Option<String>,
}

#[test]
fn test_gen() {
	if std::path::Path::new("./output/").exists() {
		fs::remove_dir_all(path::Path::new("./output/")).unwrap();
	}
	fs::create_dir_all(path::Path::new("./output/")).unwrap();

	let mut tera = Tera::default();
	let mut context = Context::new();
	context.insert("packageName", &"com.example.app");
	let user = Entity {
		name: "User".to_string(),
		low_case_name: "user".to_string(),
		fileds: vec![
			EntityFiled{
				name: "id".to_string(),
				type_: EntityFiledType::Long,
				is_required: true,
			},	
			EntityFiled{
				name: "name".to_string(),
				type_: EntityFiledType::String,
				is_required: true,
			},
			EntityFiled{
				name: "password".to_string(),
				type_: EntityFiledType::String,
				is_required: true,
			},
		],
	};
	let image = Entity {
		name: "Image".to_string(),
		low_case_name: "image".to_string(),
		fileds: vec![
			EntityFiled{
				name: "id".to_string(),
				type_: EntityFiledType::Long,
				is_required: true,
			},	
			EntityFiled{
				name: "name".to_string(),
				type_: EntityFiledType::String,
				is_required: true,
			},
			EntityFiled{
				name: "url".to_string(),
				type_: EntityFiledType::String,
				is_required: true,
			},
		],
	};
	let entitys = vec![&user, &image];
	rgen::gen("./templates/", "./output/", &mut tera, &mut context, &entitys);
}
