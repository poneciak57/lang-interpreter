use super::test_parse_expr;


#[test]
fn if_expressions() {
    test_parse_expr("if (1 < 3) print 1;", "(if (< 1 3) (print 1))");
    test_parse_expr("if (true) {
        print 1;
        print 2;
        print 3;
    }", "(if true (block (print 1) (print 2) (print 3)))");
    todo!()
}

// todo else and other tests