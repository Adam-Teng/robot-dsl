use robot_dsl::{
    syntax::{Expr, Visitor},
    token::{Token, TokenType},
};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&self, name: String, exprs: Vec<&Expr>) -> String {
        let mut r = String::new();
        r.push_str("(");
        r.push_str(&name);
        for e in &exprs {
            r.push_str(" ");
            r.push_str(&e.accept(self));
        }
        r.push_str(")");
        r
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_literal_expr(&self, value: String) -> String {
        value
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(operator.lexeme.clone(), vec![right])
    }
}

#[test]
fn test_printer() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Bang, "!", 1),
            right: Box::new(Expr::Literal {
                value: "123".to_string(),
            }),
        }),
        operator: Token::new(TokenType::Minus, "+", 1),
        right: Box::new(Expr::Literal {
            value: "45.67".to_string(),
        }),
    };
    let printer = AstPrinter;

    assert_eq!(printer.print(expression), "(+ (! 123) 45.67)");
}
