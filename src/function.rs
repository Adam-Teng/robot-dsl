use crate::env::Environment;
use crate::error::Error;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::syntax::Stmt;
use crate::token::Token;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
///
/// 函数枚举类型
///
pub enum Function {
    /// 原生函数
    Native {
        /// 函数名
        arity: usize,
        /// 函数体
        body: Box<fn(&Vec<Object>) -> Object>,
    },

    /// 用户调用函数
    User {
        /// 函数名
        name: Token,
        /// 函数参数
        params: Vec<Token>,
        /// 函数体
        body: Vec<Stmt>,
        /// 函数环境
        closure: Rc<RefCell<Environment>>,
    },
}

impl Function {
    ///
    /// 调用语句
    ///
    /// # 参数列表
    /// * interpreter: 解释器
    /// * arguments:  dsl 函数的参数列表
    ///
    /// # 返回值
    /// * dsl 函数的返回值
    /// * 错误
    ///
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &Vec<Object>,
    ) -> Result<Object, Error> {
        match self {
            Function::Native { body, .. } => Ok(body(arguments)),
            Function::User {
                params,
                body,
                closure,
                ..
            } => {
                let environment = Rc::new(RefCell::new(Environment::from(closure)));
                for (param, argument) in params.iter().zip(arguments.iter()) {
                    environment
                        .borrow_mut()
                        .define(param.lexeme.clone(), argument.clone());
                }
                match interpreter.execute_block(body, environment) {
                    Err(other) => Err(other),
                    Ok(..) => Ok(Object::Null), // We don't have a return statement.
                }
            }
        }
    }

    /// 元数检查
    /// # 返回值
    /// * 元数
    pub fn arity(&self) -> usize {
        match self {
            Function::Native { arity, .. } => *arity,
            Function::User { params, .. } => params.len(),
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::Native { .. } => write!(f, "<native func>"),
            Function::User { name, .. } => write!(f, "<fn {}>", name.lexeme),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Function::Native { .. } => write!(f, "<native func>"),
            Function::User { name, .. } => write!(f, "<fn {}>", name.lexeme),
        }
    }
}
