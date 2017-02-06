#![allow(dead_code, unused_imports)]
use nom::{IResult, digit, space, multispace, alpha, alphanumeric};
use std::str;
use std::str::FromStr;
use node::{Node, Operator};
use datatype::DataType;

named!(i64_digit<Node>,
  map!(
    map_res!(
      digit,
      str::from_utf8
    ),
    |s| {
      let v = FromStr::from_str(s).unwrap();
      Node::value(DataType::Int(v), 0)
    }
  )
);

named!(identifier<Node>,
  map!(
    map_res!(
      escaped!(call!(alphanumeric), '\\', is_a!("\"n\\")),
      str::from_utf8
    ),
    |s| {
      Node::ident_new(String::from(s), 0)
    }
  )
);

named!(primary<Node>,
  alt!(
    // i64_digit
    i64_digit | identifier
  )
);

named!(operator<Operator>,
  map!(
    map_res!(
      alt!(
        tag!("+") |
        tag!("-") |
        tag!("*") |
        tag!("/") |
        alphanumeric
      ),
      str::from_utf8
    ),
    |s| {
      Operator::Call(String::from(s))
    }
  )
);

named!(arg<Node>,
  alt!(primary | expr)
);

named!(expr1<Node>,
  chain!(
    multispace? ~
    op: operator ~
    multispace ~
    nds: separated_list!(multispace, arg) ~
    multispace?,
    || {
      return Node::call_new(op, nds, 0)
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

named!(statement<Node>,
  chain!(
    nd: expr ~
    multispace?,
    || {
      return nd
    }
  )
);

named!(program<&[u8], Vec<Node>>,
  many0!(
    statement
  )
);

pub fn node_parse(input: String) -> Vec<Node> {
  match program(input.as_bytes()) {
    IResult::Done(_, nodes) => nodes,
    IResult::Error(error) => panic!("Error: {:?}", error),
    IResult::Incomplete(needed) => panic!("Incomplete: {:?}", needed),
  }
}
