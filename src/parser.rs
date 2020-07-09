use super::error::parser_error;
use super::expr::Expr;
use super::stmt::Stmt;
use super::token::{Literal, Token};
use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct ParserError(String);

type ParseResult<T> = Result<T, ParserError>;

#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut statements = vec![];
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(statements)
    }

    fn expression(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    fn declaration(&mut self) -> ParseResult<Stmt> {
        let result = if self.contains(&[TokenType::Var]) {
            self.var_declaration()
        } else if self.contains(&[TokenType::Class]) {
            self.class_declaration()
        } else if self.contains(&[TokenType::Fun]) {
            self.function(String::from("function"))
        } else {
            self.statement()
        };

        match result {
            Ok(r) => Ok(r),
            Err(e) => {
                self.synchronize();
                Err(e)
            }
        }
    }

    fn class_declaration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect class name.")?;
        let super_class = if self.contains(&[TokenType::Less]) {
            self.consume(TokenType::Identifier, "Expect superclass name.")?;
            Some(Expr::Variable {
                name: self.previous().clone(),
            })
        } else {
            None
        };
        self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;
        let mut methods = vec![];
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let function = self.function(String::from("method"))?;
            methods.push(function);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;

        Ok(Stmt::Class {
            name,
            super_class,
            methods,
        })
    }

    fn statement(&mut self) -> ParseResult<Stmt> {
        if self.contains(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.contains(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.contains(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.contains(&[TokenType::Return]) {
            return self.return_statemet();
        }
        if self.contains(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.contains(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block {
                statements: self.block()?,
            });
        }
        self.expression_statement()
    }

    fn for_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.contains(&[TokenType::Semicolon]) {
            None
        } else if self.contains(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal {
                value: Literal::Bool(true),
            }
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;
        if increment.is_some() {
            body = Stmt::Block {
                statements: vec![
                    body,
                    Stmt::Expression {
                        expression: increment.unwrap(),
                    },
                ],
            }
        }
        body = Stmt::While {
            condition,
            body: Box::new(body),
        };
        if initializer.is_some() {
            body = Stmt::Block {
                statements: vec![initializer.unwrap(), body],
            }
        }
        Ok(body)
    }

    fn if_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.contains(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn print_statement(&mut self) -> ParseResult<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ':' after value.")?;
        Ok(Stmt::Print { expression: value })
    }

    fn return_statemet(&mut self) -> ParseResult<Stmt> {
        let keyword = self.previous().clone();
        let value = if !self.check(TokenType::Semicolon) {
            self.expression()?
        } else {
            Expr::Literal {
                value: Literal::None,
            }
        };
        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return { keyword, value })
    }

    fn var_declaration(&mut self) -> ParseResult<Stmt> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.contains(&[TokenType::Equal]) {
            self.expression()?
        } else {
            Expr::Literal {
                value: Literal::None,
            }
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var { name, initializer })
    }

    fn while_statement(&mut self) -> ParseResult<Stmt> {
        self.consume(TokenType::LeftParen, "Expectct '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expectct ')' after condition.")?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition, body })
    }

    fn expression_statement(&mut self) -> ParseResult<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ':' after value.")?;
        Ok(Stmt::Expression { expression: expr })
    }

    fn function(&mut self, kind: String) -> ParseResult<Stmt> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {} name.", kind))?;
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {} name.", kind),
        )?;
        let mut parameters = vec![];
        loop {
            if !self.check(TokenType::RightParen) {
                if parameters.len() >= 255 {
                    return Err(Parser::error(
                        self.peek().clone(),
                        "Cannot have more than 255 parameters.",
                    ));
                }
                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?)
            }
            if !self.contains(&[TokenType::Comma]) {
                break;
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;
        let body = self.block()?;
        Ok(Stmt::Function {
            name,
            params: parameters,
            body,
        })
    }

    fn block(&mut self) -> ParseResult<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?)
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.or()?;

        if self.contains(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            return match expr {
                Expr::Variable { name } => Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                }),
                Expr::Get { object, name } => Ok(Expr::Set {
                    object,
                    name,
                    value: Box::new(value),
                }),
                _ => Err(Parser::error(equals, "Invalid assignment target.")),
            };
        }
        Ok(expr)
    }

    fn or(&mut self) -> ParseResult<Expr> {
        let mut expr = self.and()?;
        while self.contains(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn and(&mut self) -> ParseResult<Expr> {
        let mut expr = self.equality()?;
        while self.contains(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;
        while self.contains(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = match self.addition() {
            Ok(result) => result,
            Err(err) => return Err(err),
        };

        while self.contains(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = match self.addition() {
                Ok(result) => result,
                Err(err) => return Err(err),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn addition(&mut self) -> ParseResult<Expr> {
        let mut expr = match self.multiplication() {
            Ok(multiplication) => multiplication,
            Err(err) => return Err(err),
        };

        while self.contains(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = match self.multiplication() {
                Ok(multiplication) => multiplication,
                Err(err) => return Err(err),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn multiplication(&mut self) -> ParseResult<Expr> {
        let mut expr = match self.unary() {
            Ok(unary) => unary,
            Err(err) => return Err(err),
        };

        while self.contains(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = match self.unary() {
                Ok(unary) => unary,
                Err(err) => return Err(err),
            };
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        if self.contains(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = match self.unary() {
                Ok(unary) => unary,
                Err(err) => return Err(err),
            };
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.call()
    }

    fn finish_call(&mut self, callee: Expr) -> ParseResult<Expr> {
        let mut arguments = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    Parser::error(self.peek().clone(), "Cannot have more than 255 arguments.");
                }
                arguments.push(self.expression()?);
                if !self.contains(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        Ok(Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn call(&mut self) -> ParseResult<Expr> {
        let mut expr = self.primary()?;
        loop {
            if self.contains(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.contains(&[TokenType::Dot]) {
                let name =
                    self.consume(TokenType::Identifier, "Expect property name after '.'.")?;
                expr = Expr::Get {
                    object: Box::new(expr),
                    name,
                }
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        if self.contains(&[TokenType::False]) {
            return Ok(Expr::Literal {
                value: Literal::Bool(false),
            });
        };
        if self.contains(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: Literal::Bool(true),
            });
        }
        if self.contains(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: Literal::None,
            });
        }
        if self.contains(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal {
                value: self.previous().literal.clone(),
            });
        }
        if self.contains(&[TokenType::This]) {
            return Ok(Expr::This {
                keyword: self.previous().clone(),
            });
        }
        if self.contains(&[TokenType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous().clone(),
            });
        }

        if self.contains(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect')' after expression.")?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(Parser::error(self.peek().clone(), "Expect expression."))
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> ParseResult<Token> {
        if self.check(token_type) {
            return Ok(self.advance().clone());
        }
        let token = self.peek();

        Err(Parser::error(token.clone(), message))
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

    // equal to match function on Java implementation
    fn contains(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
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

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
