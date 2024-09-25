
use crate::test_parse;

use super::*;

test_parse!(var1, STMT, "var test = 1;", "(var id(test) 1.0)");
test_parse!(var2, STMT, "var test = 1 + 2;", "(var id(test) (+ 1.0 2.0))");
test_parse!(var3, STMT, "var test;", "(var id(test) nil)");


test_parse!(var_is_not_an_expression1, EXP, ERROR, "var test = 1;");
test_parse!(var_is_not_an_expression2, EXP, ERROR, "var test;");


test_parse!(var_incorrect_syntax1, STMT, ERROR, "var test 1;");
test_parse!(var_incorrect_syntax2, STMT, ERROR, "var = 1;");
test_parse!(var_incorrect_syntax3, STMT, ERROR, "var;");
