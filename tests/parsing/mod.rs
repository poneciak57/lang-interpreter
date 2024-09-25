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

#[macro_export]
macro_rules! test_parse {
    ($name:ident, EXP, $inp:literal, $out:literal) => {
        #[test]
        fn $name() {
            test_parse_expr($inp, $out);
        }
    };
    ($name:ident, STMT, $inp:literal, $out:literal) => {
        #[test]
        fn $name() {
            test_parse_stmt($inp, $out);
        }
    };
    ($name:ident, $inp:literal, $out:literal) => {
        mod $name {
            use super::*;

            test_parse!(statement, STMT, $inp, $out);
            test_parse!(expression, EXP, $inp, $out);
        }
    };
    ($name:ident, ERROR, $inp:literal) => {
        mod $name {
            use super::*;
            
            test_parse!(statement, STMT, ERROR, $inp);
            test_parse!(expression, EXP, ERROR, $inp);
        }
    };
    ($name:ident, OK, $inp:literal) => {
        mod $name {
            use super::*;
            
            test_parse!(statement, STMT, OK, $inp);
            test_parse!(expression, EXP, OK, $inp);
        }
    };


    ($name:ident, EXP, OK, $inp:literal) => {
        #[test]
        fn $name() {
            unwrap_parse_expr($inp);
        }
    };
    ($name:ident, EXP, ERROR, $inp:literal) => {
        #[test]
        #[should_panic]
        fn $name() {
            unwrap_parse_expr($inp);
        }
    };
    ($name:ident, STMT, OK, $inp:literal) => {
        #[test]
        fn $name() {
            unwrap_parse_stmt($inp);
        }
    };
    ($name:ident, STMT, ERROR, $inp:literal) => {
        #[test]
        #[should_panic]
        fn $name() {
            unwrap_parse_stmt($inp);
        }
    };
}