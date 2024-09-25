use crate::test_parse;

use super::*;

test_parse!(if1,
    "if (1 < 3) { print 1; }", 
    "(if (< 1.0 3.0) (block (print 1.0)))");

test_parse!(if2,
    "if (true) {
        print 1;
        print 2;
        print 3;
    }", 
    "(if true (block (print 1.0) (print 2.0) (print 3.0)))");

test_parse!(if_else1,
    "if (1 < 3) { print 1; } else { print 2; }",
    "(if (< 1.0 3.0) (block (print 1.0)) (block (print 2.0)))");

test_parse!(if_else2,
    "
    if (true) {
        print 1;
        print 2;
    } else {
        print 3;
        print 4;
    }
    ",
    "(if true (block (print 1.0) (print 2.0)) (block (print 3.0) (print 4.0)))");

test_parse!(if_incorrect_syntax1, ERROR, "if (true)");
test_parse!(if_incorrect_syntax2, ERROR, "if (true) else");
test_parse!(if_incorrect_syntax3, ERROR, "if else print 1;");
test_parse!(if_incorrect_syntax4, ERROR, "if (true) print 1; else");
test_parse!(if_incorrect_syntax5, ERROR, "if (true) { print 1; } else print 1;");
test_parse!(if_incorrect_syntax6, ERROR, "if (true) print 1;");