use crate::error::Error;
use crate::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Boolean(bool),
    Null,
    Number(f64),
    String(String),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Null => write!(f, "null"),
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn expr::Visitor<R>) -> Result<R, Error> {
        match self {
            Expr::Assign { name, value } => visitor.visit_assign_expr(name, value),
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
            Expr::Variable { name } => visitor.visit_variable_expr(name),
        }
    }
}

pub mod expr {
    use super::{Expr, LiteralValue};
    use crate::error::Error;
    use crate::token::Token;

    pub trait Visitor<R> {
        fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<R, Error>;
        fn visit_binary_expr(
            &mut self,
            left: &Expr,
            operator: &Token,
            right: &Expr,
        ) -> Result<R, Error>;

        fn visit_literal_expr(&self, value: &LiteralValue) -> Result<R, Error>;
        fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<R, Error>;
        fn visit_variable_expr(&mut self, name: &Token) -> Result<R, Error>;
    }
}

pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Expression {
        expression: Expr,
    },
    Speak {
        expression: Expr,
    },
    Input {
        input: Token,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    Null,
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn stmt::Visitor<R>) -> Result<R, Error> {
        match self {
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Speak { expression } => visitor.visit_speak_stmt(expression),
            Stmt::Input { input} => visitor.visit_input_stmt(input),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, initializer),
            Stmt::Null => unimplemented!(),
        }
    }
}

pub mod stmt {
    use super::{Expr, Stmt};
    use crate::{error::Error, token::Token};

    pub trait Visitor<R> {
        fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<R, Error>;
        fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<R, Error>;
        fn visit_speak_stmt(&mut self, expression: &Expr) -> Result<R, Error>;
        fn visit_input_stmt(&mut self, name: &Token) -> Result<R, Error>;
        fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<R, Error>;
    }
}
