use lang_interpreter::parser::Parser;

mod iff;
mod forr;
mod fnn;
mod block;
mod var;
mod arithmetics;

// for testing if something can be parsed without an error
pub fn unwrap_parse(inp: &str) {
    let parser = Parser::new(&inp);
    let _out = parser.parse().unwrap().to_string();
}


// for testing if something is parsed correctly
pub fn test_parse(inp: &str, expected_out: &str) {
    let parser = Parser::new(&inp);
    let out = parser.parse().unwrap().to_string();
    assert_eq!(&out, expected_out);
}

// for testing if something is parsed correctly as expression
pub fn test_parse_expr(inp: &str, expected_out: &str) {
    let mut parser = Parser::new(&inp);
    let out = parser.parse_expression_within(0).unwrap().to_string();
    assert_eq!(&out, expected_out);
}