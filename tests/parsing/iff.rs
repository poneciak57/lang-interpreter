use super::{test_parse, test_parse_expr, unwrap_parse};


#[test]
fn if_expressions() {
    test_parse_expr("if (1 < 3) print 1;", "(if (< 1.0 3.0) (print 1.0))");
    test_parse_expr("if (true) {
        print 1;
        print 2;
        print 3;
    }", "(if true (block (print 1.0) (print 2.0) (print 3.0)))");
}

#[test]
fn if_statement() {
    test_parse("if (1 < 3) print 1;", "(if (< 1.0 3.0) (print 1.0))");
    test_parse("
    if (true) {
        print 1;
        print 2;
        print 3;
    }
    ", "(if true (block (print 1.0) (print 2.0) (print 3.0)))");
}

#[test]
fn if_else_expressions() {
    test_parse_expr("if (1 < 3) print 1; else print 2;", "(if (< 1.0 3.0) (print 1.0) (print 2.0))");
    test_parse_expr("
    if (true) {
        print 1;
        print 2;
    } else {
        print 3;
        print 4;
    }
    ", "(if true (block (print 1.0) (print 2.0)) (block (print 3.0) (print 4.0)))");
}

#[test]
fn if_else_statement() {
    test_parse("if (1 < 3) print 1; else print 2;", "(if (< 1.0 3.0) (print 1.0) (print 2.0))");
    test_parse("
    if (true) {
        print 1;
        print 2;
    } else {
        print 3;
        print 4;
    }
    ", "(if true (block (print 1.0) (print 2.0)) (block (print 3.0) (print 4.0)))");
}

#[test]
#[should_panic]
fn if_incorrect_syntax1() {
    unwrap_parse("if (true)");
}
#[test]
#[should_panic]
fn if_incorrect_syntax2() {
    unwrap_parse("if (true) else");
}
#[test]
#[should_panic]
fn if_incorrect_syntax3() {
    unwrap_parse("if else print 1;");
}
#[test]
#[should_panic]
fn if_incorrect_syntax4() {
    unwrap_parse("if (true) print 1; else");
}
