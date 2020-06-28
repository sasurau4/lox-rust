use super::callable::LoxCallable;
use super::error::{Error, Result};
use super::interpreter::Interpreter;
use super::lox_instance::LoxInstance;
use super::object::Object;
use std::fmt;

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
}

impl LoxCallable for LoxClass {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object> {
        Ok(Object::Instance(LoxInstance {
            class: self.clone(),
        }))
    }
    fn arity(&self) -> usize {
        0
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
