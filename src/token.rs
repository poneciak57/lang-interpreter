use std::{borrow::Cow, fmt};

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'de> {
    pub kind: TokenKind,
    pub offset: usize,
    pub origin: &'de str,
}
impl<'de> Token<'de> {
    pub fn new(kind: TokenKind, origin: &'de str, offset: usize) -> Self {
        Self { kind, origin, offset } 
    }
}
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    LEFT_SQUARE_BRACKET,
    RIGHT_SQUARE_BRACKET,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    STAR,

    EQUAL,
    BANG_EQUAL,
    EQUAL_EQUAL,
    LESS_EQUAL,
    GREATER_EQUAL,
    LESS,
    GREATER,
    SLASH,
    BANG,

    STRING,
    IDENT,
    NUMBER(f64),

    // keywords
    PRINT,
    AND,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    RETURN,
    TRUE,
    VAR,
    WHILE,
    BREAK,
    CONTINUE,
}
impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = self.kind;
        let i = self.origin;
        match kind {
            TokenKind::LEFT_PAREN => write!(f, "LEFT_PAREN {i} null"),
            TokenKind::RIGHT_PAREN => write!(f, "RIGHT_PAREN {i} null"),
            TokenKind::LEFT_BRACE => write!(f, "LEFT_BRACE {i} null"),
            TokenKind::RIGHT_BRACE => write!(f, "RIGHT_BRACE {i} null"),
            TokenKind::LEFT_SQUARE_BRACKET => write!(f, "LEFT_SQUARE_BRACKET {i} null"),
            TokenKind::RIGHT_SQUARE_BRACKET => write!(f, "RIGHT_SQUARE_BRACKET {i} null"),
            TokenKind::COMMA => write!(f, "COMMA {i} null"),
            TokenKind::DOT => write!(f, "DOT {i} null"),
            TokenKind::MINUS => write!(f, "MINUS {i} null"),
            TokenKind::PLUS => write!(f, "PLUS {i} null"),
            TokenKind::SEMICOLON => write!(f, "SEMICOLON {i} null"),
            TokenKind::STAR => write!(f, "STAR {i} null"),
            TokenKind::EQUAL => write!(f, "EQUAL {i} null"),
            TokenKind::BANG_EQUAL => write!(f, "BANG_EQUAL {i} null"),
            TokenKind::EQUAL_EQUAL => write!(f, "EQUAL_EQUAL {i} null"),
            TokenKind::LESS_EQUAL => write!(f, "LESS_EQUAL {i} null"),
            TokenKind::GREATER_EQUAL => write!(f, "GREATER_EQUAL {i} null"),
            TokenKind::LESS => write!(f, "LESS {i} null"),
            TokenKind::GREATER => write!(f, "GREATER {i} null"),
            TokenKind::SLASH => write!(f, "SLASH {i} null"),
            TokenKind::BANG => write!(f, "BANG {i} null"),
            TokenKind::STRING => write!(f, "STRING \"{}\" {}", unescape(i), unescape(i)),
            TokenKind::IDENT => write!(f, "IDENTIFIER {i} null"),
            TokenKind::NUMBER(n) => write!(f, "NUMBER {i} {}", format_num(n)),
            TokenKind::PRINT => write!(f, "PRINT {i} null"),
            TokenKind::AND => write!(f, "AND {i} null"),
            TokenKind::ELSE => write!(f, "ELSE {i} null"),
            TokenKind::FALSE => write!(f, "FALSE {i} null"),
            TokenKind::FOR => write!(f, "FOR {i} null"),
            TokenKind::FUN => write!(f, "FUN {i} null"),
            TokenKind::IF => write!(f, "IF {i} null"),
            TokenKind::NIL => write!(f, "NIL {i} null"),
            TokenKind::OR => write!(f, "OR {i} null"),
            TokenKind::RETURN => write!(f, "RETURN {i} null"),
            TokenKind::TRUE => write!(f, "TRUE {i} null"),
            TokenKind::VAR => write!(f, "VAR {i} null"),
            TokenKind::WHILE => write!(f, "WHILE {i} null"),
            TokenKind::BREAK => write!(f, "BREAK {i} null"),
            TokenKind::CONTINUE => write!(f, "CONTINUE {i} null"),
        }
    }
}

pub fn format_num(n: f64) -> String {
    if n.fract() == 0.0 { format!("{n:.1}") } else { format!("{n}") }
}

pub fn unescape<'de>(s: &'de str) -> Cow<'de, str> {
    // TODO change this latter with better escapings
    let mut chars = s.chars();
    let mut ret = String::new();
    enum State {
        Starting,
        Normal,
        Escaping,
    }
    let mut state = State::Starting;
    while let Some(c) = chars.next() {
        match c {
            '\\' => match state {
                State::Escaping => {
                    state = State::Normal;
                    ret.push('\\');
                },
                _ => state = State::Escaping
            },
            '"' => match state {
                State::Starting => state = State::Normal,
                State::Normal => {},
                State::Escaping => {
                    state = State::Normal;
                    ret.push('"');
                },
            },
            _ => match state {
                State::Escaping => {
                    state = State::Normal;
                    let special = match c {
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        '\'' => '\'',
                        _ => c
                    };
                    ret.push(special);
                },
                _ => ret.push(c)
            }
        }
    }
    Cow::Owned(ret)
}