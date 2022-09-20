use crate::token::Token;
use crate::error::Error;
use std::fmt;

pub enum Expr {
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
}

pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Number(n) => write!(f, "{}", n),
            LiteralValue::String(s) => write!(f, "{}", s),
        }
    }
}

pub trait Visitor<R> {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> Result<R, Error>;
    fn visit_literal_expr(&self, value: &LiteralValue) -> Result<R, Error>;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> Result<R, Error>;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> Result<R, Error> {
        match self {
            Expr::Binary {left, operator, right} => visitor.visit_binary_expr(left, operator, right),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary {operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}
