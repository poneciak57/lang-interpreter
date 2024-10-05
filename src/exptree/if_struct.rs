use std::fmt;
use miette::Error;

use crate::{evaluator::Value, evaluator::Eval};

use super::ExprTree;


#[derive(Debug, Clone, PartialEq)]
pub struct If<'de> {
    cond: Box<ExprTree<'de>>,
    if_block: Box<ExprTree<'de>>,
    else_block: Option<Box<ExprTree<'de>>>
}

impl<'de: 'a, 'a> Eval<'a> for If<'de> {
    fn eval(&self, ctx: &crate::context::CtxTree<'a>) -> Result<Value, Error> {
        if self.cond.eval(ctx)?.into() {
            self.if_block.eval(ctx)
        } else {
            if let Some(ref else_block) = self.else_block {
                else_block.eval(ctx)
            } else {
                Ok(Value::Nil)
            }
        }
    }
}


impl<'de> If<'de> {
    pub fn new(cond: Box<ExprTree<'de>>, if_block: Box<ExprTree<'de>>, else_block: Option<Box<ExprTree<'de>>>) -> Self {
        Self { cond, if_block, else_block }
    }
}

impl fmt::Display for If<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref else_block) = self.else_block {
            write!(f, "(if {} {} {})", self.cond, self.if_block, else_block)
        } else {
            write!(f, "(if {} {})", self.cond, self.if_block)
        }
    }
}