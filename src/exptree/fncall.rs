use std::fmt;
use miette::Error;

use crate::{error::DefaultRuntimeError, evaluator::{Eval, Value}};

use super::ExprTree;

#[derive(Debug, Clone, PartialEq)]
pub struct FnCall<'de> {
    ident: &'de str,
    args: Vec<ExprTree<'de>>
}

impl<'de: 'a, 'a> Eval<'a> for FnCall<'de> {
    fn eval(&self, ctx: &crate::context::CtxTree<'a>) -> Result<Value, Error> {
        let mut v_args = Vec::new();
        for a in &self.args {
            let v = a.eval(ctx)?;
            if matches!(v, Value::Event(_)) {
                return Err(DefaultRuntimeError {}.into()) // TODO change error
            }
            v_args.push(v);
        }
        ctx.exec_fn(self.ident, v_args).unwrap_or(Err(DefaultRuntimeError {}.into())) // TODO change error
    }
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
            write!(f, "(call {} ({}))", self.ident, args.join(" "))
        }
    }
}