use std::fmt;
use miette::Error;

use crate::{context::CtxTree, error::DefaultRuntimeError, evaluator::{Eval, Event, Value}};

use super::ExprTree;

#[derive(Debug, Clone, PartialEq)]
pub struct FnBlock<'de> {
    ident: &'de str,
    args: Vec<&'de str>,
    block: Box<ExprTree<'de>>
}

impl<'de: 'a, 'a> Eval<'a> for FnBlock<'de> {
    fn eval(&self, ctx: &CtxTree<'a>) -> Result<Value, Error> {
        // here we just add fn to context
        ctx.insert_fn(self.ident, self.clone());
        Ok(Value::Event(Event::NoVal))
    }
}

impl<'de> FnBlock<'de> {
    pub fn new(ident: &'de str, args: Vec<&'de str>, block: Box<ExprTree<'de>>) -> Self {
        Self { ident, args, block }
    }

    pub fn exec(&self, ctx: &crate::context::CtxTree<'de>, args: Vec<Value>) -> Result<Value, Error> {
        let fork = ctx.fork();
        if self.args.len() != args.len() {
            return Err(DefaultRuntimeError {}.into()) // TODO change error
        }
        for i in 0..args.len() {
            fork.insert(self.args[i], args[i].clone());
        }

        self.block.eval(&fork)
    }
}

impl fmt::Display for FnBlock<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = self.args.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(" ");
        write!(f, "(fun {} ({}) {})", self.ident, args, self.block)
    }
}