use miette::Error;

use crate::{exptree::ExprTree, lexer::Lexer};

pub struct Parser<'de> {
    whole: &'de str,
    lexer: Lexer<'de>
}

impl<'de> Parser<'de> {
    pub fn new(input: &'de str) -> Self {
        Self { whole: input, lexer: Lexer::new(input) }
    }

    pub fn parser(self) -> Result<ExprTree<'de>, Error> {
        todo!()
    }

    pub fn parse_expression_within(&mut self, min_bp: u8) -> Result<ExprTree<'de>, Error> {
        // TODO pratt parsing expression
        todo!()
    }
}