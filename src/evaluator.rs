use std::{fmt, ops::{Add, Neg, Not, Sub}};

use miette::Error;

use crate::{context::CtxTree, error::DefaultRuntimeError};


#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,

    Event(Event),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Continue,
    Break(Box<Value>),
    Return(Box<Value>),
    NoVal, // more than nil, only for stmts that cant return
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Event::Continue => write!(f, "continue"),
            Event::Break(bv) => write!(f, "break {}", bv),
            Event::Return(rv) => write!(f, "return {}", rv),
            _ => panic!("can't print the statement")
        }
    }
}

pub trait Eval<'a> {
    fn eval(&self, ctx: &CtxTree<'a>) -> Result<Value, Error>;
}


impl Add for Value {
    type Output = Result<Value, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::String(s1), Value::String(s2)) => Ok(Value::String(format!("{s1}{s2}"))),
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
            (Value::Number(n1), Value::Bool(b)) => Ok(Value::Number(n1 + ((b as usize) as f64))),
            (Value::Bool(b), Value::Number(n2)) => Ok(Value::Number(((b as usize) as f64) + n2)),
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Number((b1 as usize + b2 as usize) as f64)),
            _ => Err(DefaultRuntimeError {}.into()) // TODO change error
        }
    }
}

impl Sub for Value {
    type Output = Result<Value, Error>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
            (Value::Number(n1), Value::Bool(b)) => Ok(Value::Number(n1 - ((b as usize) as f64))),
            (Value::Bool(b), Value::Number(n2)) => Ok(Value::Number(((b as usize) as f64) - n2)),
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Number((b1 as usize - b2 as usize) as f64)),
            _ => Err(DefaultRuntimeError {}.into()) // TODO change error
        }
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        match value {
            Value::String(_) => true,
            Value::Number(n) => n != 0f64,
            Value::Bool(b) => b,
            Value::Nil => false,
            Value::Event(_) => false,
        }
    }
}

impl Neg for Value {
    type Output = Result<Value, Error>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(DefaultRuntimeError {}.into()), // TODO change error
        }
    }
}

impl Not for Value {
    type Output = Result<Value, Error>;

    fn not(self) -> Self::Output {
        match self {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            Value::Number(n) => Ok(Value::Bool(n == 0f64)),
            Value::Nil => Ok(Value::Bool(true)),
            _ => Err(DefaultRuntimeError {}.into()), // TODO change error
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Event(e) => write!(f, "{}", e),
        }
    }
}