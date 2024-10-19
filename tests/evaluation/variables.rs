use lang_interpreter::{context::CtxTree, evaluator::{Eval, Value}, parser::Parser};

use crate::test_expr_eval;

test_expr_eval!(var1, "{ var t; t }", Value::Nil);
test_expr_eval!(var2, "{ var t = 1; t}", Value::Number(1f64));
test_expr_eval!(var3, "{ var t = 1; var t = 2; t}", Value::Number(2f64));
test_expr_eval!(var4, "{ var t = 1; t = 3; t }", Value::Number(3f64));