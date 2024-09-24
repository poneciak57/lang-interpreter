use super::{test_parse_stmt, test_parse_expr, unwrap_parse_stmt};


#[test]
fn if_expressions() {
    test_parse_expr("if (1 < 3) { print 1; }", "(if (< 1.0 3.0) (block (print 1.0)))");
    test_parse_expr("if (true) {
        print 1;
        print 2;
        print 3;
    }", "(if true (block (print 1.0) (print 2.0) (print 3.0)))");
}

#[test]
fn if_statement() {
    test_parse_stmt("if (1 < 3) { print 1; }", "(if (< 1.0 3.0) (block (print 1.0)))");
    test_parse_stmt("
    if (true) {
        print 1;
        print 2;
        print 3;
    }
    ", "(if true (block (print 1.0) (print 2.0) (print 3.0)))");
}

#[test]
fn if_else_expressions() {
    test_parse_expr("if (1 < 3) { print 1; } else { print 2; }", "(if (< 1.0 3.0) (block (print 1.0)) (block (print 2.0)))");
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
    test_parse_stmt("if (1 < 3) { print 1; } else { print 2; }", "(if (< 1.0 3.0) (block (print 1.0)) (block (print 2.0)))");
    test_parse_stmt("
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
    unwrap_parse_stmt("if (true)");
}
#[test]
#[should_panic]
fn if_incorrect_syntax2() {
    unwrap_parse_stmt("if (true) else");
}
#[test]
#[should_panic]
fn if_incorrect_syntax3() {
    unwrap_parse_stmt("if else print 1;");
}
#[test]
#[should_panic]
fn if_incorrect_syntax4() {
    unwrap_parse_stmt("if (true) print 1; else");
}

#[test]
#[should_panic]
fn if_incorrect_syntax5() {
    unwrap_parse_stmt("if (true) print 1;");
}

#[test]
#[should_panic]
fn if_incorrect_syntax6() {
    unwrap_parse_stmt("if (true) { print 1; } else print 1;");
}