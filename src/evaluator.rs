use miette::Error;

use crate::context::{CtxTree, Value};


pub trait Eval {
    fn eval(&self, ctx: &CtxTree) -> Result<Value, Error>;
}