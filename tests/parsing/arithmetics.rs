use super::test_parse_expr;


#[test]
fn math_and_binding_power() {
    test_parse_expr("1 + 2 + 3", "(+ (+ 1.0 2.0) 3.0)");
    test_parse_expr("1 + 2 - 3", "(- (+ 1.0 2.0) 3.0)");
    test_parse_expr("1 + 2 * 3", "(+ 1.0 (* 2.0 3.0))");
    test_parse_expr("1 - 2 / 3", "(- 1.0 (/ 2.0 3.0))");
    test_parse_expr("(1 + 2) * 3", "(* (group (+ 1.0 2.0)) 3.0)");
}

#[test]
fn condiotions() {
    test_parse_expr("1 < 2 and 2 < 3", "(&& (< 1.0 2.0) (< 2.0 3.0))");
    test_parse_expr("1 < 2 or 2 < 3", "(|| (< 1.0 2.0) (< 2.0 3.0))");
    test_parse_expr("1 != 2 or 2 > 3", "(|| (!= 1.0 2.0) (> 2.0 3.0))");
    test_parse_expr("(1 > 2) == (2 > 3)", "(== (group (> 1.0 2.0)) (group (> 2.0 3.0)))");
    test_parse_expr("1 + 2 == 4 - 1", "(== (+ 1.0 2.0) (- 4.0 1.0))");
    test_parse_expr("1 * 3 != 3 * 1", "(!= (* 1.0 3.0) (* 3.0 1.0))");
}

#[test]
fn unary_ops() {
    test_parse_expr("-1 + -2 + -3", "(+ (+ (- 1.0) (- 2.0)) (- 3.0))");
    test_parse_expr("1 + -2 * 3", "(+ 1.0 (* (- 2.0) 3.0))");
    test_parse_expr("1 + -(2 * 3)", "(+ 1.0 (- (group (* 2.0 3.0))))");
    test_parse_expr("!true == false", "(== (! true) false)");
    test_parse_expr("!(1 < 2)", "(! (group (< 1.0 2.0)))");
    test_parse_expr("!(1 < 2) == true", "(== (! (group (< 1.0 2.0))) true)");
}