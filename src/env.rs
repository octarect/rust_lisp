#![allow(dead_code)]

use datatype::DataType;

use std::collections::HashMap;

pub struct Env {
  parent: Option<Box<Env>>,
  table: HashMap<String, DataType>,
}

impl Env {
  pub fn global_new() -> Env {
    Env {parent: None, table: HashMap::new()}
  }
  pub fn local_new(p: Env) -> Env {
    Env {parent: Some(Box::new(p)), table: HashMap::new()}
  }
  pub fn set(&mut self, k: String, v: DataType) {
    self.table.insert(k, v);
  }
  pub fn get(&mut self, k: String) -> DataType {
    match self.table.get(&k) {
      Some(&v) => v,
      _ => panic!("Undefined variable: {}", k),
    }
  }
}
