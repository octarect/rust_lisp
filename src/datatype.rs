#![allow(dead_code)]
use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::clone::Clone;
use env::Env;

#[derive(Copy)]
pub enum DataType {
  Int(i64),
  Func(fn(&mut Env, Vec<DataType>)->DataType),
  // Str(String),
  Null,
}

impl fmt::Display for DataType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      DataType::Int(v) => write!(f, "{}", v),
      DataType::Func(fp) => write!(f, "function"),
      // DataType::Str(v) => write!(f, "{}", v),
      _ => write!(f, "Not implemented type."),
    }
  }
}

impl fmt::Debug for DataType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      DataType::Func(fp) => write!(f, "function"),
      x => write!(f, "{}", x),
    }
  }
}

impl Add for DataType {
  type Output = DataType;
  fn add(self, d: DataType) -> DataType {
    match (self, d) {
      (DataType::Int(v1), DataType::Int(v2)) => DataType::Int(v1 + v2),
      (_, _) => DataType::Null,
    }
  }
}

impl Sub for DataType {
  type Output = DataType;
  fn sub(self, d: DataType) -> DataType {
    match (self, d) {
      (DataType::Int(v1), DataType::Int(v2)) => DataType::Int(v1 - v2),
      (_, _) => DataType::Null,
    }
  }
}

impl Mul for DataType {
  type Output = DataType;
  fn mul(self, d: DataType) -> DataType {
    match (self, d) {
      (DataType::Int(v1), DataType::Int(v2)) => DataType::Int(v1 * v2),
      (_, _) => DataType::Null,
    }
  }
}

impl Div for DataType {
  type Output = DataType;
  fn div(self, d: DataType) -> DataType {
    match (self, d) {
      (DataType::Int(v1), DataType::Int(v2)) => DataType::Int(v1 / v2),
      (_, _) => DataType::Null,
    }
  }
}

impl Clone for DataType {
  fn clone(&self) -> DataType {
    *self
  }
}
