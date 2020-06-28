use super::callable;
use super::lox_class;
use super::lox_instance;
use super::token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    Literal(token::Literal),
    Func(callable::LoxFunction),
    Clock(callable::Clock),
    Class(lox_class::LoxClass),
    Instance(lox_instance::LoxInstance),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Literal(l) => write!(f, "{}", l),
            Object::Func(l) => write!(f, "{}", l),
            Object::Clock(l) => write!(f, "{}", l),
            Object::Class(l) => write!(f, "{}", l),
            Object::Instance(l) => write!(f, "{}", l),
        }
    }
}
