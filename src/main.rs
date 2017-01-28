// #![allow(dead_code, unused_imports)]
#[macro_use]
extern crate rust_lisp;

use rust_lisp::parse::node_parse;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::str;

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

  let node = node_parse(buf);
  println!("{}", node.eval());
}
