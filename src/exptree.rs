use std::{borrow::Cow, fmt};

use crate::token::format_num;



#[derive(Debug, Clone, PartialEq)]
pub enum Atom<'de> {
    String(Cow<'de, str>),
    Number(f64),
    Nil,
    Bool(bool),
    Ident(&'de str),
    Continue
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnCall<'de> {
    ident: &'de str,
    args: Vec<ExprTree<'de>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnBlock<'de> {
    ident: &'de str,
    args: Vec<&'de str>,
    block: Box<ExprTree<'de>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConOp<'de> {
    op: Op,
    lhs: Box<ExprTree<'de>>,
    rhs: Box<ExprTree<'de>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOp<'de> {
    op: Op,
    lhs: Box<ExprTree<'de>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct If<'de> {
    cond: Box<ExprTree<'de>>,
    if_block: Box<ExprTree<'de>>,
    else_block: Option<Box<ExprTree<'de>>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Loop<'de> {
    condition: Box<ExprTree<'de>>,
    var: Option<Box<ExprTree<'de>>>,
    step: Option<Box<ExprTree<'de>>>,
    block: Box<ExprTree<'de>>
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprTree<'de> {
    Atom(Atom<'de>),
    ConOp(ConOp<'de>),
    UnaryOp(UnaryOp<'de>),
    FnCall(FnCall<'de>),
    FnBlock(FnBlock<'de>),
    Block(Vec<ExprTree<'de>>, Option<Box<ExprTree<'de>>>),
    If(If<'de>),
    Loop(Loop<'de>)
}

// Unary operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UOp {
    Minus,
    Bang,
    Print,
    Return,
    Break
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Minus,
    Plus,
    Star,
    Slash,
    Group,

    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Equal,
    And,
    Or,

    Var,
}


impl fmt::Display for FnCall<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = self.args.iter().map(|e| e.to_string()).collect::<String>();
        write!(f, "(call {} {})", self.ident, args)
    }
}
impl fmt::Display for FnBlock<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args = self.args.iter().map(|e| e.to_string()).collect::<String>();
        write!(f, "(fn {} {} {})", self.ident, args, self.block)
    }
}
impl fmt::Display for ConOp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.op, self.lhs, self.rhs)
    }
}
impl fmt::Display for UnaryOp<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.op, self.lhs)
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
impl fmt::Display for Loop<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let step = self.step.clone().unwrap_or(Box::new(ExprTree::Atom(Atom::Nil))).to_string();
        let var = self.var.clone().unwrap_or(Box::new(ExprTree::Atom(Atom::Nil))).to_string();
        let condition = self.condition.to_string();
        let block = self.block.to_string();
        write!(f, "(loop {var} {condition} {step} {block})")
    }
}
impl fmt::Display for ExprTree<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprTree::Atom(a) => write!(f, "{a}"),
            ExprTree::ConOp(co) => write!(f, "{co}"),
            ExprTree::UnaryOp(uo) => write!(f, "{uo}"),
            ExprTree::FnCall(fnc) => write!(f, "{fnc}"),
            ExprTree::FnBlock(fnb) => write!(f, "{fnb}"),
            ExprTree::Block(stmts, ret_exp) => {
                let mut statements = stmts.iter().map(|e| e.to_string()).collect::<Vec<String>>();
                if let Some(ref ret) = ret_exp {
                    statements.push(ret.to_string())
                }
                write!(f, "(block {})", statements.join(", "))
            },
            ExprTree::If(i) => write!(f, "{i}"),
            ExprTree::Loop(l) => write!(f, "{l}"),
        }
    }
}
impl fmt::Display for Atom<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::String(s) => write!(f, "{}", s),
            Atom::Number(n) => write!(f, "{}", format_num(*n)),
            Atom::Nil => write!(f, "nil"),
            Atom::Bool(b) => write!(f, "{b}"),
            Atom::Ident(name) => write!(f, "id({name})"),
            Atom::Continue => write!(f, "continue"),
        }
    }
}
impl fmt::Display for UOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Bang => write!(f, "!"),
            Self::Print => write!(f, "print"),
            Self::Return => write!(f, "return"),
            Self::Break => write!(f, "break"),
        }
    }
}
impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Minus => write!(f, "-"),
            Self::Plus => write!(f, "+"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Group => write!(f, "group"),
            Self::BangEqual => write!(f, "!="),
            Self::EqualEqual => write!(f, "=="),
            Self::LessEqual => write!(f, "<="),
            Self::GreaterEqual => write!(f, ">="),
            Self::Less => write!(f, "<"),
            Self::Greater => write!(f, ">"),
            Self::Equal => write!(f, "="),
            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
            Self::Var => write!(f, "var"),
        }
    }
}