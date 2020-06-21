use super::object::Object;
use super::token::Token;
use super::token_type::TokenType;
use log::error;

#[derive(Debug, Clone)]
pub enum Error {
    Return(Object),
    ParseError(String),
    RuntimeError(Token, String),
    ResolveError(Token, String),
}

pub type Result<T> = std::result::Result<T, Error>;

fn report(line: usize, place: &str, message: &str) {
    error!("[line {}] Error {}: {}", line, place, message);
}

pub fn resolve_error(token: Token, message: &str) {
    if token.token_type == TokenType::EOF {
        report(token.line, " at end", message)
    }
    report(token.line, "", message)
}

pub fn parser_error(token: Token, message: &str) {
    if token.token_type == TokenType::EOF {
        report(token.line, " at end", message)
    }
    report(token.line, "", message)
}

pub fn lexer_error(line: usize, message: &str) {
    report(line, "", message)
}
