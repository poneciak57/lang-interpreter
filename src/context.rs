use std::{cell::RefCell, collections::HashMap, rc::Rc};

use miette::Error;

use crate::{evaluator::Value, exptree::{ExprTree, FnBlock}};


#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum CtxError {
    VARIABLE_NOT_FOUND
}

#[derive(Clone)]
pub struct CtxTree<'de> (Rc<RefCell<Context<'de>>>);


struct Context<'de> {
    vars: HashMap<String, Value>,
    funcs: HashMap<String, FnBlock<'de>>,
    prev: Option<CtxTree<'de>>
}

impl<'de> CtxTree<'de> {

    /// ## Creates new context tree
    /// creates new rooted tree of context returning the root
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Context { vars: HashMap::new(), funcs: HashMap::new(), prev: None })))
    }

    /// ## Forks the tree
    /// forks the tree from current node, node will be dropped 
    /// if uts dropped and all child nodes are dropped
    pub fn fork(&self) -> Self {
        let ctx: Context = Context {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            prev: Some(self.clone())
        };
        Self(Rc::new(RefCell::new(ctx)))
    }


    /// ## Searches for the variable in the context tree
    /// searches for the variable in current node and all the 
    /// parrent nodes up to the root
    pub fn search(&self, name: &str) -> Option<Value> {
        let ctx = &self.0;
        if let Some(v) = ctx.borrow().vars.get(name) {
            return Some(v.clone())
        }
        if let Some(ref prev) = ctx.borrow().prev {
            return prev.search(name)
        }
        None
    }

    // ## Inserts the new function
    // If the function with that name already exists it overides it
    pub fn insert_fn(&self, name: &'de str,  fun: FnBlock<'de>) {
        let mut ctx = self.0.borrow_mut();
        ctx.funcs.insert(name.to_string(), fun);
        *ctx = Context {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            prev: Some(self.clone())
        }; // we update current ctx
    }

    // ## Executes function
    // executes function with given name and arguments
    // returns None if no function with given name exists in current scope
    pub fn exec_fn(&self, name: &str, args: Vec<Value>) -> Option<Result<Value, Error>> {
        let ctx = &self.0;
        if let Some(f) = ctx.borrow().funcs.get(name) {
            return Some(f.exec(self, args));
        }
        if let Some(ref prev) = ctx.borrow().prev {
            return prev.exec_fn(name, args);
        }
        None
    }

    /// ## Inserts the new value
    /// it inserts or owewrites the value in current node
    /// parrent nodes will not be able to search or update this variable
    pub fn insert(&self, name: &str, value: Value) {
        let mut ctx = self.0.borrow_mut();
        ctx.vars.insert(name.to_string(), value);
    }

    /// ## Uptades the value of the variable
    /// updates the value of the variable in current node or returns an 
    /// error if variable does not exists
    pub fn set(&self, name: &str, value: Value) -> Result<(), CtxError> {
        if self.0.borrow().vars.get(name).is_some() {
            self.0.borrow_mut().vars.insert(name.to_string(), value);
            Ok(())
        } else if let Some(ref prev) = self.0.borrow().prev {
            prev.set(name, value)
        } else {
            Err(CtxError::VARIABLE_NOT_FOUND)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functions() {
        let context = CtxTree::new();
        assert_eq!(context.0.borrow().vars.len(), 0);

        context.insert("test", Value::Number(3f64));
        assert_eq!(context.0.borrow().vars.len(), 1);

        let v = context.search("test");
        assert_eq!(v, Some(Value::Number(3f64)));

        context.set("test", Value::String("test".to_string())).unwrap();
        assert_eq!(context.0.borrow().vars.len(), 1);

        let v = context.search("test");
        assert_eq!(v, Some(Value::String("test".to_string())));

        let error = context.set("test1", Value::String("test".to_string()));
        assert_eq!(error, Err(CtxError::VARIABLE_NOT_FOUND));
    }

    #[test]
    fn test_fork_line_basic() {
        let context = CtxTree::new();
        assert_eq!(Rc::strong_count(&context.0), 1);
        let fork1 = context.fork();
        assert_eq!(Rc::strong_count(&context.0), 2);
        assert_eq!(Rc::strong_count(&fork1.0), 1);
        let fork2 = fork1.fork();
        assert_eq!(Rc::strong_count(&context.0), 2);
        assert_eq!(Rc::strong_count(&fork1.0), 2);
        assert_eq!(Rc::strong_count(&fork2.0), 1);
    }

    #[test]
    fn test_fork_line_search() {
        let context = CtxTree::new();
        context.insert("test1", Value::Number(1f64));
        let fork1 = context.fork();
        let fork2 = fork1.fork();

        context.insert("test2", Value::Number(2f64));
        fork2.insert("test3", Value::Number(3f64));
        assert_eq!(fork2.search("test1"), Some(Value::Number(1f64)));
        assert_eq!(fork2.search("test2"), Some(Value::Number(2f64)));
        assert_eq!(fork1.search("test3"), None);
    }

    #[test]
    fn test_fork_line_set() {
        let context = CtxTree::new();
        context.insert("test1", Value::Number(1f64));
        let fork1 = context.fork();
        let fork2 = fork1.fork();
        context.insert("test2", Value::Number(2f64));

        fork2.set("test1", Value::Nil).unwrap();
        assert_eq!(context.search("test1"), Some(Value::Nil));
        assert_eq!(fork1.search("test1"), Some(Value::Nil));
        assert_eq!(fork2.search("test1"), Some(Value::Nil));

        fork1.set("test2", Value::Nil).unwrap();
        assert_eq!(context.search("test2"), Some(Value::Nil));
        assert_eq!(fork1.search("test2"), Some(Value::Nil));
        assert_eq!(fork2.search("test2"), Some(Value::Nil));
    }

    #[test]
    fn test_fork_line_dropping() {
        let context = CtxTree::new();
        let fork1 = context.fork();
        let fork2 = fork1.fork();

        drop(fork1);
        assert_eq!(Rc::strong_count(&context.0), 2);
        drop(fork2);
        assert_eq!(Rc::strong_count(&context.0), 1);
    }

    #[test]
    fn test_fork_wide_basic() {
        let context = CtxTree::new();
        assert_eq!(Rc::strong_count(&context.0), 1);
        let fork1 = context.fork();
        assert_eq!(Rc::strong_count(&context.0), 2);
        assert_eq!(Rc::strong_count(&fork1.0), 1);
        let fork2 = fork1.fork();
        assert_eq!(Rc::strong_count(&fork1.0), 2);
        assert_eq!(Rc::strong_count(&fork2.0), 1);
        let fork3 = context.fork();
        assert_eq!(Rc::strong_count(&context.0), 3);
        assert_eq!(Rc::strong_count(&fork1.0), 2);
        assert_eq!(Rc::strong_count(&fork2.0), 1);
        assert_eq!(Rc::strong_count(&fork3.0), 1);
    }

    #[test]
    fn test_fork_wide_search() {
        let context = CtxTree::new();
        let fork1 = context.fork();
        let fork2 = fork1.fork();
        let fork3 = context.fork();

        context.insert("test1", Value::Number(1f64));
        fork2.insert("test2", Value::Number(2f64));
        fork3.insert("test3", Value::Number(3f64));

        assert_eq!(fork2.search("test1"), Some(Value::Number(1f64)));
        assert_eq!(fork1.search("test1"), Some(Value::Number(1f64)));
        assert_eq!(fork3.search("test1"), Some(Value::Number(1f64)));
        assert_eq!(fork2.search("test2"), Some(Value::Number(2f64)));
        assert_eq!(fork1.search("test2"), None);
        assert_eq!(fork1.search("test3"), None);
        assert_eq!(fork2.search("test3"), None);
        assert_eq!(fork3.search("test2"), None);
        assert_eq!(fork3.search("test3"), Some(Value::Number(3f64)));
    }

    #[test]
    fn test_fork_wide_set() {
        let context = CtxTree::new();
        let fork1 = context.fork();
        let fork2 = fork1.fork();
        let fork3 = context.fork();

        context.insert("test", Value::Number(1f64));

        fork2.set("test", Value::Number(2f64)).unwrap();
        assert_eq!(context.search("test"), Some(Value::Number(2f64)));
        assert_eq!(fork1.search("test"), Some(Value::Number(2f64)));
        assert_eq!(fork2.search("test"), Some(Value::Number(2f64)));
        assert_eq!(fork3.search("test"), Some(Value::Number(2f64)));

        fork3.set("test", Value::Number(3f64)).unwrap();
        assert_eq!(context.search("test"), Some(Value::Number(3f64)));
        assert_eq!(fork1.search("test"), Some(Value::Number(3f64)));
        assert_eq!(fork2.search("test"), Some(Value::Number(3f64)));
        assert_eq!(fork3.search("test"), Some(Value::Number(3f64)));
    }

    #[test]
    fn test_fork_wide_dropping1() {
        let context = CtxTree::new();
        let fork1 = context.fork();
        let fork2 = fork1.fork();
        let fork3: CtxTree = fork1.fork();

        assert_eq!(Rc::strong_count(&fork1.0), 3);
        assert_eq!(Rc::strong_count(&context.0), 2);
        drop(fork2);
        assert_eq!(Rc::strong_count(&fork1.0), 2);
        assert_eq!(Rc::strong_count(&context.0), 2);
        drop(fork3);
        assert_eq!(Rc::strong_count(&fork1.0), 1);
        assert_eq!(Rc::strong_count(&context.0), 2);
    }

    #[test]
    fn test_fork_wide_dropping2() {
        let context = CtxTree::new();
        let fork1 = context.fork();
        let fork2 = fork1.fork();
        let fork3: CtxTree = fork1.fork();

        assert_eq!(Rc::strong_count(&fork1.0), 3);
        assert_eq!(Rc::strong_count(&context.0), 2);
        drop(fork1);
        assert_eq!(Rc::strong_count(&context.0), 2);
        drop(fork2);
        assert_eq!(Rc::strong_count(&context.0), 2);
        drop(fork3);
        assert_eq!(Rc::strong_count(&context.0), 1);
    }
}