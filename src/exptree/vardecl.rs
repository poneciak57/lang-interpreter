use std::fmt;
use miette::Error;

use crate::{error::DefaultRuntimeError, evaluator::{Eval, Event, Value}, exptree::Atom};

use super::ExprTree;

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl<'de> {
    indent: &'de str,
    exp: Box<ExprTree<'de>>
}

impl<'de: 'a, 'a> Eval<'a> for VarDecl<'de> {
    fn eval(&self, ctx: &crate::context::CtxTree<'a>) -> Result<Value, Error> {
        let v = self.exp.eval(ctx)?;

        if matches!(v, Value::Event(_)) {
            return Err(DefaultRuntimeError {}.into()) // TODO change errors
        }
        ctx.insert(self.indent, v);
        Ok(Value::Event(Event::NoVal))
    }
}


impl<'de> VarDecl<'de> {
    pub fn new(indent: &'de str, exp: Option<Box<ExprTree<'de>>>) -> Self {
        Self { indent, exp: exp.unwrap_or(Box::new(ExprTree::Atom(Atom::Nil))) }
    }
}

impl fmt::Display for VarDecl<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(var id({}) {})", self.indent, self.exp)
    }
}