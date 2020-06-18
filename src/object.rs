use super::callable;
use super::token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    Literal(token::Literal),
    Callable(callable::LoxFunction),
    Clock(callable::Clock),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Literal(l) => write!(f, "{}", l),
            Object::Callable(l) => write!(f, "{}", l),
            Object::Clock(l) => write!(f, "{}", l),
        }
    }
}
