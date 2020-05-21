use super::token::Token;
use super::token_type::TokenType;
use super::token_type::TokenType::*;

pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &str) -> Lexer {
        Lexer {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn tokenizeAll(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.tokenize()
        }
        self.tokens.push(Token::new(EOF, "", None, self.line));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn tokenize(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_without_literal(LeftParen),
            _ => self.add_token_without_literal(EOF),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        println!("current: {}", self.current);
        println!("source: {:#?}", self.source.chars().collect::<Vec<char>>());
        self.source.chars().collect::<Vec<char>>()[self.current - 1]
    }

    fn add_token_without_literal(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<&'a str>) {
        let text = self.source.get(self.start..self.current).unwrap();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }
}
