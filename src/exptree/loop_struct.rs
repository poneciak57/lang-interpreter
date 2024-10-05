use std::fmt;
use miette::Error;

use crate::{error::DefaultRuntimeError, evaluator::{Eval, Value}, exptree::Atom};

use super::ExprTree;

#[derive(Debug, Clone, PartialEq)]
pub struct Loop<'de> {
    condition: Box<ExprTree<'de>>,
    init: Option<Box<ExprTree<'de>>>,
    step: Option<Box<ExprTree<'de>>>,
    block: Box<ExprTree<'de>>
}

impl<'de: 'a, 'a> Eval<'a> for Loop<'de> {
    fn eval(&self, ctx: &crate::context::CtxTree<'a>) -> Result<Value, Error> {
        let outer_scope = ctx.fork();
        if let Some(ref init) = self.init {
            init.eval(&outer_scope)?;
        }

        while self.condition.eval(&outer_scope)?.into() {
            let b_val = self.block.eval(&outer_scope)?;
            if let Value::Event(e) = b_val {
                match e {
                    crate::evaluator::Event::Continue => (),
                    crate::evaluator::Event::Break(ret) => return Ok(*ret),
                    crate::evaluator::Event::Return(_) => return Ok(Value::Event(e)),
                    crate::evaluator::Event::NoVal => return Err(DefaultRuntimeError {}.into()), // TODO change error
                }
            }
            if let Some(ref step) = self.step {
                step.eval(&outer_scope)?;
            }
        }

        Ok(Value::Nil)
    }
}


impl<'de> Loop<'de> {
    pub fn new(condition: Box<ExprTree<'de>>, init: Option<Box<ExprTree<'de>>>, step: Option<Box<ExprTree<'de>>>, block: Box<ExprTree<'de>>) -> Self {
        Self { condition, init, step, block }
    }
}

impl fmt::Display for Loop<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let step = self.step.clone().unwrap_or(Box::new(ExprTree::Atom(Atom::Nil))).to_string();
        let var = self.init.clone().unwrap_or(Box::new(ExprTree::Atom(Atom::Nil))).to_string();
        let condition = self.condition.to_string();
        let block = self.block.to_string();
        write!(f, "(loop {var} {condition} {step} {block})")
    }
}