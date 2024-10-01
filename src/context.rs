use std::{cell::RefCell, collections::HashMap, rc::Rc};


#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
    Nil,

}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum CtxError {
    VARIABLE_NOT_FOUND
}

#[derive(Clone)]
pub struct CtxTree(Rc<RefCell<Context>>);


struct Context {
    vars: HashMap<String, Value>,
    prev: Option<CtxTree>
}

impl CtxTree {

    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Context { vars: HashMap::new(), prev: None })))
    }

    fn fork(&self) -> Self {
        let ctx: Context = Context {
            vars: HashMap::new(),
            prev: Some(self.clone())
        };
        Self(Rc::new(RefCell::new(ctx)))
    }

    fn search(&self, name: &str) -> Option<Value> {
        let ctx = &self.0;
        if let Some(v) = ctx.borrow().vars.get(name) {
            return Some(v.clone())
        }
        if let Some(ref prev) = ctx.borrow().prev {
            return prev.search(name)
        }
        None
    }

    fn insert(&self, name: &str, value: Value) {
        let mut ctx = self.0.borrow_mut();
        ctx.vars.insert(name.to_string(), value);
    }

    fn set(&self, name: &str, value: Value) -> Result<(), CtxError> {
        if let Some(v) = self.0.borrow().vars.get(name) {
            
            Ok(())
        } else if let Some(ref prev) = self.0.borrow().prev {
            prev.set(name, value)
        } else {
            Err(CtxError::VARIABLE_NOT_FOUND)
        }
    }
}