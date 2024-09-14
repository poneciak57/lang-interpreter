use lang_interpreter::parser::Parser;


pub fn test_parse(inp: &str, expected_out: &str) {
    let parser = Parser::new(&inp);
    let out = parser.parser().unwrap().to_string();
    assert_eq!(&out, expected_out);
}