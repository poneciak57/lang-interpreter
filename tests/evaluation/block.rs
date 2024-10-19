use lang_interpreter::evaluator::Value;

use crate::test_expr_eval;
use super::*;


test_expr_eval!(block_without_return_value_1, "{}" , Value::Nil);
test_expr_eval!(block_without_return_value_2, "{ for(var i = 0; i < 10; i = i + 1) {}; }" , Value::Nil);

test_expr_eval!(block_returning_1, "{ 3 }" , Value::Number(3f64));
test_expr_eval!(block_returning_2, "{ if(true) { 2 } else { 1 } }" , Value::Number(2f64));
test_expr_eval!(block_returning_3, "{ { 2 } }" , Value::Number(2f64));
test_expr_eval!(block_returning_4, "{ var a = 0; a = a + 1; a }" , Value::Number(1f64));
