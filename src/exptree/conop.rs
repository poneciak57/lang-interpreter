use std::fmt;

use super::ExprTree;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Minus,
    Plus,
    Star,
    Slash,

    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Equal,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConOp<'de> {
    op: Op,
    lhs: Box<ExprTree<'de>>,
    rhs: Box<ExprTree<'de>>
}

impl<'de> ConOp<'de> {
    pub fn new(op: Op, lhs: Box<ExprTree<'de>>, rhs: Box<ExprTree<'de>>) -> Self {
        Self { op, lhs, rhs }
    }
}

impl fmt::Display for ConOp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.op, self.lhs, self.rhs)
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Plus => write!(f, "+"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::BangEqual => write!(f, "!="),
            Self::EqualEqual => write!(f, "=="),
            Self::LessEqual => write!(f, "<="),
            Self::GreaterEqual => write!(f, ">="),
            Self::Less => write!(f, "<"),
            Self::Greater => write!(f, ">"),
            Self::Equal => write!(f, "="),
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
        }
    }
}