use miette::{Context, Error, LabeledSpan};

use crate::{error::Eof, exptree::{Atom, ConOp, ExprTree, If, Loop, Op, UOp, UnaryOp, VarDecl}, lexer::Lexer, token::{unescape, Token, TokenKind}};

pub struct Parser<'de> {
    whole: &'de str,
    lexer: Lexer<'de>
}

impl<'de> Parser<'de> {
    pub fn new(input: &'de str) -> Self {
        Self { whole: input, lexer: Lexer::new(input) }
    }

    pub fn parse(self) -> Result<ExprTree<'de>, Error> {
        todo!()
    }

    /// ## Parses statement
    /// Parses: `return`, `print`, `break`, `continue`, `var`, `fun`, `if`, `while`, `for` and `block` as statements.
    /// 
    /// Everything else is parsed as an expression
    /// 
    /// *It does NOT consume trailing semicolons or braces*
    pub fn parse_statement_within(&mut self) -> Result<ExprTree<'de>, Error> {
        let stmt_exp = match self.lexer.peek() {
            Some(Ok(Token { kind: TokenKind::RETURN, ..})) => {
                self.lexer.next(); // we advance lexer
                if matches!(self.lexer.peek(), Some(Ok(Token { kind: TokenKind::SEMICOLON, ..}))) {
                    ExprTree::UnaryOp(UnaryOp::new(UOp::Return, Box::new(ExprTree::Atom(Atom::Nil))))
                } else {
                    ExprTree::UnaryOp(UnaryOp::new(UOp::Return, 
                        Box::new(self.parse_expression_within(0).wrap_err("in statement")?)))
                }
            },
            Some(Ok(Token { kind: TokenKind::PRINT, ..})) => {
                self.lexer.next(); // we advance lexer
                ExprTree::UnaryOp(UnaryOp::new(UOp::Print, 
                    Box::new(self.parse_expression_within(0).wrap_err("in statement")?)))
            },
            Some(Ok(Token { kind: TokenKind::BREAK, ..})) => {
                self.lexer.next(); // we advance lexer
                if matches!(self.lexer.peek(), Some(Ok(Token { kind: TokenKind::SEMICOLON, ..}))) {
                    ExprTree::UnaryOp(UnaryOp::new(UOp::Break, Box::new(ExprTree::Atom(Atom::Nil))))
                } else {
                    ExprTree::UnaryOp(UnaryOp::new(UOp::Break, 
                        Box::new(self.parse_expression_within(0).wrap_err("in statement")?)))
                }
            },
            Some(Ok(Token { kind: TokenKind::CONTINUE, ..})) => {
                self.lexer.next(); // we advance lexer
                ExprTree::Atom(Atom::Continue)
            },
            Some(Ok(Token { kind: TokenKind::VAR, ..})) => 
                self.parse_var(false).wrap_err("in statement")?,
            Some(Ok(Token { kind: TokenKind::FUN, ..})) => 
                self.parse_fun(false).wrap_err("in statement")?,
            Some(Ok(Token { kind: TokenKind::IF, ..})) => 
                self.parse_if(false).wrap_err("in statement")?,
            Some(Ok(Token { kind: TokenKind::WHILE, ..})) => 
                self.parse_while(false).wrap_err("in statement")?,
            Some(Ok(Token { kind: TokenKind::FOR, ..})) => 
                self.parse_for(false).wrap_err("in statement")?,
            Some(Ok(Token { kind: TokenKind::LEFT_BRACE, ..})) => 
                self.parse_block(false).wrap_err("in statement")?,
            Some(Ok(Token { kind: TokenKind::SEMICOLON, ..})) => { // empty semicolon is statement
                ExprTree::Atom(Atom::Nil)
            },
            Some(Err(_)) => return Err(self.lexer.next().unwrap().err().unwrap()),
            None => return Err(Eof.into()),
            _ => self.parse_expression_within(0).wrap_err("in statement")?
        };
        Ok(stmt_exp)
    }

    /// ## Parsess only expressions
    /// It uses pratt-parsing and as everything is an expression we are allowed 
    /// to have `for_exp + 1 + if_exp` for example. It is recursive algorithm 
    /// that parses only expressions it stops on 
    /// `semicolon|right_paren|right_brace|comma`
    /// otherwise it tries to parse next expressions
    /// 
    /// ## Arguments
    /// - `min_bp` - minimal binding power if an operator has 
    /// left binding power lower than `min_bp` it will be aplied later
    /// 
    /// ## Returns
    /// - ExpressionTree
    /// - miette error if any occured durring parsing
    /// **(returning an error stops parsing right away)**
    pub fn parse_expression_within(&mut self, min_bp: u8) -> Result<ExprTree<'de>, Error> {
        let lhs = self.lexer.next_or_error().wrap_err("on left-hand side")?;
        let mut lhs = match lhs {
            // Atoms
            Token { kind: TokenKind::STRING, origin, ..} => ExprTree::Atom(Atom::String(unescape(origin))),
            Token { kind: TokenKind::NUMBER(n), .. } => ExprTree::Atom(Atom::Number(n)),
            Token { kind: TokenKind::NIL, .. } => ExprTree::Atom(Atom::Nil),
            Token { kind: TokenKind::TRUE, .. } => ExprTree::Atom(Atom::Bool(true)),
            Token { kind: TokenKind::FALSE, .. } => ExprTree::Atom(Atom::Bool(false)),
            Token { kind: TokenKind::CONTINUE, .. } => ExprTree::Atom(Atom::Continue),
            Token { kind: TokenKind::IDENT, origin, .. } => self.parse_ident(origin),
            // prefix/unary
            Token { kind: TokenKind::BANG | TokenKind::MINUS, ..} => {
                let uop = match lhs.kind {
                    TokenKind::BANG => UOp::Bang,
                    TokenKind::MINUS => UOp::Minus,
                    _ => unreachable!("checked above")
                };
                let (_, r_bp) = Self::prefix_binding_power(uop);
                let rhs = self
                    .parse_expression_within(r_bp)
                    .wrap_err("in unary expression rhs")?;
                ExprTree::UnaryOp(UnaryOp::new(uop, Box::new(rhs)))
            },
            Token { kind: TokenKind::LEFT_PAREN, .. } => {
                let inner = self
                    .parse_expression_within(0)
                    .wrap_err("in bracketed expresion")?;
                self.lexer
                    .expect_next(TokenKind::RIGHT_PAREN, "Unexpected bracketed expression terminator")
                    .wrap_err("after bracketed expression")?;
                ExprTree::UnaryOp(UnaryOp::new(UOp::Group, Box::new(inner)))
            },
            // special
            Token { kind: TokenKind::LEFT_BRACE, .. } => self.parse_block(true)?,
            Token { kind: TokenKind::FOR, .. } => self.parse_for(true)?,
            Token { kind: TokenKind::IF, .. } => self.parse_if(true)?,
            Token { kind: TokenKind::WHILE, .. } => self.parse_while(true)?,

            token => return Err(miette::miette! {
                labels = vec![
                    LabeledSpan::at(token.offset..token.offset + token.origin.len(), "here"),
                ],
                help = format!("Unexpected {token:?}"),
                "Expected the left-hand side of an expression"
            }.with_source_code(self.whole.to_string()))
        };

        // Here we loop over operators and recursively parse expressions while 
        // those operators have proper binding power
        loop {
            let op = self.lexer.peek();
            if op.map_or(false, |op| op.is_err()) {
                return Err(self.lexer.next().expect("checked Some above").unwrap_err()).wrap_err("after left-hand side");
            }
            let op = match op {
                None => break, // EOF
                // arithmetic
                Some(Ok(Token { kind: TokenKind::MINUS, ..})) => Op::Minus,
                Some(Ok(Token { kind: TokenKind::PLUS, ..})) => Op::Plus,
                Some(Ok(Token { kind: TokenKind::STAR, ..})) => Op::Star,
                Some(Ok(Token { kind: TokenKind::SLASH, ..})) => Op::Slash,

                // binary
                Some(Ok(Token { kind: TokenKind::BANG_EQUAL, ..})) => Op::BangEqual,
                Some(Ok(Token { kind: TokenKind::EQUAL_EQUAL, ..})) => Op::EqualEqual,
                Some(Ok(Token { kind: TokenKind::LESS_EQUAL, ..})) => Op::LessEqual,
                Some(Ok(Token { kind: TokenKind::GREATER_EQUAL, ..})) => Op::GreaterEqual,
                Some(Ok(Token { kind: TokenKind::LESS, ..})) => Op::Less,
                Some(Ok(Token { kind: TokenKind::GREATER, ..})) => Op::Greater,
                Some(Ok(Token { kind: TokenKind::AND, ..})) => Op::And,
                Some(Ok(Token { kind: TokenKind::OR, ..})) => Op::Or,

                // assigment
                Some(Ok(Token { kind: TokenKind::EQUAL, ..})) => Op::Equal,

                // ending
                Some(Ok(Token { kind: TokenKind::RIGHT_PAREN | TokenKind::RIGHT_BRACE | 
                    TokenKind::SEMICOLON | TokenKind::COMMA, .. })) => return Ok(lhs),
                
                // unexpected
                Some(Ok(token)) => return Err(miette::miette! {
                    labels = vec![
                        LabeledSpan::at(token.offset..token.offset + token.origin.len(), "here"),
                    ],
                    help = format!("Unexpected {token:?}"),
                    "Expected an expression operator"
                }.with_source_code(self.whole.to_string())),
                Some(Err(_)) => unreachable!("checked above")
            };
            if let Some((_l_bp, ())) = Self::postfix_binding_power(op) {
                unimplemented!("Not yet needed")
            }
            if let Some((l_bp, r_bp)) = Self::infix_binding_power(op) {
                if l_bp < min_bp { break; }
                self.lexer.next(); // consume the op token
                let rhs = self.parse_expression_within(r_bp)
                    .wrap_err("on the right-hand side")?;
                lhs = ExprTree::ConOp(ConOp::new(op, Box::new(lhs), Box::new(rhs)));
                continue;
            }
            break;
        }
        Ok(lhs)
    }

    /// ## Parses something starting from an ident
    /// something starting from ident can be either:
    /// - ident itself (variable reference)
    /// - function call
    fn parse_ident(&mut self, name: &'de str) -> ExprTree<'de> {
        if matches!(self.lexer.peek(), Some(Ok(Token { kind: TokenKind::LEFT_PAREN, .. }))) {
            self.lexer.next(); // we advance lexer
            // its a function call
            todo!()
        }
        ExprTree::Atom(Atom::Ident(name))
    }

    /// ## Parses for loop
    fn parse_for(&mut self, skip_first_keyword: bool) -> Result<ExprTree<'de>, Error> {
        if !skip_first_keyword {
            self.lexer.expect_next(TokenKind::FOR, "expected for loop")?;
        }
        self.lexer.expect_next(TokenKind::LEFT_PAREN, "expected (")
            .wrap_err("in for loop")?;
        let init = 
        if matches!(self.lexer.peek(), Some(Ok(Token { kind: TokenKind::SEMICOLON, ..}))) {
            None
        } else {
            Some(Box::new(self.parse_statement_within()
                .wrap_err("in for loop's init")?))
        };
        self.lexer.expect_next(TokenKind::SEMICOLON, "expected ;")
            .wrap_err("in for loop")?;
        let cond = 
        if matches!(self.lexer.peek(), Some(Ok(Token { kind: TokenKind::SEMICOLON, ..}))) {
            ExprTree::Atom(Atom::Bool(true))
        } else {
            self.parse_statement_within()
                .wrap_err("in for loop's cond")?
        };
        self.lexer.expect_next(TokenKind::SEMICOLON, "expected ;")
            .wrap_err("in for loop")?;
        let step = 
        if matches!(self.lexer.peek(), Some(Ok(Token { kind: TokenKind::RIGHT_PAREN, ..}))) {
            None
        } else {
            Some(Box::new(self.parse_statement_within()
            .wrap_err("in for loop's step")?))
        };
        self.lexer.expect_next(TokenKind::RIGHT_PAREN, "expected )")
            .wrap_err("in for loop")?;
        let block = self.parse_block(false)
            .wrap_err("in for loop's block")?;

        let loop_strc = Loop::new(
            Box::new(cond), 
            init, 
            step, 
            Box::new(block));
        Ok(ExprTree::Loop(loop_strc))
    }

    /// ## Parses while loop
    fn parse_while(&mut self, skip_first_keyword: bool) -> Result<ExprTree<'de>, Error> {
        if !skip_first_keyword {
            self.lexer.expect_next(TokenKind::WHILE, "expected while loop")?;
        }
        self.lexer.expect_next(TokenKind::LEFT_PAREN, "expected (")
            .wrap_err("in while loop")?;
        let cond = self.parse_statement_within()
            .wrap_err("in while loop's cond")?;
        self.lexer.expect_next(TokenKind::RIGHT_PAREN, "expected )")
            .wrap_err("in while loop")?;
        let block = self.parse_block(false)
            .wrap_err("in while loop's block")?;

        let loop_strc = Loop::new(
            Box::new(cond), 
            None, 
            None, 
            Box::new(block));
        Ok(ExprTree::Loop(loop_strc))
    }

    /// ## Parses if
    fn parse_if(&mut self, skip_first_keyword: bool) -> Result<ExprTree<'de>, Error> {
        if !skip_first_keyword {
            self.lexer.expect_next(TokenKind::IF, "expected if")?;
        }
        self.lexer.expect_next(TokenKind::LEFT_PAREN, "expected (").wrap_err("in if condition")?;
        let condition = Box::new(self.parse_expression_within(0).wrap_err("in if condition")?);
        self.lexer.expect_next(TokenKind::RIGHT_PAREN, "expected )").wrap_err("in if condition")?;
        let yes_stmt = Box::new(self.parse_block(false).wrap_err("in if expression")?);
        let no_stmt = if matches!(self.lexer.peek(), Some(Ok(Token { kind: TokenKind::ELSE, .. }))) {
            self.lexer.next(); // we advance lexer, checked above
            Some(Box::new(self.parse_block(false)?))
        } else {
            None
        };

        Ok(ExprTree::If(If::new(condition, yes_stmt, no_stmt)))
    }

    /// ## Parses block
    fn parse_block(&mut self, skip_first_keyword: bool) -> Result<ExprTree<'de>, Error> {
        if !skip_first_keyword {
            self.lexer.expect_next(TokenKind::LEFT_BRACE, "expected block")?;
        }
        let mut stmts: Vec<ExprTree<'de>> = Vec::new();
        let mut ret_expr: Option<ExprTree<'de>> = None;
        
        loop {
            match self.lexer.peek() {
                Some(Ok(Token { kind: TokenKind::RIGHT_BRACE, ..})) => {
                    ret_expr = stmts.pop();
                    break;
                },
                Some(Ok(Token { kind: TokenKind::SEMICOLON, ..})) => { 
                    self.lexer.next(); // we advance lexer 
                    if matches!(self.lexer.peek(), Some(Ok(Token {kind: TokenKind::RIGHT_BRACE, ..}))) {
                        break;
                    }
                },
                _ => stmts.push(self.parse_statement_within().wrap_err("in block")?),
            }
        };
        self.lexer.expect_next(TokenKind::RIGHT_BRACE, "expected }")?;

        Ok(ExprTree::Block(stmts, ret_expr.map(|e| Box::new(e))))
    }

    /// ## Parses function declaration
    fn parse_fun(&mut self, skip_first_keyword: bool) -> Result<ExprTree<'de>, Error> {
        if !skip_first_keyword {
            self.lexer.expect_next(TokenKind::FUN, "expected fun")?;
        }
        self.lexer.expect_next(TokenKind::LEFT_PAREN, "expected (")
            .wrap_err("in function decl")?;
        while !matches!(self.lexer.peek(), Some(Ok(Token { kind: TokenKind::RIGHT_PAREN, ..}))) {
            todo!() // parse idents
        }
        self.lexer.expect_next(TokenKind::RIGHT_PAREN, "expected )")?;
        todo!()
    }

    /// ## Parses var declaration
    fn parse_var(&mut self, skip_first_keyword: bool) -> Result<ExprTree<'de>, Error> {
        if !skip_first_keyword {
            self.lexer.expect_next(TokenKind::VAR, "expected var")?;
        }
        let ident = self.lexer.expect_next(TokenKind::IDENT, "expected ident")
            .wrap_err("in variable declaration")?.origin;
        if matches!(self.lexer.peek(), Some(Ok(Token { kind: TokenKind::SEMICOLON, ..}))) {
            return Ok(ExprTree::Var(VarDecl::new(ident, None)))
        }
        self.lexer.expect_next(TokenKind::EQUAL, "expected =")
            .wrap_err("in variable declaration")?;
        let expr = self.parse_expression_within(0)
            .wrap_err("in variable declaration")?;
        let var_decl = VarDecl::new(ident, Some(Box::new(expr)));
        Ok(ExprTree::Var(var_decl))
    }

    /// ## Prefix bp
    /// Returns prefix binding power for given unary operator
    fn prefix_binding_power(uop: UOp) -> ((), u8) {
        match uop {
            UOp::Minus | UOp::Bang => ((), 20),
            UOp::Group | UOp::Break | UOp::Return | UOp::Print => panic!("Groups and statements should be handled independently"),
            // _ => panic!("Should never be called on non prefix operator")
        }
    }
    
    /// ## Infix bp
    /// Returns infix binding power for given infix operator
    fn infix_binding_power(op: Op) -> Option<(u8, u8)> {
        let res = match op {
            Op::Plus | Op::Minus => (7, 8),
            Op::Star | Op::Slash => (9, 10),
            Op::Less | Op::LessEqual | 
            Op::EqualEqual | Op::GreaterEqual | 
            Op::Greater | Op::BangEqual => (5, 6),
            Op::Equal => (1, 2),
            Op::And | Op::Or => (3, 4),
            // _ => return None,
        };
        Some(res)
    }
    
    /// ## Postfix bp
    /// Returns postfix binding power for given postfix operator
    /// 
    /// *Not yet required*
    fn postfix_binding_power(_op: Op) -> Option<(u8, ())> {
        // it can be used later for example for array field access 
        None
    } 

    #[allow(dead_code)]
    /// ## Skips semicolon
    /// Skips the next character only if it is semicolon
    /// It also propagates any lexer errors
    fn skip_trailing_semicolon(&mut self) -> Result<(), Error> {
        let peek = self.lexer.peek();
        match peek {
            Some(Ok(Token { kind: TokenKind::SEMICOLON, ..})) => {
                self.lexer.next(); // we advance lexer
                Ok(())
            }
            Some(Err(_)) => {
                let err = self.lexer.next().unwrap().err().unwrap();
                Err(err)
            }
            _ => Ok(())
        }
    }
}