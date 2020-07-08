use super::error::{Error, Result};
use super::object::Object;
use super::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    pub values: RefCell<HashMap<String, Object>>,
    pub is_repl: bool,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>, is_repl: bool) -> Environment {
        let values = HashMap::new();
        Environment {
            enclosing,
            values: RefCell::new(values),
            is_repl,
        }
    }

    pub fn define(&self, name: String, value: &Object) {
        self.values.borrow_mut().insert(name, value.clone());
    }

    pub fn get(&self, name: &Token) -> Result<Object> {
        if let Some(r) = self.values.borrow().get(&name.lexeme) {
            return Ok(r.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }
        Err(Error::RuntimeError(
            name.clone(),
            format!("Undefined variableble '{}'.", &name.lexeme),
        ))
    }

    pub fn get_at(&self, distance: usize, name: String) -> Result<Object> {
        match self.ancestor(distance).values.borrow().get(&name) {
            Some(o) => Ok(o.clone()),
            None => unreachable!(),
        }
    }

    pub fn assign_at(&self, distance: usize, name: Token, value: Object) {
        self.ancestor(distance)
            .values
            .borrow_mut()
            .insert(name.lexeme, value);
    }

    pub fn assign(&self, name: &Token, value: &Object) -> Result<()> {
        if self.values.borrow().contains_key(&name.lexeme) {
            self.values
                .borrow_mut()
                .insert(name.lexeme.clone(), value.clone());
            return Ok(());
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }
        Err(Error::RuntimeError(
            name.clone(),
            format!("Undefined variable '{}'", &name.lexeme),
        ))
    }

    fn ancestor(&self, distance: usize) -> Environment {
        let mut environment = self.clone();
        for _i in 0..distance {
            let enclosing = environment
                .enclosing
                .unwrap_or_else(|| panic!("No enclosing format at distance: {}", distance))
                .borrow()
                .clone();
            environment = enclosing
        }
        environment
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "This env has keys {:?}", self.values.borrow().keys(),)
    }
}
