use std::fmt;
use miette::Error;

use crate::{error::DefaultRuntimeError, evaluator::{Eval, Value}};

use super::{Atom, ExprTree};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Minus,
    Plus,
    Star,
    Slash,

    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Equal,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConOp<'de> {
    op: Op,
    lhs: Box<ExprTree<'de>>,
    rhs: Box<ExprTree<'de>>
}

impl<'de: 'a, 'a> Eval<'a> for ConOp<'de> {
    fn eval(&self, ctx: &crate::context::CtxTree<'a>) -> Result<Value, Error> {
        if self.op == Op::Equal {
            return if let ExprTree::Atom(Atom::Ident(id)) = *self.lhs {
                let set_res = ctx.set(id, self.lhs.eval(ctx)?);
                if set_res.is_err() { 
                    Err(DefaultRuntimeError {}.into()) // TODO change errors 
                } else {
                    Ok(Value::Nil)
                }
            } else {
                Err(DefaultRuntimeError {}.into()) // TODO change errors
            }
        }

        let left = self.lhs.eval(ctx)?;
        let right = self.rhs.eval(ctx)?;
        if matches!(left, Value::Event(_)) || matches!(right, Value::Event(_)) {
            return Err(DefaultRuntimeError {}.into()); // TODO change errors
        }
        match self.op {
            Op::Minus => left - right,
            Op::Plus => left + right,
            Op::Star => left * right,
            Op::Slash => left / right,

            Op::BangEqual => Ok(Value::Bool(left != right)),
            Op::EqualEqual => Ok(Value::Bool(left == right)),
            Op::LessEqual => Ok(Value::Bool(left <= right)),
            Op::GreaterEqual => Ok(Value::Bool(left >= right)),
            Op::Less => Ok(Value::Bool(left < right)),
            Op::Greater => Ok(Value::Bool(left > right)),
            Op::And => Ok(Value::Bool(left.into() && right.into())),
            Op::Or => Ok(Value::Bool(left.into() || right.into())),

            Op::Equal => unreachable!("already checked"),
        }
    }
}

impl<'de> ConOp<'de> {
    pub fn new(op: Op, lhs: Box<ExprTree<'de>>, rhs: Box<ExprTree<'de>>) -> Self {
        Self { op, lhs, rhs }
    }
}

impl fmt::Display for ConOp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.op, self.lhs, self.rhs)
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Plus => write!(f, "+"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::BangEqual => write!(f, "!="),
            Self::EqualEqual => write!(f, "=="),
            Self::LessEqual => write!(f, "<="),
            Self::GreaterEqual => write!(f, ">="),
            Self::Less => write!(f, "<"),
            Self::Greater => write!(f, ">"),
            Self::Equal => write!(f, "="),
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
        }
    }
}