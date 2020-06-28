use super::lox_class::LoxClass;
use std::fmt;

#[derive(Debug, Clone)]
pub struct LoxInstance {
    pub class: LoxClass,
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} instance", self.class.name)
    }
}
