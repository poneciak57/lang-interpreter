use lang_interpreter::parser::Parser;

mod iff;
mod forr;
mod fnn;
mod block;
mod var;
mod arithmetics;

// for testing if something can be parsed as statement without an error
pub fn unwrap_parse_stmt(inp: &str) {
    let mut parser = Parser::new(&inp);
    let _out = parser.parse_statement_within().unwrap().to_string();
}

// for testing if something can be parsed as expression without an error
pub fn unwrap_parse_expr(inp: &str) {
    let mut parser = Parser::new(&inp);
    let _out = parser.parse_expression_within(0).unwrap().to_string();
}


// for testing if something is parsed correctly as statement
pub fn test_parse_stmt(inp: &str, expected_out: &str) {
    let mut parser = Parser::new(&inp);
    let out = parser.parse_statement_within().unwrap().to_string();
    assert_eq!(&out, expected_out);
}

// for testing if something is parsed correctly as expression
pub fn test_parse_expr(inp: &str, expected_out: &str) {
    let mut parser = Parser::new(&inp);
    let out = parser.parse_expression_within(0).unwrap().to_string();
    assert_eq!(&out, expected_out);
}