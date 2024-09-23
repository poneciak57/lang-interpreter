use std::fmt;

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
}

impl fmt::Display for FnBlock<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = self.args.iter().map(|e| e.to_string()).collect::<String>();
        write!(f, "(fn {} {} {})", self.ident, args, self.block)
    }
}