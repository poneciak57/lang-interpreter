use lang_interpreter::{context::CtxTree, evaluator::{Eval, Value}, parser::Parser};

use crate::test_expr_eval;



test_expr_eval!(addition, "1 + 2 + 3", Value::Number(6f64));
test_expr_eval!(substraction, "11 - 3", Value::Number(8f64));
test_expr_eval!(multiplication, "5 * 3", Value::Number(15f64));
test_expr_eval!(division, "12 / 4", Value::Number(3f64));

test_expr_eval!(mixed1, "3 * 2 + 3", Value::Number(9f64));
test_expr_eval!(mixed2, "3 * 4 / 2", Value::Number(6f64));
test_expr_eval!(mixed3, "5 - 2 * 2", Value::Number(1f64));

test_expr_eval!(group1, "3 * (2 + 3)", Value::Number(15f64));
test_expr_eval!(group2, "(5 - 2) * 2", Value::Number(6f64));

test_expr_eval!(unaryop1, "-(5 - 2) * 2", Value::Number(-6f64));
test_expr_eval!(unaryop2, "!(5 - 2 == 3)", Value::Bool(false));

test_expr_eval!(equalequal1, "5 - 2 == 3", Value::Bool(true));
test_expr_eval!(equalequal2, "5 - 2 == 3 * 2", Value::Bool(false));
test_expr_eval!(notequal1, "5 - 2 != 3", Value::Bool(false));
test_expr_eval!(notequal2, "5 - 2 != 3 * 2", Value::Bool(true));
test_expr_eval!(less1, "3 < 10", Value::Bool(true));
test_expr_eval!(less2, "3 < 3", Value::Bool(false));
test_expr_eval!(lessequal1, "3 <= 3", Value::Bool(true));
test_expr_eval!(lessequal2, "3 <= 2", Value::Bool(false));
test_expr_eval!(greater1, "4 > 3", Value::Bool(true));
test_expr_eval!(greater2, "3 > 3", Value::Bool(false));
test_expr_eval!(greaterequal1, "4 >= 3", Value::Bool(true));
test_expr_eval!(greaterequal2, "3 >= 3", Value::Bool(true));
test_expr_eval!(greaterequal3, "2 >= 3", Value::Bool(false));

test_expr_eval!(and1, "1 == 1 and 2 == 2", Value::Bool(true));
test_expr_eval!(and2, "1 == 2 and 2 == 2", Value::Bool(false));
test_expr_eval!(and3, "1 == 1 and 2 == 1", Value::Bool(false));
test_expr_eval!(and4, "1 == 2 and 1 == 2", Value::Bool(false));

test_expr_eval!(or1, "1 == 1 or 2 == 2", Value::Bool(true));
test_expr_eval!(or2, "1 == 2 or 2 == 2", Value::Bool(true));
test_expr_eval!(or3, "1 == 1 or 2 == 1", Value::Bool(true));
test_expr_eval!(or4, "1 == 2 or 1 == 2", Value::Bool(false));