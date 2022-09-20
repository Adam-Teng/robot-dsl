use robot_dsl::{
    error::Error,
    parser::Parser,
    scanner::Scanner,
    syntax::{Expr, LiteralValue, expr},
    token::{Token, TokenType},
};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> Result<String, Error> {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: String, exprs: Vec<&Expr>) -> Result<String, Error> {
        let mut r = String::new();
        r.push_str("(");
        r.push_str(&name);
        for e in &exprs {
            r.push_str(" ");
            r.push_str(&e.accept(self)?);
        }
        r.push_str(")");
        Ok(r)
    }
}

impl expr::Visitor<String> for AstPrinter {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<String, Error> {
        self.parenthesize(operator.lexeme.clone(), vec![left, right])
    }

    fn visit_literal_expr(&self, value: &LiteralValue) -> Result<String, Error> {
        Ok(value.to_string())
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<String, Error> {
        self.parenthesize(operator.lexeme.clone(), vec![right])
    }
}

#[test]
fn test_printer() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Bang, "!", 1),
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(123f64),
            }),
        }),
        operator: Token::new(TokenType::Minus, "+", 1),
        right: Box::new(Expr::Literal {
            value: LiteralValue::Number(45.67f64),
        }),
    };
    let mut printer = AstPrinter;

    assert_eq!(printer.print(expression).unwrap(), "(+ (! 123) 45.67)");
}

#[test]
fn test_parser_binary() {
    let mut scanner = Scanner::new("123 + 45".to_string());
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    // println!("{:?}", parser.tokens[1]);
    let expression = parser.calculate().expect("Failed to calculate");
    let mut printer = AstPrinter;

    assert_eq!(printer.print(expression).unwrap(), "(+ 123 45)");
}

#[test]
fn test_parser_unary() {
    let mut scanner = Scanner::new("!!123".to_string());
    let tokens = scanner.scan_tokens();

    let mut parser = Parser::new(tokens);
    // println!("{:?}", parser.tokens[0]);
    let expression = parser.calculate().expect("Failed to calculate");
    let mut printer = AstPrinter;

    assert_eq!(printer.print(expression).unwrap(), "(! (! 123))");
}
