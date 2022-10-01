use crate::error::{parser_error, Error};
use crate::syntax::{Expr, LiteralValue, Stmt};
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

    // for test expression calculating
    pub fn calculate(&mut self) -> Option<Expr> {
        self.expression().ok()
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt, Error> {
        let statement = if matches!(self, TokenType::Var) {
            self.var_declaration()
        } else {
            self.statement()
        };

        match statement {
            Err(Error::Parse) => {
                self.synchronize();
                Ok(Stmt::Null)
            }
            other => other,
        }
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        if matches!(self, TokenType::Speak) {
            self.speak_statement()
        } else if matches!(self, TokenType::Input) {
            self.input_statement()
        } else if matches!(self, TokenType::Listen) {
            self.listen_statement()
        } else if matches!(self, TokenType::Step) {
            self.function("step")
        } else if matches!(self, TokenType::Branch) {
            self.branch_statement()
        } else if matches!(self, TokenType::Loop) {
            self.loop_statement() 
        } else if matches!(self, TokenType::Exit) {
            self.exit_statement()
        } else if matches!(self, TokenType::Inputn) {
            self.inputn_statement()
        } else if matches!(self, TokenType::LeftBrace) {
            Ok(Stmt::Block {
                statements: self.block()?,
            })
        } else {
            self.expression_statement()
        }
    }

    fn exit_statement(&mut self) -> Result<Stmt, Error> {
        Ok(Stmt::Exit)
    }

    fn inputn_statement(&mut self) -> Result<Stmt, Error> {
        let input = self.consume(TokenType::Identifier, "Expect variable name.")?;
        self.consume(TokenType::SemiColon, "Expect ';' after input.")?;
        Ok(Stmt::Inputn { input })
    }

    fn function(&mut self, kind: &str) -> Result<Stmt, Error> {
        let name = self.consume(
            TokenType::Identifier,
            format!("Expect {} name.", kind).as_str(),
        )?;
        self.consume(
            TokenType::LeftParen,
            format!("Expect '(' after {} name.", kind).as_str(),
        )?;
        let mut params: Vec<Token> = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if params.len() >= 255 {
                    // We are not returning an error here.
                    self.error(self.peek(), "Cannot have more than 255 parameters.");
                }
                params.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            format!("Expect '{{' before {} body.", kind).as_str(),
        )?;
        let body = self.block()?;
        Ok(Stmt::Function { name, params, body })
    }

    fn loop_statement(&mut self) -> Result<Stmt, Error> {
        let body = Box::new(self.statement()?);
        Ok(Stmt::Loop { body })
    }

    fn branch_statement(&mut self) -> Result<Stmt, Error> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'branch'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after branch condition.")?;
        let then = Box::new(self.statement()?);
        Ok(Stmt::Branch { condition, then })
    }

    fn speak_statement(&mut self) -> Result<Stmt, Error> {
        let value = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after value.")?;
        Ok(Stmt::Speak { expression: value })
    }

    fn input_statement(&mut self) -> Result<Stmt, Error> {
        // 'input str' let str be input
        let input = self.consume(TokenType::Identifier, "Expect variable name.")?;
        self.consume(TokenType::SemiColon, "Expect ';' after input.")?;
        Ok(Stmt::Input { input })
    }

    fn listen_statement(&mut self) -> Result<Stmt, Error> {
        let time = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after time.")?;
        Ok(Stmt::Listen { time })
    }

    fn var_declaration(&mut self) -> Result<Stmt, Error> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if matches!(self, TokenType::Equal) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::SemiColon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    fn expression_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.assignment()
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.addition()?;

        while matches!(self, TokenType::BangEqual, TokenType::EqualEqual) {
            let operator = self.previous().clone();
            let right = self.addition()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, Error> {
        let expr = self.equality()?;

        if matches!(self, TokenType::Equal) {
            let value = Box::new(self.assignment()?);

            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign { name, value });
            }

            let equals = self.previous();
            self.error(equals, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn addition(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while matches!(self, TokenType::Plus, TokenType::Minus) {
            let operator = self.previous().clone();
            let right = self.unary()?;
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
            self.call()
        }
    }

    fn call(&mut self) -> Result<Expr, Error> {
        let mut expr = self.primary()?;

        loop {
            if matches!(self, TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, Error> {
        let mut arguments: Vec<Expr> = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    // We are just reporting the error but not return them.
                    self.error(self.peek(), "Cannot have more than 255 arguments.");
                }
                arguments.push(self.expression()?);
            }
        }

        let parent = self.consume(TokenType::RightParen, "Exprec ')' after arguments.")?;
        Ok(Expr::Call {
            callee: Box::new(callee),
            paren: parent,
            arguments,
        })
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        let expr = match &self.peek().tpe {
            TokenType::False => Expr::Literal {
                value: LiteralValue::Boolean(false),
            },
            TokenType::True => Expr::Literal {
                value: LiteralValue::Boolean(true),
            },
            TokenType::Nil => Expr::Literal {
                value: LiteralValue::Null,
            },
            TokenType::Number { literal } => Expr::Literal {
                value: LiteralValue::Number(literal.clone()),
            },
            TokenType::String { literal } => Expr::Literal {
                value: LiteralValue::String(literal.clone()),
            },
            TokenType::Identifier => Expr::Variable {
                name: self.peek().clone(),
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

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, Error> {
        if self.check(token_type) {
            Ok(self.advance().clone())
        } else {
            Err(self.error(self.peek(), message))
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().tpe == TokenType::SemiColon {
                return;
            }

            match self.peek().tpe {
                TokenType::Var
                | TokenType::Branch
                | TokenType::Exit
                | TokenType::Input
                | TokenType::Listen
                | TokenType::Speak
                | TokenType::Loop
                | TokenType::Step => return,
                _ => self.advance(),
            };
        }
    }
}
