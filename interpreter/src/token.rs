use super::token_type::TokenType;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Literal {
    Isize(isize),
    String(String),
    Float(f64),
    Bool(bool),
    None,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Isize(i) => write!(f, "{}", i),
            Literal::String(string) => write!(f, "{}", string),
            Literal::Float(float) => write!(f, "{}", float),
            Literal::Bool(boolean) => write!(f, "{}", boolean),
            Literal::None => write!(f, "{}", &"nil"),
        }
    }
}

impl Eq for Literal {}

impl PartialEq for Literal {
    fn eq(&self, other: &Literal) -> bool {
        match (self, other) {
            (Literal::Bool(a), Literal::Bool(b)) => a.eq(b),
            (Literal::Isize(a), Literal::Isize(b)) => a.eq(b),
            (Literal::String(a), Literal::String(b)) => a.eq(b),
            (Literal::Float(a), Literal::Float(b)) => a.eq(b),
            (Literal::None, Literal::None) => true,
            (_, _) => false,
        }
    }
}

impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Literal::Isize(i) => i.hash(state),
            Literal::String(s) => s.hash(state),
            Literal::Float(f) => f.to_bits().hash(state),
            Literal::Bool(b) => b.hash(state),
            Literal::None => "".hash(state),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
