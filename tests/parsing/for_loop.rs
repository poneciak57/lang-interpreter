use crate::test_parse;
use super::*;


test_parse!(for_simple,
    "for (var i = 1; i < 10; i = i + 1) { print i; }",
    "(loop (var id(i) 1.0) (< id(i) 10.0) (= id(i) (+ id(i) 1.0)) (block (print id(i))))"
);

test_parse!(for_weird,
    "for ({ print 1; print 2;}; print 3; print 4) { print 5; }",
    "(loop (block (print 1.0) (print 2.0)) (print 3.0) (print 4.0) (block (print 5.0)))"
);

test_parse!(for_infinite, 
    "for (;;) { print 1; }",
    "(loop nil true nil (block (print 1.0)))"
);

test_parse!(for_incomplete_params1, OK, "for(var i = 1; ; i = i + 1) { print i; }");
test_parse!(for_incomplete_params2, OK, "for(var i = 1; i < 10;) { print i; }");
test_parse!(for_incomplete_params3, OK, "for(; i < 10; i = i + 1) { print i; }");

test_parse!(for_invalid_syntax1, ERROR, "for(;) { print 1; }");
test_parse!(for_invalid_syntax2, ERROR, "for ;;) { print 1; }");
test_parse!(for_invalid_syntax3, ERROR, "for(;; { print 1; }");
test_parse!(for_invalid_syntax4, ERROR, "for(;;) print 1;");