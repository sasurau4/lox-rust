use super::callable::{LoxCallable, LoxFunction};
use super::error::Result;
use super::interpreter::Interpreter;
use super::lox_instance::LoxInstance;
use super::object::Object;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, LoxFunction>) -> LoxClass {
        LoxClass { name, methods }
    }
    pub fn find_method(&self, name: String) -> Option<&LoxFunction> {
        self.methods.get(&name)
    }
}
impl LoxCallable for LoxClass {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object> {
        Ok(Object::Instance(LoxInstance::new(self.clone())))
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
