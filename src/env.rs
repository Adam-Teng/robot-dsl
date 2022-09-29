use crate::error::Error;
use crate::object::Object;
use crate::token::Token;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>, // Parent
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn from(enclosing: &Rc<RefCell<Environment>>) -> Self {
        Environment {
            enclosing: Some(Rc::clone(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

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
        env.borrow_mut().define("a".to_string(), Object::Number(1.0));
        env.borrow_mut().define("b".to_string(), Object::Number(2.0));
        env.borrow_mut().define("c".to_string(), Object::Number(3.0));

        assert!(env.borrow().get(&Token::new(TokenType::Identifier, &"a".to_string(), 1)).is_ok());
        assert!(env.borrow().get(&Token::new(TokenType::Identifier, &"b".to_string(), 1)).is_ok());
        assert!(env.borrow().get(&Token::new(TokenType::Identifier, &"c".to_string(), 1)).is_ok());
    }

    #[test]
    fn test_get_undefined() {
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut().define("a".to_string(), Object::Number(1.0));
        env.borrow_mut().define("b".to_string(), Object::Number(2.0));
        env.borrow_mut().define("c".to_string(), Object::Number(3.0));

        assert!(env.borrow().get(&Token::new(TokenType::Identifier, &"d".to_string(), 1)).is_err());
    }

    #[test]
    fn test_assign() {
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut().define("a".to_string(), Object::Number(1.0));
        env.borrow_mut().define("b".to_string(), Object::Number(2.0));
        env.borrow_mut().define("c".to_string(), Object::Number(3.0));

        assert!(env.borrow_mut().assign(&Token::new(TokenType::Identifier, &"a".to_string(), 1), Object::Number(4.0)).is_ok());
        assert!(env.borrow_mut().assign(&Token::new(TokenType::Identifier, &"b".to_string(), 1), Object::Number(5.0)).is_ok());
        assert!(env.borrow_mut().assign(&Token::new(TokenType::Identifier, &"c".to_string(), 1), Object::Number(6.0)).is_ok());
    }

    #[test]
    fn test_assign_undefined() {
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut().define("a".to_string(), Object::Number(1.0));
        env.borrow_mut().define("b".to_string(), Object::Number(2.0));
        env.borrow_mut().define("c".to_string(), Object::Number(3.0));

        assert!(env.borrow_mut().assign(&Token::new(TokenType::Identifier, &"d".to_string(), 1), Object::Number(4.0)).is_err());
    }
}
