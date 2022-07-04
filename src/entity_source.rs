use crate::*;

pub trait EntitySourceWalker {
  fn load(&self) -> Vec<Entity>;
}

pub fn get_entity_walker(source: &EntitySource) -> Box<dyn EntitySourceWalker> {
  Box::new(RgeEntitySourceWalker::new(source))
}

pub struct RgeEntitySourceWalker {

}

impl RgeEntitySourceWalker {
  pub fn new(source: &EntitySource) -> Self {
    RgeEntitySourceWalker {

    }
  } 
}

impl EntitySourceWalker for RgeEntitySourceWalker {
  fn load(&self) -> Vec<Entity> {
    let mut entitys = Vec::new();
    entitys
  }
}

pub struct DbEntitySourceWalker {

}

// impl EntitySourceWalker for DbEntitySourceWalker {
//   fn load(s: &EntitySource) -> Vec<Entity> {
//     // 读取文件 
//     // 解析文件
//     let mut entities = Vec::new();
//     entities
//   } 
// }