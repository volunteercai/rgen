use std::{fs::{self, File}, path, io::Write};
use tera::{Tera, Context};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Entity {
	pub name: String,
	pub low_case_name: String,
	pub fileds: Vec<EntityFiled>,
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