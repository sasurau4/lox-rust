use super::token_type::TokenType;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    literal: Option<&'a str>,
    line: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'a str,
        literal: Option<&'a str>,
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

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.lexeme,
            self.literal.unwrap_or("none")
        )
    }
}
