use lang_interpreter::{context::CtxTree, evaluator::{Eval, Value}, parser::Parser};

use crate::test_expr_eval;

pub mod math;

// #[test]
// fn t1() {
//     let expr = "1 + 2 * 3";
//     let mut parser = Parser::new(expr);
//     let ctx = CtxTree::new();
//     let exp_tree = parser.parse_expression_within(0).unwrap();
//     println!("tree: {}", exp_tree);
//     let val = exp_tree.eval(&ctx).unwrap();

//     println!("ret: {}", val);
// }

test_expr_eval!(basic_test1, "1 + 2 * 3", Value::Number(7f64));

#[macro_export]
macro_rules! test_expr_eval {
    ($name:ident, $inp:literal, $out:expr) => {
        #[test]
        fn $name() {
            let expr = $inp;
            let mut parser = Parser::new(expr);
            let ctx = CtxTree::new();
            let exp_tree = parser.parse_expression_within(0).unwrap();
            let val = exp_tree.eval(&ctx).unwrap();
            assert_eq!(val, $out);
        }
    }
}