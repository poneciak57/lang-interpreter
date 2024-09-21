use lang_interpreter::parser::Parser;

mod iff;
mod forr;
mod fnn;
mod block;
mod var;
mod arithmetics;

// for testing if something can be parsed or throws an error
pub fn unwrap_parse(inp: &str) {
    let parser = Parser::new(&inp);
    let _out = parser.parser().unwrap().to_string();
}


// for testing if something is parsed correctly
pub fn test_parse(inp: &str, expected_out: &str) {
    let parser = Parser::new(&inp);
    let out = parser.parser().unwrap().to_string();
    assert_eq!(&out, expected_out);
}

pub fn test_parse_expr(inp: &str, expected_out: &str) {
    let mut parser = Parser::new(&inp);
    let out = parser.parse_expression_within(0).unwrap().to_string();
    assert_eq!(&out, expected_out);
}