use lang_interpreter::lexer::Lexer;


#[test]
fn one_letter_signs_including_whitespaces() {
    test_lex(" ( ", "LEFT_PAREN ( null\n");
    test_lex(" ! = ", "BANG_EQUAL ! = null\n");
    test_lex(" = ==", "EQUAL_EQUAL = = null\nEQUAL = null\n");
}

#[test]
fn one_letter_signs() {
    test_lex("(", "LEFT_PAREN ( null\n");
    test_lex(")", "RIGHT_PAREN ) null\n");
    test_lex("{", "LEFT_BRACE { null\n");
    test_lex("}", "RIGHT_BRACE } null\n");
    test_lex("[", "LEFT_SQUARE_BRACKET [ null\n");
    test_lex("]", "RIGHT_SQUARE_BRACKET ] null\n");
    test_lex(",", "COMMA , null\n");
    test_lex(".", "DOT . null\n");
    test_lex("-", "MINUS - null\n");
    test_lex("+", "PLUS + null\n");
    test_lex(";", "SEMICOLON ; null\n");
    test_lex("*", "STAR * null\n");
    test_lex("=", "EQUAL = null\n");
    test_lex("<", "LESS < null\n");
    test_lex(">", "GREATER > null\n");
    test_lex("/", "SLASH / null\n");
    test_lex("!", "BANG ! null\n");
}

#[test]
fn two_letter_signs() {
    test_lex("!=", "BANG_EQUAL != null\n");
    test_lex("==", "EQUAL_EQUAL == null\n");
    test_lex("<=", "LESS_EQUAL <= null\n");
    test_lex(">=", "GREATER_EQUAL >= null\n");
}

#[test]
fn special_keywords() {
    test_lex("print", "PRINT print null\n");
    test_lex("and", "AND and null\n");
    test_lex("else", "ELSE else null\n");
    test_lex("false", "FALSE false null\n");
    test_lex("for", "FOR for null\n");
    test_lex("fun", "FUN fun null\n");
    test_lex("if", "IF if null\n");
    test_lex("nil", "NIL nil null\n");
    test_lex("or", "OR or null\n");
    test_lex("return", "RETURN return null\n");
    test_lex("true", "TRUE true null\n");
    test_lex("var", "VAR var null\n");
    test_lex("while", "WHILE while null\n");
}

#[test]
fn strings() {
    test_lex(r#""test""#, "STRING \"test\" test\n");
    test_lex(r#""\"""#, "STRING \"\"\" \"\n");
    test_lex(r#""\n""#, "STRING \"\n\" \n\n");
    test_lex(r#""\t""#, "STRING \"\t\" \t\n");
}

#[test]
fn numbers() {
    test_lex("3", "NUMBER 3 3.0\n");
    test_lex("13.21", "NUMBER 13.21 13.21\n");
    test_lex("3.4 + 3", "NUMBER 3.4 3.4\nPLUS + null\nNUMBER 3 3.0\n");
}

#[test]
fn idents() {
    test_lex("test", "IDENTIFIER test null\n");
    test_lex("te1st123", "IDENTIFIER te1st123 null\n");
    test_lex("fun test()", r#"
FUN fun null
IDENTIFIER test null
LEFT_PAREN ( null
RIGHT_PAREN ) null
"#.trim_ascii_start());
}

#[test]
fn full_code1() {
    test_lex(r#"
while (a < 3) {
    print "test";
}
"#, r#"
WHILE while null
LEFT_PAREN ( null
IDENTIFIER a null
LESS < null
NUMBER 3 3.0
RIGHT_PAREN ) null
LEFT_BRACE { null
PRINT print null
STRING "test" test
SEMICOLON ; null
RIGHT_BRACE } null
"#.trim_ascii_start());
}

#[test]
fn full_code2() {
    test_lex(r#"
var test = 3;
test = test + 1.1;
"#, r#"
VAR var null
IDENTIFIER test null
EQUAL = null
NUMBER 3 3.0
SEMICOLON ; null
IDENTIFIER test null
EQUAL = null
IDENTIFIER test null
PLUS + null
NUMBER 1.1 1.1
SEMICOLON ; null
"#.trim_ascii_start());
}

pub fn test_lex(inp: &str, expected_out: &str) {
    let lexer = Lexer::new(&inp);
    let mut out = String::new();
    for n in lexer.into_iter() {
        out.push_str(&format!("{}\n", &n.unwrap().to_string()))
    }
    assert_eq!(&out, expected_out);
}