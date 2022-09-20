use crate::error::Error;
use crate::syntax::{Expr, LiteralValue, Stmt, expr, stmt};
use crate::token::{Token, TokenType};
enum Object {
    Boolean(bool),
    Number(f64),
    String(String),
}

impl Object {
    fn equals(&self, other: &Object) -> bool {
        match (self, other) {
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            _ => false,
        }
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), Error> {
        for statement in statements {
            self.execute(&statement)?;
        }

        Ok(())
    }

    pub fn interpret_cal(&mut self, expression: &Expr) -> Result<String, Error> {
        self.evaluate(expression).map(|value| self.stringify(value))
    }

    fn evaluate(&mut self, expression: &Expr) -> Result<Object, Error> {
        expression.accept(self)
    }

    fn execute(&mut self, statement: &Stmt) -> Result<(), Error> {
        statement.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> bool {
        match object {
            Object::Boolean(value) => *value,
            _ => false,
        }
    }

    fn is_equal(&self, left: &Object, right: &Object) -> bool {
        left.equals(right)
    }

    fn stringify(&self, object: Object) -> String {
        match object {
            Object::Number(n) => n.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::String(s) => s,
        }
    }

    fn number_operand_error<R>(&self, operator: &Token) -> Result<R, Error> {
        Err(Error::Runtime {
            token: operator.clone(),
            message: "Operand must be a number.".to_string(),
        })
    }
}

impl expr::Visitor<Object> for Interpreter {
    fn visit_literal_expr(&self, value: &LiteralValue) -> Result<Object, Error> {
        match value {
            LiteralValue::Boolean(b) => Ok(Object::Boolean(b.clone())),
            LiteralValue::Number(n) => Ok(Object::Number(n.clone())),
            LiteralValue::String(s) => Ok(Object::String(s.clone())),
        }
    }

    fn visit_binary_expr(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object, Error> {
        let l = self.evaluate(left)?;
        let r = self.evaluate(right)?;

        match &operator.tpe {
            TokenType::Minus => match (l, r) {
                (Object::Number(left_number), Object::Number(right_number)) => Ok(Object::Number(left_number - right_number)),
                _ => self.number_operand_error(operator),
            }
            TokenType::Plus => match (l, r) {
                (Object::Number(left_number), Object::Number(right_number)) => Ok(Object::Number(left_number + right_number)),
                (Object::String(left_string), Object::String(right_string)) => Ok(Object::String(format!("{}{}", left_string, right_string))),
                _ => self.number_operand_error(operator),
            }
            TokenType::BangEqual => Ok(Object::Boolean(!self.is_equal(&l, &r))),
            TokenType::EqualEqual => Ok(Object::Boolean(self.is_equal(&l, &r))),
            _ => unreachable!(),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, Error> {
        let right = self.evaluate(right)?;

        match &operator.tpe {
            TokenType::Bang => Ok(Object::Boolean(!self.is_truthy(&right))),
            _ => unreachable!(),
        }
    }

}

impl stmt::Visitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), Error> {
        self.evaluate(expression)?;
        Ok(())
    }

    fn visit_speak_stmt(&mut self, expression: &Expr) -> Result<(), Error> {
        let value = self.evaluate(expression)?;
        println!("{}", self.stringify(value));
        Ok(())
    }
}
