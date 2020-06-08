use super::environment::Environment;
use super::error::{Error, Result};
use super::expr;
use super::expr::{Acceptor as ExprAcceptor, Expr};
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

    fn evaluate(&mut self, expr: &Expr) -> Result<Literal> {
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

    fn is_equal(&self, a: Literal, b: Literal) -> bool {
        match (a, b) {
            (Literal::None, Literal::None) => true,
            (Literal::Bool(a), Literal::Bool(b)) => a == b,
            (Literal::String(a), Literal::String(b)) => a == b,
            (Literal::Isize(a), Literal::Isize(b)) => a == b,
            (Literal::Float(a), Literal::Float(b)) => a == b,
            _ => false,
        }
    }
}

impl expr::Visitor<Result<Literal>> for Interpreter {
    fn visit_grouping(&mut self, expr: &Expr) -> Result<Literal> {
        self.evaluate(expr)
        // match expr {
        //     Expr::Grouping { mut expression } => self.evaluate(&mut *expression),
        //     _ => unreachable!(),
        // }
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Result<Literal> {
        let right = self.evaluate(right)?;
        match operator.token_type {
            TokenType::Minus => match right {
                Literal::Isize(r) => Ok(Literal::Isize(-r)),
                Literal::Float(r) => Ok(Literal::Float(-r)),
                _ => Err(Error::RuntimeError(
                    operator.clone(),
                    String::from("Operand must be a number."),
                )),
            },
            TokenType::Bang => Ok(Literal::Bool(!self.is_truthy(right))),
            _ => Ok(Literal::None),
        }
    }

    fn visit_variable(&mut self, name: &Token) -> Result<Literal> {
        let env = self.environment.get(name)?;

        match env {
            Expr::Literal { value } => Ok(value),
            _ => unreachable!(),
        }
    }

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Literal> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Greater => match (left, right) {
                (Literal::Isize(l), Literal::Isize(r)) => Ok(Literal::Bool(l > r)),
                (Literal::Isize(l), Literal::Float(r)) => Ok(Literal::Bool(l as f64 > r)),
                (Literal::Float(l), Literal::Isize(r)) => Ok(Literal::Bool(l > r as f64)),
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Bool(l > r)),
                _ => Err(Error::RuntimeError(
                    operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Literal::Isize(l), Literal::Isize(r)) => Ok(Literal::Bool(l >= r)),
                (Literal::Isize(l), Literal::Float(r)) => Ok(Literal::Bool(l as f64 >= r)),
                (Literal::Float(l), Literal::Isize(r)) => Ok(Literal::Bool(l >= r as f64)),
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Bool(l >= r)),
                _ => Err(Error::RuntimeError(
                    operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::Less => match (left, right) {
                (Literal::Isize(l), Literal::Isize(r)) => Ok(Literal::Bool(l < r)),
                (Literal::Isize(l), Literal::Float(r)) => Ok(Literal::Bool((l as f64) < r)),
                (Literal::Float(l), Literal::Isize(r)) => Ok(Literal::Bool(l < r as f64)),
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Bool(l < r)),
                _ => Err(Error::RuntimeError(
                    operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Literal::Isize(l), Literal::Isize(r)) => Ok(Literal::Bool(l <= r)),
                (Literal::Isize(l), Literal::Float(r)) => Ok(Literal::Bool((l as f64) <= r)),
                (Literal::Float(l), Literal::Isize(r)) => Ok(Literal::Bool(l <= r as f64)),
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Bool(l <= r)),
                _ => Err(Error::RuntimeError(
                    operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::Minus => match (left, right) {
                (Literal::Isize(l), Literal::Isize(r)) => Ok(Literal::Isize(l - r)),
                (Literal::Isize(l), Literal::Float(r)) => Ok(Literal::Float(l as f64 - r)),
                (Literal::Float(l), Literal::Isize(r)) => Ok(Literal::Float(l - r as f64)),
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Float(l - r)),
                _ => Err(Error::RuntimeError(
                    operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::Plus => match (left, right) {
                (Literal::Isize(l), Literal::Isize(r)) => Ok(Literal::Isize(l + r)),
                (Literal::Isize(l), Literal::Float(r)) => Ok(Literal::Float(l as f64 + r)),
                (Literal::Float(l), Literal::Isize(r)) => Ok(Literal::Float(l + r as f64)),
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Float(l + r)),
                (Literal::String(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r)))
                }
                _ => Err(Error::RuntimeError(
                    operator.clone(),
                    String::from("Operands must be two numbers or two strings."),
                )),
            },
            TokenType::Slash => match (left, right) {
                (Literal::Isize(l), Literal::Isize(r)) => Ok(Literal::Isize(l / r)),
                (Literal::Isize(l), Literal::Float(r)) => Ok(Literal::Float(l as f64 / r)),
                (Literal::Float(l), Literal::Isize(r)) => Ok(Literal::Float(l / r as f64)),
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Float(l / r)),
                _ => Err(Error::RuntimeError(
                    operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::Star => match (left, right) {
                (Literal::Isize(l), Literal::Isize(r)) => Ok(Literal::Isize(l * r)),
                (Literal::Isize(l), Literal::Float(r)) => Ok(Literal::Float(l as f64 * r)),
                (Literal::Float(l), Literal::Isize(r)) => Ok(Literal::Float(l * r as f64)),
                (Literal::Float(l), Literal::Float(r)) => Ok(Literal::Float(l * r)),
                _ => Err(Error::RuntimeError(
                    operator.clone(),
                    String::from("Operands must be numbers."),
                )),
            },
            TokenType::BangEqual => Ok(Literal::Bool(!self.is_equal(left, right))),
            TokenType::EqualEqual => Ok(Literal::Bool(self.is_equal(left, right))),
            _ => Ok(Literal::None),
        }
    }

    fn visit_literal(&mut self, expr: &Literal) -> Result<Literal> {
        Ok(expr.clone())
    }
}

impl stmt::Visitor<Result<()>> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Expression { expression } => self.evaluate(expression),
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
        self.environment
            .define(name.lexeme.clone(), &Expr::Literal { value });
        Ok(())
    }
}
