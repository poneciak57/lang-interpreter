use std::fmt;

use crate::exptree::Atom;

use super::ExprTree;

#[derive(Debug, Clone, PartialEq)]
pub struct VarDecl<'de> {
    indent: &'de str,
    exp: Box<ExprTree<'de>>
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