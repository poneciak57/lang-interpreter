use std::fmt;
use miette::Error;

use crate::{error::DefaultRuntimeError, evaluator::{Eval, Event, Value}};

use super::ExprTree;

// Unary operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UOp {
    Minus,
    Bang,
    Print,
    Return,
    Break,
    Group
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOp<'de> {
    op: UOp,
    lhs: Box<ExprTree<'de>>,
}

impl<'de: 'a, 'a> Eval<'a> for UnaryOp<'de> {
    fn eval(&self, ctx: &crate::context::CtxTree<'a>) -> Result<Value, Error> {
        let v = self.lhs.eval(ctx)?;
        if matches!(v, Value::Event(_)) {
            return Err(DefaultRuntimeError {}.into()) // TODO change error
        }
        match self.op {
            UOp::Minus => -v,
            UOp::Bang => !v,
            UOp::Print => {
                print!("{}", v);
                Ok(Value::Event(Event::NoVal))
            },
            UOp::Return => Ok(Value::Event(Event::Return(Box::new(v)))),
            UOp::Break => Ok(Value::Event(Event::Break(Box::new(v)))),
            UOp::Group => Ok(v),
        }
    }
}


impl<'de> UnaryOp<'de> {
    pub fn new(op: UOp, lhs: Box<ExprTree<'de>>) -> Self {
        Self { op, lhs }
    }
}

impl fmt::Display for UnaryOp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.op, self.lhs)
    }
}

impl fmt::Display for UOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UOp::Minus => write!(f, "-"),
            UOp::Bang => write!(f, "!"),
            UOp::Print => write!(f, "print"),
            UOp::Return => write!(f, "return"),
            UOp::Break => write!(f, "break"),
            UOp::Group => write!(f, "group"),
        }
    }
}