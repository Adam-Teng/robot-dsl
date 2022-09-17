use crate::token::Token;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value: String,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

pub trait Visitor<R> {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> R;
    fn visit_literal_expr(&self, value: String) -> R;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> R;
}

impl Expr {
    pub fn accept<R>(&self, visitor: &dyn Visitor<R>) -> R {
        match self {
            Expr::Binary {left, operator, right} => visitor.visit_binary_expr(left, operator, right),
            Expr::Literal { value } => visitor.visit_literal_expr(value.to_string()),
            Expr::Unary {operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}
