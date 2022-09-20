use crate::error::{parser_error, Error};
use crate::syntax::{Expr, LiteralValue};
use crate::token::{Token, TokenType};

pub struct Parser<'t> {
    pub tokens: &'t Vec<Token>,
    current: usize,
}

macro_rules! matches {
    ( $sel:ident, $( $x:expr ),* ) => {
        {
            if $( $sel.check($x) )||* {
                $sel.advance();
                true
            } else {
                false
            }
        }
    };
}

impl<'t> Parser<'t> {
    pub fn new(tokens: &'t Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression().ok()
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        if self.tokens[1].tpe == TokenType::BangEqual || self.tokens[1].tpe == TokenType::EqualEqual
        {
            self.equality()
        } else if self.tokens[1].tpe == TokenType::Plus || self.tokens[1].tpe == TokenType::Minus {
            self.addition()
        } else if self.tokens[0].tpe == TokenType::Bang {
            self.unary()
        } else {
            self.primary()
        }
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;

        while matches!(self, TokenType::BangEqual, TokenType::EqualEqual) {
            let operator = self.previous().clone();
            let right = self.primary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;

        while matches!(self, TokenType::Plus, TokenType::Minus) {
            let operator = self.previous().clone();
            let right = self.primary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if matches!(self, TokenType::Bang) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        let expr = match &self.peek().tpe {
            TokenType::False => Expr::Literal {
                value: LiteralValue::Boolean(false),
            },
            TokenType::True => Expr::Literal {
                value: LiteralValue::Boolean(true),
            },
            TokenType::Number { literal } => Expr::Literal {
                value: LiteralValue::Number(literal.clone()),
            },
            TokenType::String { literal } => Expr::Literal {
                value: LiteralValue::String(literal.clone()),
            },
            _ => return Err(self.error(self.peek(), "Expected expression")),
        };

        self.advance();

        Ok(expr)
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        token_type == self.peek().tpe
    }

    fn is_at_end(&self) -> bool {
        self.peek().tpe == TokenType::EOF
    }

    fn error(&self, token: &Token, message: &str) -> Error {
        parser_error(token, message);
        Error::Parse
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn previous(&self) -> &Token {
        self.tokens
            .get(self.current - 1)
            .expect("No previous token")
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.current)
            .expect("Peek into end of token stream.")
    }
}
