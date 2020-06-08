use super::environment::Environment;
use super::error::{Error, Result};
use super::expr;
use super::expr::{Acceptor as ExprAcceptor, Expr};
use super::object::Object;
use super::stmt;
use super::stmt::{Acceptor as StmtAcceptor, Stmt};
use super::token::{Literal, Token};
use super::token_type::TokenType;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Interpreter {
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Rc::new(Environment::new()),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<()> {
        for mut statement in statements {
            match self.execute(&mut statement) {
                Ok(_) => {}
                Err(r) => println!("{:?}", r),
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &mut Stmt) -> Result<()> {
        stmt.accept(self)
    }

    fn is_truthy(&mut self, literal: Literal) -> bool {
        match literal {
            Literal::None => false,
            Literal::Bool(b) => b,
            _ => true,
        }
    }

    fn is_equal(&self, a: Object, b: Object) -> bool {
        match (a, b) {
            (Object::Literal(ola), Object::Literal(olb)) => match (ola, olb) {
                (Literal::None, Literal::None) => true,
                (Literal::Bool(a), Literal::Bool(b)) => a == b,
                (Literal::String(a), Literal::String(b)) => a == b,
                (Literal::Isize(a), Literal::Isize(b)) => a == b,
                (Literal::Float(a), Literal::Float(b)) => a == b,
                _ => false,
            },
        }
    }
}

impl expr::Visitor<Result<Object>> for Interpreter {
    fn visit_grouping(&mut self, expr: &Expr) -> Result<Object> {
        self.evaluate(expr)
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<Object> {
        use super::token::Literal::{Bool, Float, Isize, None};
        use super::token_type::TokenType::{Bang, Minus};

        let right = self.evaluate(right)?;
        match (operator.token_type, right) {
            (Minus, Object::Literal(lit)) => match lit {
                Isize(r) => Ok(Object::Literal(Isize(-r))),
                Float(r) => Ok(Object::Literal(Float(-r))),
                _ => Err(Error::RuntimeError(
                    operator.clone(),
                    String::from("Operand must be a number."),
                )),
            },
            (Bang, Object::Literal(lit)) => Ok(Object::Literal(Bool(!self.is_truthy(lit)))),
            _ => Ok(Object::Literal(None)),
        }
    }

    fn visit_variable(&mut self, name: &Token) -> Result<Object> {
        self.environment.get(name)
    }

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object> {
        use super::token::Literal::{Bool, Float, Isize, None};
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Greater => match (left, right) {
                (Object::Literal(oll), Object::Literal(olr)) => match (oll, olr) {
                    (Isize(l), Isize(r)) => Ok(Object::Literal(Bool(l > r))),
                    (Isize(l), Float(r)) => Ok(Object::Literal(Bool(l as f64 > r))),
                    (Float(l), Isize(r)) => Ok(Object::Literal(Bool(l > r as f64))),
                    (Float(l), Float(r)) => Ok(Object::Literal(Bool(l > r))),
                    _ => Err(Error::RuntimeError(
                        operator.clone(),
                        String::from("Operands must be numbers."),
                    )),
                },
            },
            TokenType::GreaterEqual => match (left, right) {
                (Object::Literal(oll), Object::Literal(olr)) => match (oll, olr) {
                    (Isize(l), Isize(r)) => Ok(Object::Literal(Bool(l >= r))),
                    (Isize(l), Float(r)) => Ok(Object::Literal(Bool(l as f64 >= r))),
                    (Float(l), Isize(r)) => Ok(Object::Literal(Bool(l >= r as f64))),
                    (Float(l), Float(r)) => Ok(Object::Literal(Bool(l >= r))),
                    _ => Err(Error::RuntimeError(
                        operator.clone(),
                        String::from("Operands must be numbers."),
                    )),
                },
            },
            TokenType::Less => match (left, right) {
                (Object::Literal(oll), Object::Literal(olr)) => match (oll, olr) {
                    (Isize(l), Isize(r)) => Ok(Object::Literal(Bool(l < r))),
                    (Isize(l), Float(r)) => Ok(Object::Literal(Bool((l as f64) < r))),
                    (Float(l), Isize(r)) => Ok(Object::Literal(Bool(l < r as f64))),
                    (Float(l), Float(r)) => Ok(Object::Literal(Bool(l < r))),
                    _ => Err(Error::RuntimeError(
                        operator.clone(),
                        String::from("Operands must be numbers."),
                    )),
                },
            },
            TokenType::LessEqual => match (left, right) {
                (Object::Literal(oll), Object::Literal(olr)) => match (oll, olr) {
                    (Isize(l), Isize(r)) => Ok(Object::Literal(Bool(l <= r))),
                    (Isize(l), Float(r)) => Ok(Object::Literal(Bool((l as f64) <= r))),
                    (Float(l), Isize(r)) => Ok(Object::Literal(Bool(l <= r as f64))),
                    (Float(l), Float(r)) => Ok(Object::Literal(Bool(l <= r))),
                    _ => Err(Error::RuntimeError(
                        operator.clone(),
                        String::from("Operands must be numbers."),
                    )),
                },
            },
            TokenType::Minus => match (left, right) {
                (Object::Literal(oll), Object::Literal(olr)) => match (oll, olr) {
                    (Isize(l), Isize(r)) => Ok(Object::Literal(Isize(l - r))),
                    (Isize(l), Float(r)) => Ok(Object::Literal(Float((l as f64) - r))),
                    (Float(l), Isize(r)) => Ok(Object::Literal(Float(l - r as f64))),
                    (Float(l), Float(r)) => Ok(Object::Literal(Float(l - r))),
                    _ => Err(Error::RuntimeError(
                        operator.clone(),
                        String::from("Operands must be numbers."),
                    )),
                },
            },
            TokenType::Plus => match (left, right) {
                (Object::Literal(oll), Object::Literal(olr)) => match (oll, olr) {
                    (Isize(l), Isize(r)) => Ok(Object::Literal(Isize(l + r))),
                    (Isize(l), Float(r)) => Ok(Object::Literal(Float((l as f64) + r))),
                    (Float(l), Isize(r)) => Ok(Object::Literal(Float(l + r as f64))),
                    (Float(l), Float(r)) => Ok(Object::Literal(Float(l + r))),
                    _ => Err(Error::RuntimeError(
                        operator.clone(),
                        String::from("Operands must be numbers."),
                    )),
                },
            },
            TokenType::Slash => match (left, right) {
                (Object::Literal(oll), Object::Literal(olr)) => match (oll, olr) {
                    (Isize(l), Isize(r)) => Ok(Object::Literal(Isize(l / r))),
                    (Isize(l), Float(r)) => Ok(Object::Literal(Float((l as f64) / r))),
                    (Float(l), Isize(r)) => Ok(Object::Literal(Float(l / r as f64))),
                    (Float(l), Float(r)) => Ok(Object::Literal(Float(l / r))),
                    _ => Err(Error::RuntimeError(
                        operator.clone(),
                        String::from("Operands must be numbers."),
                    )),
                },
            },
            TokenType::Star => match (left, right) {
                (Object::Literal(oll), Object::Literal(olr)) => match (oll, olr) {
                    (Isize(l), Isize(r)) => Ok(Object::Literal(Isize(l * r))),
                    (Isize(l), Float(r)) => Ok(Object::Literal(Float((l as f64) * r))),
                    (Float(l), Isize(r)) => Ok(Object::Literal(Float(l * r as f64))),
                    (Float(l), Float(r)) => Ok(Object::Literal(Float(l * r))),
                    _ => Err(Error::RuntimeError(
                        operator.clone(),
                        String::from("Operands must be numbers."),
                    )),
                },
            },
            TokenType::BangEqual => Ok(Object::Literal(Bool(!self.is_equal(left, right)))),
            TokenType::EqualEqual => Ok(Object::Literal(Bool(self.is_equal(left, right)))),
            _ => Ok(Object::Literal(None)),
        }
    }

    fn visit_literal(&mut self, expr: &Literal) -> Result<Object> {
        Ok(Object::Literal(expr.clone()))
    }
}

impl stmt::Visitor<Result<()>> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Expression { expression } => self.evaluate(expression)?,
            _ => unreachable!(),
        };
        Ok(())
    }
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        let value = match stmt {
            Stmt::Print { expression } => self.evaluate(expression),
            _ => unreachable!(),
        };
        match value {
            Ok(r) => {
                println!("{}", r);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<()> {
        let value = self.evaluate(initializer)?;
        self.environment.define(name.lexeme.clone(), &value);
        Ok(())
    }
}
