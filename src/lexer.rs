use super::error::error;
use super::token::{Literal, Token};
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

    pub fn tokenize_all(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.tokenize()
        }
        self.tokens
            .push(Token::new(EOF, "", Literal::None, self.line));
        self.tokens.clone()
    }

    fn tokenize(&mut self) {
        let c = self.advance();
        match c {
            // Single Lexemes
            '(' => self.add_token_without_literal(TokenType::LeftParen),
            ')' => self.add_token_without_literal(TokenType::RightParen),
            '{' => self.add_token_without_literal(TokenType::LeftBrace),
            '}' => self.add_token_without_literal(TokenType::RightBrace),
            ',' => self.add_token_without_literal(TokenType::Comma),
            '.' => self.add_token_without_literal(TokenType::Dot),
            '-' => self.add_token_without_literal(TokenType::Minus),
            '+' => self.add_token_without_literal(TokenType::Plus),
            ';' => self.add_token_without_literal(TokenType::Semicolon),
            '*' => self.add_token_without_literal(TokenType::Star),
            // Multi or single lexemes
            '!' => {
                let next_token = if self.match_to_expected('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token_without_literal(next_token)
            }
            '=' => {
                let next_token = if self.match_to_expected('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token_without_literal(next_token)
            }
            '<' => {
                let next_token = if self.match_to_expected('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token_without_literal(next_token)
            }
            '>' => {
                let next_token = if self.match_to_expected('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token_without_literal(next_token)
            }
            // slash
            '/' => {
                if self.match_to_expected('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_without_literal(TokenType::Slash);
                }
            }

            // Ignore whitespaces
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(),
            '0'..='9' => self.number(),
            _ => error(self.line, "Unexpected character."),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token_without_literal(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, Literal::None)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal<'a>) {
        let text = self.source.get(self.start..self.current).unwrap();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }

    fn match_to_expected(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        };
        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\n';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        // Unterminated string.
        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return;
        }

        // The closing ".
        self.advance();

        let value = self.source.get(self.start + 1..self.current - 1).unwrap();
        self.add_token_with_literal(TokenType::String, Literal::String(value));
    }

    fn number(&mut self) {
        while ('0'..='9').contains(&self.peek()) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && ('0'..='9').contains(&self.peek_next()) {
            // Consume the "."
            self.advance();

            while ('0'..='9').contains(&self.peek()) {
                self.advance();
            }
        }

        let target_literal = self.source.get(self.start..self.current).unwrap();
        let parsed_usize = target_literal.parse::<usize>();
        let parsed_float = target_literal.parse::<f64>();

        let literal = if parsed_usize.is_ok() {
            Literal::Usize(parsed_usize.ok().unwrap())
        } else if parsed_float.is_ok() {
            Literal::Float(parsed_float.ok().unwrap())
        } else {
            error(self.line, "Unexpected character.");
            panic!("")
        };
        self.add_token_with_literal(TokenType::Number, literal)
    }
}
