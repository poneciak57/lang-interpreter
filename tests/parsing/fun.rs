use crate::test_parse;


use super::*;

test_parse!(fun_unparametrized, STMT,
    "fun test() {}",
    "(fun test () (block))"
);

test_parse!(fun_with_params, STMT,
    "fun test(param1, param2) { print param1; }",
    "(fun test (param1 param2) (block (print id(param1))))"
);

test_parse!(fun_with_a_lot_of_params, STMT, OK,
    "fun test(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10, p11, p12, p13, p14, p15) { }"
);

test_parse!(fun_is_not_an_expression1, EXP, ERROR, "fun test() { }");
test_parse!(fun_is_not_an_expression2, EXP, ERROR, "fun test(p1, p2) { }");

test_parse!(fun_incorrect_syntax1, STMT, ERROR, "fun () { }");
test_parse!(fun_incorrect_syntax2, STMT, ERROR, "fun test ()");
test_parse!(fun_incorrect_syntax3, STMT, ERROR, "fun test () print 1;");
test_parse!(fun_incorrect_syntax4, STMT, ERROR, "fun test( { }");
test_parse!(fun_incorrect_syntax5, STMT, ERROR, "fun test) { }");
test_parse!(fun_incorrect_syntax6, STMT, ERROR, "fun test { }");
test_parse!(fun_incorrect_syntax7, STMT, ERROR, "fun test(p1, ) { }");
test_parse!(fun_incorrect_syntax8, STMT, ERROR, "fun test(p1 { }");
test_parse!(fun_incorrect_syntax9, STMT, ERROR, "fun test p1) { }");
test_parse!(fun_incorrect_syntax10, STMT, ERROR, "fun test p1 { }");
test_parse!(fun_incorrect_syntax11, STMT, ERROR, "fun test(1) { }");
test_parse!(fun_incorrect_syntax12, STMT, ERROR, "fun test(1 + 2) { }");
