#![allow(dead_code)]
use datatype::DataType;
use env::Env;

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
  IdentNode {
    header: NodeHeader,
    id: String,
  }
}

impl Node {
  pub fn eval(&self, ctx: &mut Env) -> DataType {
    match *self {
      Node::CallNode {ref op, ref args, ..} => {
        match *op {
          Operator::Call(ref fname) => {
            let data = args.iter().map(|nd| nd.eval(ctx)).collect::<Vec<_>>();
            match ctx.get(fname) {
              DataType::Func(f) => f(ctx, data),
              _ => panic!("{} is not function", fname),
            }
          },
          _ => DataType::Int(-1),
        }
      },
      Node::ValueNode {value, ..} => value,
      Node::IdentNode {ref id, ..} => ctx.get(id)
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
  pub fn ident_new(s: String, pos: u16) -> Node {
    Node::IdentNode {
      header: NodeHeader {lineno: pos},
      id: s,
    }
  }
}
