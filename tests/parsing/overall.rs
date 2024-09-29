use lang_interpreter::parser::Parser;


#[test]
fn test_parser_overally1() {

    let code = "
    var sum = 0;
    for (var i = 0; i <= 10; i = i + 1) {
        sum = sum + i;
    }
    print sum;
    
    ";
    let parser = Parser::new(code);
    let parser_out = parser.parse().unwrap();

    assert_eq!(parser_out.len(), 3);
    assert_eq!(parser_out[0].to_string(), "(var id(sum) 0.0)");
    assert_eq!(parser_out[1].to_string(), "(loop (var id(i) 0.0) (<= id(i) 10.0) (= id(i) (+ id(i) 1.0)) (block (= id(sum) (+ id(sum) id(i)))))");
    assert_eq!(parser_out[2].to_string(), "(print id(sum))");
}

#[test]
fn test_parser_overally2() {

    let code = "
    // adds to arguments
    fun add_args(arg1, arg2) {
        return arg1 + arg2;
    }

    print add_args(1, 2);
    
    ";
    let parser = Parser::new(code);
    let parser_out = parser.parse().unwrap();

    assert_eq!(parser_out.len(), 2);
    assert_eq!(parser_out[0].to_string(), "(fun add_args (arg1 arg2) (block (return (+ id(arg1) id(arg2)))))");
    assert_eq!(parser_out[1].to_string(), "(print (call add_args (1.0 2.0)))");
}