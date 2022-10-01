use crate::env::Environment;
use crate::error::Error;
use crate::function::Function;
use crate::object::Object;
use crate::syntax::{expr, stmt};
use crate::syntax::{Expr, LiteralValue, Stmt};
use crate::token::{Token, TokenType};

use std::cell::RefCell;
use std::rc::Rc;
use std::io;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        let clock: Object = Object::Callable(Function::Native {
            arity: 0,
            body: Box::new(|_args: &Vec<Object>| {
                Object::Number(
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Could not retrieve time.")
                        .as_millis() as f64,
                )
            }),
        });
        globals.borrow_mut().define("clock".to_string(), clock);
        Interpreter {
            globals: Rc::clone(&globals),
            environment: Rc::clone(&globals),
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

    pub fn execute_block(
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
            Object::Callable(f) => f.to_string(),
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
                // number + string
                (Object::Number(left_number), Object::String(right_string)) => {
                    Ok(Object::String(left_number.to_string() + &right_string))
                }
                (Object::String(left_string), Object::Number(right_number)) => {
                    Ok(Object::String(left_string + &right_number.to_string()))
                }
                // others
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

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<Object, Error> {
        let callee_value = self.evaluate(callee)?;

        let argument_values: Result<Vec<Object>, Error> = arguments
            .into_iter()
            .map(|expr| self.evaluate(expr))
            .collect();
        let args = argument_values?;

        if let Object::Callable(function) = callee_value {
            let args_size = args.len();
            if args_size != function.arity() {
                Err(Error::Runtime {
                    token: paren.clone(),
                    message: format!(
                        "Expected {} arguments but got {}.",
                        function.arity(),
                        args_size
                    ),
                })
            } else {
                function.call(self, &args)
            }
        } else {
            Err(Error::Runtime {
                token: paren.clone(),
                message: "Can only call functions and classes.".to_string(),
            })
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

    fn visit_function_stmt(
        &mut self,
        name: &Token,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
    ) -> Result<(), Error> {
        let function = Function::User {
            name: name.clone(),
            params: params.clone(),
            body: body.clone(),
            closure: Rc::clone(&self.environment),
        };
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Object::Callable(function));
        Ok(())
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

    fn visit_inputn_stmt(&mut self, name: &Token) -> Result<(), Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        // remove '\n'
        input.pop();
        let number: f64 = input.parse().unwrap();
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Object::Number(number));
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
