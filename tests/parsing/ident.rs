use crate::test_parse;

use super::*;


test_parse!(ident_variable_name, 
    "test + 1 + test1 + test2",
    "(+ (+ (+ id(test) 1.0) id(test1)) id(test2))"
);

test_parse!(ident_fn_call, 
    "test() + 1 + test1 + test2(arg1, 2)",
    "(+ (+ (+ (call test) 1.0) id(test1)) (call test2 (id(arg1) 2.0)))"
);

test_parse!(ident_fn_call_incorrect_syntax1, ERROR, "test(");