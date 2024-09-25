
use crate::test_parse;

use super::*;

test_parse!(while_simple,
    "while (i < 10) { print i; i = i + 1; }",
    "(loop nil (< id(i) 10.0) nil (block (print id(i)) (= id(i) (+ id(i) 1.0))))"
);

test_parse!(while_infinite,
    "while (true) { print 1; }",
    "(loop nil true nil (block (print 1.0)))"
);

test_parse!(while_weird,
    "while ({print 1; true}) {}",
    "(loop nil (block (print 1.0) true) nil (block))"
);


test_parse!(while_incorrect_syntax1, ERROR, "while true) {}");
test_parse!(while_incorrect_syntax2, ERROR, "while (true {}");
test_parse!(while_incorrect_syntax3, ERROR, "while (true) print 1;");
