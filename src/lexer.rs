use std::mem;

use miette::{Error, LabeledSpan, SourceSpan};

use crate::{error::{Eof, SingleTokenError, UnterminatedStringError}, token::{Token, TokenKind}};

pub struct Lexer<'de> {
    whole: &'de str,
    rest: &'de str,
    byte: usize,
    peeked: Option<Result<Token<'de>, Error>>
}

impl<'de> Lexer<'de> {
    pub fn new(input: &'de str) -> Self {
        Self { 
            whole: input,
            rest: input,
            byte: 0,
            peeked: None
        }
    }
    pub fn peek(&mut self) -> Option<&Result<Token<'de>, Error>> {
        if self.peeked.is_some() {
            return self.peeked.as_ref()
        }
        self.peeked = self.next();
        self.peeked.as_ref()
    }
    pub fn expect_next(&mut self, next: TokenKind, unexpected: &str) -> Result<Token<'de>, miette::Error> {
        match self.next() {
            Some(Ok(token)) if token.kind == next => Ok(token),
            Some(Ok(token)) => {
                return Err(miette::miette! {
                    labels = vec![
                    LabeledSpan::at(token.offset..token.offset + token.origin.len(), "here"),
                    ],
                    help = format!("Expected {next:?}"),
                    "{unexpected}"
                }.with_source_code(self.whole.to_string()))
            },
            Some(Err(e)) => return Err(e),
            None => return Err(Eof.into())
        }
    }
    pub fn next_or_error(&mut self) -> Result<Token<'de>, miette::Error> {
        match self.next() {
            Some(Ok(token)) => Ok(token),
            None => Err(Eof.into()),
            Some(Err(e)) => Err(e)
        }
    }
}

