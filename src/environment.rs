use super::error::{Error, Result};
use super::object::Object;
use super::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    values: RefCell<HashMap<String, Object>>,
}

impl Environment {
    pub fn new() -> Environment {
        let values = HashMap::new();
        Environment {
            values: RefCell::new(values),
        }
    }

    pub fn define(&self, name: String, value: &Object) {
        self.values.borrow_mut().insert(name, value.clone());
    }

    pub fn get(&self, name: &Token) -> Result<Object> {
        match self.values.borrow_mut().get(&name.lexeme) {
            Some(r) => Ok(r.clone()),
            None => Err(Error::RuntimeError(
                name.clone(),
                format!("Undefined variableble '{}'.", &name.lexeme),
            )),
        }
    }

    pub fn assign(&self, name: &Token, value: &Object) -> Result<()> {
        if self.values.borrow().contains_key(&name.lexeme) {
            self.values
                .borrow_mut()
                .insert(name.lexeme.clone(), value.clone());
            return Ok(());
        }
        Err(Error::RuntimeError(
            name.clone(),
            format!("Undefined variable '{}'", &name.lexeme),
        ))
    }
}
