use std::fmt;

use crate::exptree::Atom;

use super::ExprTree;

#[derive(Debug, Clone, PartialEq)]
pub struct Loop<'de> {
    condition: Box<ExprTree<'de>>,
    var: Option<Box<ExprTree<'de>>>,
    step: Option<Box<ExprTree<'de>>>,
    block: Box<ExprTree<'de>>
}

impl<'de> Loop<'de> {
    pub fn new(condition: Box<ExprTree<'de>>, var: Option<Box<ExprTree<'de>>>, step: Option<Box<ExprTree<'de>>>, block: Box<ExprTree<'de>>) -> Self {
        Self { condition, var, step, block }
    }
}

impl fmt::Display for Loop<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let step = self.step.clone().unwrap_or(Box::new(ExprTree::Atom(Atom::Nil))).to_string();
        let var = self.var.clone().unwrap_or(Box::new(ExprTree::Atom(Atom::Nil))).to_string();
        let condition = self.condition.to_string();
        let block = self.block.to_string();
        write!(f, "(loop {var} {condition} {step} {block})")
    }
}