use super::token_type::TokenType;

#[derive(Debug, Clone, Copy)]
pub enum Literal<'a> {
    Usize(usize),
    String(&'a str),
    Float(f64),
    None,
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    literal: Literal<'a>,
    line: usize,
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
