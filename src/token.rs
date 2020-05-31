use super::token_type::TokenType;
// use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Literal<'a> {
    Usize(usize),
    String(&'a str),
    Float(f64),
    Bool(bool),
    None,
}

// impl<'a> fmt::Display for Literal<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", "p")
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: Literal<'a>,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'a str,
        literal: Literal<'a>,
        line: usize,
    ) -> Token<'a> {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