impl<'de> Iterator for Lexer<'de> 
{
    type Item = Result<Token<'de>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(_) = self.peeked {
            return mem::replace(&mut self.peeked, None);
        }
        loop {
            let mut chars = self.rest.chars();
            let c = chars.next()?;
            let c_at = self.byte;
            let c_str = &self.rest[..c.len_utf8()];
            let c_onwards = self.rest;
            self.rest = chars.as_str();
            self.byte += c.len_utf8();

            enum Started {
                String,
                Number,
                Ident,
                IfEqualElse(TokenKind, TokenKind),
                Comment
            }

            let just = move |kind: TokenKind| {
                Some(Ok(Token::new(kind, c_str, c_at)))
            };

            let started = match c {
                '(' => return just(TokenKind::LEFT_PAREN),
                ')' => return just(TokenKind::RIGHT_PAREN),
                '{' => return just(TokenKind::LEFT_BRACE),
                '}' => return just(TokenKind::RIGHT_BRACE),
                '[' => return just(TokenKind::LEFT_SQUARE_BRACKET),
                ']' => return just(TokenKind::RIGHT_SQUARE_BRACKET),
                ',' => return just(TokenKind::COMMA),
                '.' => return just(TokenKind::DOT),
                '-' => return just(TokenKind::MINUS),
                '+' => return just(TokenKind::PLUS),
                ';' => return just(TokenKind::SEMICOLON), 
                '*' => return just(TokenKind::STAR),

                '/' => Started::Comment,
                '<' => Started::IfEqualElse(TokenKind::LESS_EQUAL, TokenKind::LESS),
                '>' => Started::IfEqualElse(TokenKind::GREATER_EQUAL, TokenKind::GREATER),
                '=' => Started::IfEqualElse(TokenKind::EQUAL_EQUAL, TokenKind::EQUAL),
                '!' => Started::IfEqualElse(TokenKind::BANG_EQUAL, TokenKind::BANG),
                '"' => Started::String,
                '0'..='9' => Started::Number,
                'a'..='z' | 'A'..='Z' | '_' => Started::Ident,

                c if c.is_whitespace() => continue,
                _ => return Some(Err(SingleTokenError {
                    src: self.whole.to_string(),
                    token: c,
                    err_span: SourceSpan::from(self.byte - c.len_utf8()..self.byte),
                }.into()))
            };


            match started {
                Started::String => {
                    // TODO maybe delete escaping its unnecessery
                    #[derive(PartialEq, Debug)]
                    enum State {
                        Normal,
                        Escape,
                        Closed
                    }
                    let mut state = State::Normal;
                    let mut i = 0;

                    while let Some(c) = chars.next() {
                        i += c.len_utf8();
                        match c {
                            '\\' => match state {
                                State::Escape => state = State::Normal,
                                _ => state = State::Escape
                            },
                            '"' => match state {
                                State::Escape => state = State::Normal,
                                _ => {
                                    state = State::Closed;
                                    break
                                }
                            },
                            _ => match state {
                                State::Escape => state = State::Normal,
                                _ => {}
                            }
                        }
                    }
                    let str_rep = &c_onwards[..i + c.len_utf8()];
                    self.rest = &self.rest[i..];
                    self.byte += i;
                    if state != State::Closed {
                        // string wasn't closed
                        return Some(Err(UnterminatedStringError {
                            src: self.whole.to_string(),
                            err_span: SourceSpan::from(self.byte - c.len_utf8() - i..self.byte),
                        }.into()))
                    }
                    return Some(Ok(Token::new(TokenKind::STRING, str_rep, c_at)));
                },
                Started::Number => {
                    #[derive(PartialEq)]
                    enum State {
                        BeforeDot,
                        Dot,
                        AfterDot
                    }
                    let mut state = State::BeforeDot;
                    let mut i = 0;
                    
                    while let Some(c) = chars.next() {
                        match c {
                            '0'..='9' | '_'  => {
                                match state {
                                    State::Dot => { state = State::AfterDot }
                                    _ => {}
                                }
                            },
                            '.' => match state {
                                State::BeforeDot => { state = State::Dot; },
                                _ => break
                            },
                            _ => break
                        }
                        i += c.len_utf8();
                    }
                    if state == State::Dot {
                        // means last characted is a dot
                        i -= '.'.len_utf8();
                    }
                    let str_rep = &c_onwards[..i + c.len_utf8()];
                    let str_rep_normalized = str_rep.replace("_", "");
                    self.rest = &self.rest[i..];
                    self.byte += i;
                    let num = match str_rep_normalized.parse() {
                        Ok(n) => n,
                        Err(err) => {
                            return Some(Err(miette::miette! {
                                labels = vec![
                                    LabeledSpan::at(self.byte - i..self.byte, "here")
                                ],
                                "Failed to parse number '{str_rep}' in input\n error: {err:?}"
                            }.with_source_code(self.whole.to_string())));
                        },
                    };
                    return Some(Ok(Token::new(TokenKind::NUMBER(num), str_rep, c_at)))

                },
                Started::Ident =>  {
                    let first_non_ident = c_onwards
                        .find(|c| !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
                        .unwrap_or_else(|| c_onwards.len());
                    let str_rep = &c_onwards[..first_non_ident];
                    let extra_bytes = str_rep.len() - c.len_utf8();
                    self.byte += extra_bytes;
                    self.rest = &self.rest[extra_bytes..];

                    let kind = match str_rep {
                        "and" => TokenKind::AND,
                        "else" => TokenKind::ELSE,
                        "false" => TokenKind::FALSE,
                        "true" => TokenKind::TRUE,
                        "for" => TokenKind::FOR,
                        "fun" => TokenKind::FUN,
                        "if" => TokenKind::IF,
                        "nil" => TokenKind::NIL,
                        "or" => TokenKind::OR,
                        "return" => TokenKind::RETURN,
                        "var" => TokenKind::VAR,
                        "while" => TokenKind::WHILE,
                        "print" => TokenKind::PRINT,
                        "break" => TokenKind::BREAK,
                        "continue" => TokenKind::CONTINUE,
                        _ => TokenKind::IDENT
                    };
                    return Some(Ok(Token::new(kind, str_rep, c_at)));
                },
                Started::Comment => {
                    if let Some(this_c) = chars.next() {
                        if this_c == '/' {
                            let n_line = c_onwards
                                .find(|c| matches!(c, '\n'))
                                .unwrap_or_else(|| c_onwards.len());
                            self.rest = &c_onwards[n_line..];
                            self.byte += c_onwards.len() - self.rest.len() - c.len_utf8();
                            continue;
                        }
                    }
                    return just(TokenKind::SLASH);
                }
                Started::IfEqualElse(yes, no) => {
                    self.rest = self.rest.trim_start();
                    let trimmed = c_onwards.len() - self.rest.len() - 1;
                    self.byte += trimmed;


                    if self.rest.starts_with('=') {
                        let span = &c_onwards[..c.len_utf8() + trimmed + 1];
                        self.rest = &self.rest[1..];
                        self.byte += 1;
                        return Some(Ok(Token::new(yes, span, c_at)));
                    } else {
                        return Some(Ok(Token::new(no, c_str, c_at)))
                    }
                }
            }
            // todo!() // unreachable
        }
    }
}