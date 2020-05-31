use super::error::parser_error;
use super::expr::Expr;
use super::token::{Literal, Token};
use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct ParserError(String);

type ParseResult<T> = Result<T, ParserError>;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParseResult<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> ParseResult<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        while self.contains(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = match self.comparison() {
                Ok(result) => result,
                Err(err) => return Err(err),
            };
            expr = &mut Expr::Binary {
                left: Box::new(expr.clone()),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr.clone())
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        Ok(Expr::Literal {
            value: Literal::String("piyo"),
        })
    }

    // fn comparison(&mut self) -> ParseResult<Expr> {
    //     let mut expr = match self.addition() {
    //         Ok(result) => result,
    //         Err(err) => return Err(err),
    //     };

    //     while self.contains(vec![
    //         TokenType::Greater,
    //         TokenType::GreaterEqual,
    //         TokenType::Less,
    //         TokenType::LessEqual,
    //     ]) {
    //         let operator = self.previous();
    //         let right = match self.addition() {
    //             Ok(result) => result,
    //             Err(err) => return Err(err),
    //         };
    //         expr = Expr::Binary {
    //             left: Box::new(expr),
    //             operator,
    //             right: Box::new(right),
    //         }
    //     }
    //     Ok(expr)
    // }

    // fn addition(&mut self) -> ParseResult<Expr> {
    //     let mut expr = match self.multiplication() {
    //         Ok(multiplication) => multiplication,
    //         Err(err) => return Err(err),
    //     };

    //     while self.contains(vec![TokenType::Minus, TokenType::Plus]) {
    //         let operator = self.previous();
    //         let right = match self.multiplication() {
    //             Ok(multiplication) => multiplication,
    //             Err(err) => return Err(err),
    //         };
    //         expr = Expr::Binary {
    //             left: Box::new(expr),
    //             operator,
    //             right: Box::new(right),
    //         }
    //     }
    //     Ok(expr)
    // }

    // fn multiplication(&mut self) -> ParseResult<Expr> {
    //     let mut expr = match self.unary() {
    //         Ok(unary) => unary,
    //         Err(err) => return Err(err),
    //     };

    //     while self.contains(vec![TokenType::Slash, TokenType::Star]) {
    //         let operator = self.previous();
    //         let right = match self.unary() {
    //             Ok(unary) => unary,
    //             Err(err) => return Err(err),
    //         };
    //         expr = Expr::Binary {
    //             left: Box::new(expr),
    //             operator,
    //             right: Box::new(right),
    //         }
    //     }
    //     Ok(expr)
    // }

    // fn unary(&mut self) -> ParseResult<Expr> {
    //     if self.contains(vec![TokenType::Bang, TokenType::Minus]) {
    //         let operator = self.previous();
    //         let right = match self.unary() {
    //             Ok(unary) => unary,
    //             Err(err) => return Err(err),
    //         };
    //         return Ok(Expr::Unary {
    //             operator,
    //             right: Box::new(right),
    //         });
    //     }
    //     self.primary()
    // }

    // fn primary(&mut self) -> ParseResult<Expr> {
    //     if self.contains(vec![TokenType::False]) {
    //         return Ok(Expr::Literal {
    //             value: Literal::Bool(false),
    //         });
    //     };
    //     if self.contains(vec![TokenType::True]) {
    //         return Ok(Expr::Literal {
    //             value: Literal::Bool(true),
    //         });
    //     }
    //     if self.contains(vec![TokenType::Nil]) {
    //         return Ok(Expr::Literal {
    //             value: Literal::None,
    //         });
    //     }
    //     if self.contains(vec![TokenType::Number, TokenType::String]) {
    //         return Ok(Expr::Literal {
    //             value: self.previous().literal,
    //         });
    //     }

    //     if self.contains(vec![TokenType::LeftParen]) {
    //         let expr = self.expression()?;
    //         self.consume(TokenType::RightParen, "Expect')' after expression.");
    //         return Ok(Expr::Grouping {
    //             expression: Box::new(expr),
    //         });
    //     }

    //     Err(Parser::error(self.peek(), "Expect expression."))
    // }

    fn consume(&mut self, token_type: TokenType, message: &str) -> ParseResult<Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        let token = self.peek();

        Err(Parser::error(token, message))
    }

    fn error(token: Token, message: &str) -> ParserError {
        parser_error(token, &message);
        ParserError(String::from(message))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn contains(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }
}
