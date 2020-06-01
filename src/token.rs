use super::token_type::TokenType;
// use std::fmt;

#[derive(Debug, Clone)]
pub enum Literal {
    Isize(isize),
    String(String),
    Float(f64),
    Bool(bool),
    None,
}

// impl<'a> fmt::Display for Literal<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", "p")
//     }
// }

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
