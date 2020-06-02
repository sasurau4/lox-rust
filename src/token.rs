use super::token_type::TokenType;
use std::fmt;

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

#[derive(Debug, Clone)]
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
