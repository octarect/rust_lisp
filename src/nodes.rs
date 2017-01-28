#![allow(dead_code)]
use types::{DataType};

pub struct NodeHeader {
  pub lineno: u16,
}

pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
  Call(String),
}

pub enum Node {
  OpNode {
    header: NodeHeader,
    op: Operator,
    left: Box<Node>,
    right: Box<Node>,
  },
  ValueNode {
    header: NodeHeader,
    value: DataType,
  },
}

impl Node {
  pub fn eval(self) -> DataType {
    match self {
      Node::OpNode {op, left, right, ..} => {
        match op {
          Operator::Add => left.eval() + right.eval(),
          Operator::Sub => left.eval() - right.eval(),
          Operator::Mul => left.eval() * right.eval(),
          Operator::Div => left.eval() / right.eval(),
          _ => DataType::Int(-1),
        }
      },
      Node::ValueNode {value, ..} => value,
    }
  }
  pub fn op(op: Operator, l: Node, r: Node, pos: u16) -> Node {
    Node::OpNode {
      header: NodeHeader {lineno: pos},
      op: op,
      left: Box::new(l),
      right: Box::new(r),
    }
  }
  pub fn value(val: DataType, pos: u16) -> Node {
    Node::ValueNode {
      header: NodeHeader {lineno: pos},
      value: val,
    }
  }
}
