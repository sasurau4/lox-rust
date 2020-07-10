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
    super_class: Option<Box<LoxClass>>,
    methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(
        name: String,
        super_class: Option<Box<LoxClass>>,
        methods: HashMap<String, LoxFunction>,
    ) -> LoxClass {
        LoxClass {
            name,
            super_class,
            methods,
        }
    }
    pub fn find_method(&self, name: String) -> Option<&LoxFunction> {
        if let Some(m) = self.methods.get(&name) {
            return Some(m);
        }
        let LoxClass { super_class, .. } = self;
        if let Some(ext_super_class) = super_class {
            return ext_super_class.find_method(name);
        }
        None
    }
}
impl LoxCallable for LoxClass {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object> {
        let instance = LoxInstance::new(self.clone());
        if let Some(initializer) = self.find_method("init".to_string()) {
            initializer
                .bind(instance.clone())
                .call(interpreter, arguments)?;
        }
        Ok(Object::Instance(instance))
    }
    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method("init".to_string()) {
            initializer.arity()
        } else {
            0
        }
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ClassType {
    None,
    Class,
}
