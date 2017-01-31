#![allow(dead_code, unused_imports)]
use nom::{IResult, digit, space};
use std::str;
use std::str::FromStr;
use nodes::{Node, Operator};
use types::DataType;

named!(i64_digit<Node>,
  map!(
    map_res!(
      digit,
      str::from_utf8
    ),
    |s| {
      let v = FromStr::from_str(s).unwrap();
      return Node::value(DataType::Int(v), 0);
    }
  )
);

named!(primary<Node>,
  alt!(
    i64_digit
  )
);

named!(operator<Operator>,
  map!(
    map_res!(
      alt!(
        tag!("+") |
        tag!("-") |
        tag!("*") |
        tag!("/")
      ),
      str::from_utf8
    ),
    |s| {
      match s {
        "+" => Operator::Add,
        "-" => Operator::Sub,
        "*" => Operator::Mul,
        "/" => Operator::Div,
        x => Operator::Call(x.to_string())
      }
    }
  )
);

named!(arg<Node>,
  alt!(primary | expr)
);

named!(expr1<Node>,
  chain!(
    op: operator ~
    space ~
    nd0: arg ~
    space ~
    nd1: arg,
    || {
      return Node::op(op, nd0, nd1, 0)
    }
  )
);

named!(expr<Node>,
  delimited!(
    char!('('),
    expr1,
    char!(')')
  )
);

pub fn node_parse(input: String) -> Node {
  match expr(input.as_bytes()) {
    IResult::Done(_, node) => node,
    IResult::Error(error) => panic!("Error: {:?}", error),
    IResult::Incomplete(needed) => panic!("Incomplete: {:?}", needed),
  }
}
