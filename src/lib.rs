use std::{fs::{self, File}, path, io::Write, collections::HashMap};
use tera::{Tera, Context};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Conf {
	out_put: Option<String>,
	templates: Option<String>,
	entity_source: EntitySource,
	context: HashMap<String, String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EntitySource {
	type_: Option<String>,
	url: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Entity {
	pub name: String,
	pub low_case_name: String,
	pub fileds: Vec<EntityFiled>,
}

impl Entity {
	pub fn new(entity_body: String) -> Self {
		let mut name = "";
		let mut fileds = Vec::new();
		let lines = entity_body.split("\n");
		for (i, line) in lines.enumerate() {
			let line = line.trim();
			if i == 0 {
				// 截取entity名称
				name = line.split(" ").nth(1).unwrap();
				continue;
			}
			if i > 0 {
				let filed_name = line.split(" ").nth(0).unwrap();
				let mut filed_type = line.split(" ").nth(1).unwrap();
				if filed_type.ends_with(",") {
						filed_type = &filed_type[..filed_type.len()-1];
				}
				// println!("filed_name: {} filed_type: {}", filed_name, filed_type);
				fileds.push(EntityFiled { 
					name: filed_name.to_string(), 
					type_: EntityFiledType::new(filed_type), 
					is_required: false 
				});	
			}
		}
		Entity {
			name: name.to_string(),
			low_case_name: name.to_string().to_lowercase(),
			fileds,
		}
	}	
	
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct EntityFiled {
	pub name: String,
	pub type_: EntityFiledType,
	pub is_required: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum EntityFiledType {
	String,
	Integer,
	Long,
	BigDecimal,
	Boolean,
	LocalDateTime,
	ZonedDateTime,
	Enum,
	Entity,
}

impl EntityFiledType {
	pub fn new(type_: &str) -> Self {
		// 匹配枚举
		match type_ {
			"String" => EntityFiledType::String,
			"Integer" => EntityFiledType::Integer,
			"Long" => EntityFiledType::Long,
			"BigDecimal" => EntityFiledType::BigDecimal,
			"Boolean" => EntityFiledType::Boolean,
			"LocalDateTime" => EntityFiledType::LocalDateTime,
			"ZonedDateTime" => EntityFiledType::ZonedDateTime,
			_ => panic!("不支持的类型 {}", type_),
		}

	}	
}

pub fn gen(from: &str, to: &str, tera: &mut Tera, context: &mut Context, entitys : &[&Entity]) {
	println!("读取{:?}", from);
	if !std::path::Path::new(from).exists() {
		println!("不存在{:?}", from);
		return;
	}
	fs::read_dir(from).unwrap().for_each(|entry| {
		let path_buf = entry.unwrap().path();
		println!("处理 {:?}", path_buf);
		let mut file_name = path_buf.file_name().unwrap().to_str().unwrap().to_string();
		if path_buf.is_dir() {
			if file_name.contains("{{") {
				file_name = tera.render_str(file_name.as_str(), &context).unwrap().to_string();
			}	
			fs::create_dir_all(path::Path::new(&format!("{}/{}", to, file_name.as_str()))).unwrap();
			gen(path_buf.to_str().unwrap(), format!("{}/{}", to, file_name.as_str()).as_str(), tera, context, entitys);
		}
		if path_buf.is_file() {
			let file_content = fs::read_to_string(path_buf.to_str().unwrap()).unwrap();
			for ele in entitys {
				println!("{:?}", ele.name);
				context.insert("entity", *ele);

				let entity_name = tera.render_str(file_name.as_str(), &context).unwrap().to_string();
				let out_content = tera.render_str(file_content.as_str(), context).unwrap();
				let mut file = File::create(format!("{}/{}", to, entity_name.as_str())).unwrap();
				file.write_all(out_content.as_bytes()).unwrap();
			}
		}
	});
}

mod entity_source;