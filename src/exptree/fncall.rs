use std::fmt;

use super::ExprTree;

#[derive(Debug, Clone, PartialEq)]
pub struct FnCall<'de> {
    ident: &'de str,
    args: Vec<ExprTree<'de>>
}

impl<'de> FnCall<'de> {
    pub fn new(ident: &'de str, args: Vec<ExprTree<'de>>) -> Self {
        Self { ident, args }
    }
}

impl fmt::Display for FnCall<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = self.args.iter().map(|e| e.to_string()).collect::<Vec<String>>();
        if args.is_empty() {
            write!(f, "(call {})", self.ident)
        } else {
            write!(f, "(call {} {})", self.ident, args.join(" "))
        }
    }
}