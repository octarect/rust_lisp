#![allow(dead_code)]
use datatype::DataType;

#[derive(Debug)]
pub struct NodeHeader {
  pub lineno: u16,
}

#[derive(Debug)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
  Let,
  Call(String),
}

#[derive(Debug)]
pub enum Node {
  CallNode {
    header: NodeHeader,
    op: Operator,
    args: Vec<Node>,
  },
  ValueNode {
    header: NodeHeader,
    value: DataType,
  },
}

impl Node {
  pub fn eval(&self) -> DataType {
    match *self {
      Node::CallNode {ref op, ref args, ..} => {
        match *op {
          Operator::Add => args[0].eval() + args[1].eval(),
          Operator::Sub => args[0].eval() - args[1].eval(),
          Operator::Mul => args[0].eval() * args[1].eval(),
          Operator::Div => args[0].eval() / args[1].eval(),
          _ => DataType::Int(-1),
        }
      },
      Node::ValueNode {value, ..} => value,
    }
  }
  pub fn op(op: Operator, l: Node, r: Node, pos: u16) -> Node {
    Node::CallNode {
      header: NodeHeader {lineno: pos},
      op: op,
      args: vec![l, r],
    }
  }
  pub fn call_new(op: Operator, args: Vec<Node>, pos: u16) -> Node {
    Node::CallNode {
      header: NodeHeader {lineno: pos},
      op: op,
      args: args,
    }
  }
  pub fn value(val: DataType, pos: u16) -> Node {
    Node::ValueNode {
      header: NodeHeader {lineno: pos},
      value: val,
    }
  }
}
