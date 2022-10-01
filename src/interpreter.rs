use crate::env::Environment;
use crate::error::Error;
use crate::object::Object;
use crate::syntax::{expr, stmt};
use crate::syntax::{Expr, LiteralValue, Stmt};
use crate::token::{Token, TokenType};

use std::cell::RefCell;
use std::rc::Rc;
use std::io;
use std::thread::sleep;
use std::time::Duration;

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), Error> {
        for statement in statements {
            self.execute(statement)?;
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

    fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), Error> {
        let previous = self.environment.clone();
        let steps = || -> Result<(), Error> {
            self.environment = environment;
            for statement in statements {
                self.execute(statement)?
            }
            Ok(())
        };
        let result = steps();
        self.environment = previous;
        result
    }

    fn is_truthy(&self, object: &Object) -> bool {
        match object {
            Object::Null => false,
            Object::Boolean(b) => b.clone(),
            _ => true,
        }
    }

    fn is_equal(&self, left: &Object, right: &Object) -> bool {
        left.equals(right)
    }

    fn stringify(&self, object: Object) -> String {
        match object {
            Object::Null => "nil".to_string(),
            Object::Number(n) => n.to_string(),
            Object::Boolean(b) => b.to_string(),
            Object::String(s) => s,
        }
    }

    // Equivalent to checkNumberOperands
    fn number_operand_error<R>(&self, operator: &Token) -> Result<R, Error> {
        Err(Error::Runtime {
            token: operator.clone(),
            message: "Operand must be a number.".to_string(),
        })
    }
}

impl expr::Visitor<Object> for Interpreter {
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, Error> {
        let l = self.evaluate(left)?;
        let r = self.evaluate(right)?;

        match &operator.tpe {
            TokenType::Minus => match (l, r) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Number(left_number - right_number))
                }
                _ => self.number_operand_error(operator),
            },
            TokenType::Plus => match (l, r) {
                (Object::Number(left_number), Object::Number(right_number)) => {
                    Ok(Object::Number(left_number + right_number))
                }
                (Object::String(left_string), Object::String(right_string)) => {
                    Ok(Object::String(left_string.clone() + &right_string))
                }
                _ => Err(Error::Runtime {
                    token: operator.clone(),
                    message: "Operands must be two numbers or two strings.".to_string(),
                }),
            },
            TokenType::BangEqual => Ok(Object::Boolean(!self.is_equal(&l, &r))),
            TokenType::EqualEqual => Ok(Object::Boolean(self.is_equal(&l, &r))),
            _ => unreachable!(),
        }
    }


    fn visit_literal_expr(&self, value: &LiteralValue) -> Result<Object, Error> {
        match value {
            LiteralValue::Boolean(b) => Ok(Object::Boolean(b.clone())),
            LiteralValue::Null => Ok(Object::Null),
            LiteralValue::Number(n) => Ok(Object::Number(n.clone())),
            LiteralValue::String(s) => Ok(Object::String(s.clone())),
        }
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, Error> {
        let right = self.evaluate(right)?;

        match &operator.tpe {
            TokenType::Minus => match right {
                Object::Number(n) => Ok(Object::Number(-n.clone())),
                _ => self.number_operand_error(operator),
            },
            TokenType::Bang => Ok(Object::Boolean(!self.is_truthy(&right))), // TODO: is_truthy could simply return an Object.
            _ => unreachable!(), // TODO: fail if right is not a number.
        }
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Object, Error> {
        self.environment.borrow().get(name)
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, Error> {
        let v = self.evaluate(value)?;
        self.environment.borrow_mut().assign(name, v.clone())?;
        Ok(v)
    }
}

impl stmt::Visitor<()> for Interpreter {
    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<(), Error> {
        self.execute_block(
            statements,
            Rc::new(RefCell::new(Environment::from(&self.environment))),
        );
        Ok(())
    }

    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), Error> {
        self.evaluate(expression)?;
        Ok(())
    }

    fn visit_branch_stmt(&mut self, condition: &Expr, then: &Stmt) -> Result<(), Error> {
        let condition = self.evaluate(condition)?;
        if self.is_truthy(&condition) {
            self.execute(then)?;
        }
        Ok(())
    }

    fn visit_loop_stmt(&mut self, body: &Stmt) -> Result<(), Error> {
        loop {
            self.execute(body)?;
        }
    }

    fn visit_speak_stmt(&mut self, expression: &Expr) -> Result<(), Error> {
        let value = self.evaluate(expression)?;
        println!("{}", self.stringify(value));
        Ok(())
    }

    fn visit_input_stmt(&mut self, name: &Token) -> Result<(), Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        // remove '\n'
        input.pop();
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Object::String(input));
        Ok(())
    }

    fn visit_listen_stmt(&mut self, time: &Expr) -> Result<(), Error> {
        let temp = self.evaluate(time)?;
        // from object to number
        let num = match temp {
            Object::Number(n) => n,
            _ => unreachable!(),
        };
        // from number to duration
        // sleep as seconds
        let dur = Duration::from_secs_f64(num);
        sleep(dur);
        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Option<Expr>) -> Result<(), Error> {
        let value: Object = initializer
            .as_ref()
            .map(|i| self.evaluate(i))
            .unwrap_or(Ok(Object::Null))?;

        self.environment.borrow_mut().define(name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_exit_stmt(&mut self) -> Result<(), Error> {
        std::process::exit(0);
    }
}
