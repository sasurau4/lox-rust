use super::error::{Error, Result};
use super::expr::Expr;
use super::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    values: RefCell<HashMap<String, Expr>>,
}

impl Environment {
    pub fn new() -> Environment {
        let values = HashMap::new();
        Environment {
            values: RefCell::new(values),
        }
    }

    pub fn define(&self, name: String, value: &Expr) {
        self.values.borrow_mut().insert(name, value.clone());
    }

    pub fn get(&self, name: &Token) -> Result<Expr> {
        match self.values.borrow_mut().get(&name.lexeme) {
            Some(r) => Ok(r.clone()),
            None => Err(Error::RuntimeError(
                name.clone(),
                String::from(format!("Undefined variableble '{}'.", &name.lexeme)),
            )),
        }
    }
}
