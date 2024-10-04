use std::fmt;
use miette::Error;

use crate::{context::Value, evaluator::Eval, exptree::Atom};

use super::ExprTree;

#[derive(Debug, Clone, PartialEq)]
pub struct Loop<'de> {
    condition: Box<ExprTree<'de>>,
    init: Option<Box<ExprTree<'de>>>,
    step: Option<Box<ExprTree<'de>>>,
    block: Box<ExprTree<'de>>
}

impl<'de> Eval for Loop<'de> {
    fn eval(&self, ctx: &crate::context::CtxTree) -> Result<Value, Error> {
        todo!()
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