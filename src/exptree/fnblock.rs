use std::fmt;

use crate::context::Value;

use super::ExprTree;

#[derive(Debug, Clone, PartialEq)]
pub struct FnBlock<'de> {
    ident: &'de str,
    args: Vec<&'de str>,
    block: Box<ExprTree<'de>>
}

impl<'de> FnBlock<'de> {
    pub fn new(ident: &'de str, args: Vec<&'de str>, block: Box<ExprTree<'de>>) -> Self {
        Self { ident, args, block }
    }

    pub fn exec(&self, ctx: &crate::context::CtxTree, args: Vec<Value>) -> Value {
        // it will map names to the values and declare them on forked ctx
        todo!()
    }
}

impl fmt::Display for FnBlock<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = self.args.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(" ");
        write!(f, "(fun {} ({}) {})", self.ident, args, self.block)
    }
}