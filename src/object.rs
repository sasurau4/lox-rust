use super::token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    Literal(token::Literal),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Literal(l) => write!(f, "{}", l),
        }
    }
}
