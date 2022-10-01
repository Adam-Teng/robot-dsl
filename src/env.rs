use crate::error::Error;
use crate::object::Object;
use crate::token::Token;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

///
/// environment 类型，对 dsl 语言中的环境进行抽象
///
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>, // Parent
    values: HashMap<String, Object>,
}

impl Environment {
    ///
    /// 创建一个新的环境
    ///
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    ///
    /// 得到该环境的父环境
    ///
    /// # 参数列表
    /// * enclosing: 所求环境
    ///
    /// # 返回值
    /// * 父环境
    ///
    pub fn from(enclosing: &Rc<RefCell<Environment>>) -> Self {
        Environment {
            enclosing: Some(Rc::clone(enclosing)),
            values: HashMap::new(),
        }
    }

    ///
    /// 在当前环境中定义一个变量
    ///
    /// # 参数列表
    /// * name: 变量名
    /// * value: 变量值
    ///
    /// # 返回值
    /// * 无
    ///
    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    ///
    /// 在当前环境中得到该变量的相关信息
    ///
    /// # 参数列表
    /// * name: 变量名
    ///
    /// # 返回值
    /// * 该变量的相关信息
    ///
    pub fn get(&self, name: &Token) -> Result<Object, Error> {
        let key = &*name.lexeme;
        if let Some(value) = self.values.get(key) {
            Ok((*value).clone())
        } else {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow().get(name)
            } else {
                Err(Error::Runtime {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'.", key),
                })
            }
        }
    }

    ///
    /// 改变变量的值
    ///
    /// # 参数列表
    /// * name: 变量名
    /// * value: 变量值
    ///
    /// # 返回值
    /// * 无
    ///
    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), Error> {
        let key = &*name.lexeme;
        if self.values.contains_key(key) {
            self.values.insert(name.lexeme.clone(), value);
            Ok(())
        } else {
            if let Some(ref enclosing) = self.enclosing {
                enclosing.borrow_mut().assign(name, value)
            } else {
                Err(Error::Runtime {
                    token: name.clone(),
                    message: format!("Undefined variable '{}'", key),
                })
            }
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "values: {:?}", self.values)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn test_get() {
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut()
            .define("a".to_string(), Object::Number(1.0));
        env.borrow_mut()
            .define("b".to_string(), Object::Number(2.0));
        env.borrow_mut()
            .define("c".to_string(), Object::Number(3.0));

        assert!(env
            .borrow()
            .get(&Token::new(TokenType::Identifier, &"a".to_string(), 1))
            .is_ok());
        assert!(env
            .borrow()
            .get(&Token::new(TokenType::Identifier, &"b".to_string(), 1))
            .is_ok());
        assert!(env
            .borrow()
            .get(&Token::new(TokenType::Identifier, &"c".to_string(), 1))
            .is_ok());
    }

    #[test]
    fn test_get_undefined() {
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut()
            .define("a".to_string(), Object::Number(1.0));
        env.borrow_mut()
            .define("b".to_string(), Object::Number(2.0));
        env.borrow_mut()
            .define("c".to_string(), Object::Number(3.0));

        assert!(env
            .borrow()
            .get(&Token::new(TokenType::Identifier, &"d".to_string(), 1))
            .is_err());
    }

    #[test]
    fn test_assign() {
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut()
            .define("a".to_string(), Object::Number(1.0));
        env.borrow_mut()
            .define("b".to_string(), Object::Number(2.0));
        env.borrow_mut()
            .define("c".to_string(), Object::Number(3.0));

        assert!(env
            .borrow_mut()
            .assign(
                &Token::new(TokenType::Identifier, &"a".to_string(), 1),
                Object::Number(4.0)
            )
            .is_ok());
        assert!(env
            .borrow_mut()
            .assign(
                &Token::new(TokenType::Identifier, &"b".to_string(), 1),
                Object::Number(5.0)
            )
            .is_ok());
        assert!(env
            .borrow_mut()
            .assign(
                &Token::new(TokenType::Identifier, &"c".to_string(), 1),
                Object::Number(6.0)
            )
            .is_ok());
    }

    #[test]
    fn test_assign_undefined() {
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut()
            .define("a".to_string(), Object::Number(1.0));
        env.borrow_mut()
            .define("b".to_string(), Object::Number(2.0));
        env.borrow_mut()
            .define("c".to_string(), Object::Number(3.0));

        assert!(env
            .borrow_mut()
            .assign(
                &Token::new(TokenType::Identifier, &"d".to_string(), 1),
                Object::Number(4.0)
            )
            .is_err());
    }
}
