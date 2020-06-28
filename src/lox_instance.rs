use super::error::{Error, Result};
use super::lox_class::LoxClass;
use super::object::Object;
use super::token::Token;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(class: LoxClass) -> LoxInstance {
        LoxInstance {
            class,
            fields: HashMap::new(),
        }
    }
    pub fn get(self, name: &Token) -> Result<Object> {
        if let Some(o) = self.fields.get(&name.lexeme) {
            return Ok(o.clone());
        }
        if let Some(method) = self.class.find_method(name.lexeme.clone()) {
            return Ok(Object::Func(method.clone()));
        }
        Err(Error::RuntimeError(
            name.clone(),
            format!("Undefined property '{}'", name.lexeme),
        ))
    }
    pub fn set(mut self, name: &Token, value: &Object) {
        self.fields.insert(name.lexeme.clone(), value.clone());
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} instance", self.class.name)
    }
}
