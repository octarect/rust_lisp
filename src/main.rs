// #![allow(dead_code, unused_imports)]
#[macro_use]
extern crate rust_lisp;

use rust_lisp::parse::node_parse;
use rust_lisp::env::Env;
use rust_lisp::datatype::DataType;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::str;

fn op_add(ctx: &mut Env, args: Vec<DataType>) -> DataType {
  args[1..].iter().fold(args[0], |acc, x| acc + *x)
}

fn op_sub(ctx: &mut Env, args: Vec<DataType>) -> DataType {
  args[1..].iter().fold(args[0], |acc, x| acc - *x)
}

fn op_mul(ctx: &mut Env, args: Vec<DataType>) -> DataType {
  args[1..].iter().fold(args[0], |acc, x| acc * *x)
}

fn op_div(ctx: &mut Env, args: Vec<DataType>) -> DataType {
  args[1..].iter().fold(args[0], |acc, x| acc / *x)
}

fn risp_print(ctx: &mut Env, args: Vec<DataType>) -> DataType {
  for v in args.iter() {
    println!("{}", v);
  }
  DataType::Null
}

// fn risp_let(ctx: &mut Env, args: Vec<DataType>) -> DataType {
  // ctx.set(args[0], args[1]);
  // DataType::Null
// }

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("Usage: risp <path>");
    return;
  }

  let path = Path::new(&args[1]);
  let display = path.display();

  let mut file = match File::open(&path) {
    Err(why) => panic!("Failed to open {} : {}", display, Error::description(&why)),
    Ok(file) => file,
  };

  let mut buf = String::new();
  let _ = file.read_to_string(&mut buf);

  let mut global = Env::global_new();

  global.set("+".to_string(), DataType::Func(op_add));
  global.set("-".to_string(), DataType::Func(op_sub));
  global.set("*".to_string(), DataType::Func(op_mul));
  global.set("/".to_string(), DataType::Func(op_div));
  global.set("print".to_string(), DataType::Func(risp_print));

  let nodes = node_parse(buf);
  for node in nodes.iter() {
    println!("{:#?}", node);
    // println!("{:#?}", node.eval(&mut global));
    node.eval(&mut global);
  }

}
