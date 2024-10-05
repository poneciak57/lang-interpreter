use std::{borrow::Cow, fmt};

use crate::{evaluator::{Event, Value}, error::DefaultRuntimeError, evaluator::Eval, token::format_num};

pub mod conop;
pub mod fnblock;
pub mod fncall;
pub mod if_struct;
pub mod loop_struct;
pub mod uop;
pub mod vardecl;

pub use conop::*;
pub use fnblock::*;
pub use fncall::*;
pub use if_struct::*;
pub use loop_struct::*;
use miette::Error;
pub use uop::*;
pub use vardecl::*;



#[derive(Debug, Clone, PartialEq)]
pub enum Atom<'de> {
    String(Cow<'de, str>),
    Number(f64),
    Nil,
    Bool(bool),
    Ident(&'de str),
    Continue
}

impl<'de: 'a, 'a> Eval<'a> for Atom<'de> {
    fn eval(&self, ctx: &crate::context::CtxTree<'a>) -> Result<Value, Error>  {
        match &self {
            Atom::String(s) => Ok(Value::String(s.to_string())),
            Atom::Number(n) => Ok(Value::Number(n.clone())),
            Atom::Nil => Ok(Value::Nil),
            Atom::Bool(b) => Ok(Value::Bool(b.clone())),
            Atom::Ident(id) => Ok(ctx.search(id).ok_or(DefaultRuntimeError{})?), // TODO switch error
            Atom::Continue => Ok(Value::Event(Event::Continue)),
        }
    }
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
    Loop(Loop<'de>),
    Var(VarDecl<'de>)
}

impl<'de: 'a, 'a> Eval<'a> for ExprTree<'de> {
    fn eval(&self, ctx: &crate::context::CtxTree<'a>) -> Result<Value, Error> {
        match self {
            ExprTree::Atom(a) => a.eval(ctx),
            ExprTree::ConOp(cop) => cop.eval(ctx),
            ExprTree::UnaryOp(uop) => uop.eval(ctx),
            ExprTree::FnCall(fnc) => fnc.eval(ctx),
            ExprTree::FnBlock(fnb) => fnb.eval(ctx),
            ExprTree::If(iff) => iff.eval(ctx),
            ExprTree::Loop(lop) => lop.eval(ctx),
            ExprTree::Var(vd) => vd.eval(ctx),
            ExprTree::Block(stmts, retexp) => {
                let fork = ctx.fork();
                for s in stmts {
                    let v = s.eval(&fork)?;
                    if !matches!(v, Value::Event(Event::NoVal)) {
                        if let Value::Event(e) = v {
                            return Ok(Value::Event(e));
                        }
                    }
                }
                if let Some(ref retexp) = retexp {
                    let v = retexp.eval(ctx)?;
                    if matches!(v, Value::Event(_)) {
                        return Err(DefaultRuntimeError {}.into()); // TODO change error
                    }
                    return Ok(v)
                }
                return Ok(Value::Nil);
            }
        }
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
                if statements.is_empty() {
                    write!(f, "(block)")
                } else {
                    write!(f, "(block {})", statements.join(" "))
                }
            },
            ExprTree::If(i) => write!(f, "{i}"),
            ExprTree::Loop(l) => write!(f, "{l}"),
            ExprTree::Var(v) => write!(f, "{v}"),
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
