use super::environment::Environment;
use super::error::{Error, Result};
use super::expr;
use super::expr::{Acceptor as ExprAcceptor, Expr};
use super::object::Object;
use super::stmt;
use super::stmt::{Acceptor as StmtAcceptor, Stmt};
use super::token::{Literal, Token};
use super::token_type::TokenType;
use log::error;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Interpreter {
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new(environment: Environment) -> Interpreter {
        Interpreter {
            environment: Rc::new(environment),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<()> {
        for statement in statements {
            match self.execute(&statement) {
                Ok(_) => {}
                Err(r) => error!("{:?}", r),
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<()> {
        stmt.accept(self)
    }

    fn is_truthy(&self, object: Object) -> bool {
        match object {
            Object::Literal(literal) => match literal {
                Literal::None => false,
                Literal::Bool(b) => b,
                _ => true,
            },
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

    fn execute_block(&mut self, statements: &[Stmt], environment: Environment) -> Result<()> {
        let previous = self.environment.clone();
        self.environment = Rc::new(environment);
        for statement in statements {
            match self.execute(statement) {
                Ok(_) => {}
                Err(e) => {
                    self.environment = previous;
                    return Err(e);
                }
            };
        }
        self.environment = previous;
        Ok(())
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
            (Bang, Object::Literal(lit)) => {
                Ok(Object::Literal(Bool(!self.is_truthy(Object::Literal(lit)))))
            }
            _ => Ok(Object::Literal(None)),
        }
    }

    fn visit_variable(&mut self, name: &Token) -> Result<Object> {
        self.environment.get(name)
    }

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object> {
        use super::token::Literal::{Bool, Float, Isize, None, String as LString};
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
                    (LString(l), LString(r)) => Ok(Object::Literal(LString(format!("{}{}", l, r)))),
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

    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Result<Object> {
        let evaluated_left = self.evaluate(left);
        let is_left_truthy = self.is_truthy(evaluated_left.clone()?);

        if operator.token_type == TokenType::Or {
            if is_left_truthy {
                return evaluated_left;
            }
        } else if !is_left_truthy {
            return evaluated_left;
        }
        self.evaluate(right)
    }

    fn visit_assign(&mut self, name: &Token, value: &Expr) -> Result<Object> {
        let value = self.evaluate(value)?;
        self.environment.assign(name, &value)?;
        Ok(value)
    }
}

impl stmt::Visitor<Result<()>> for Interpreter {
    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<()> {
        let result = self.evaluate(expression)?;
        if self.environment.is_repl {
            println!("{}", result);
        }
        Ok(())
    }
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> Result<()> {
        let evaluated = self.evaluate(condition)?;
        if self.is_truthy(evaluated) {
            self.execute(then_branch)?
        }
        match else_branch {
            Some(eb) => self.execute(&*eb)?,
            None => {}
        }
        Ok(())
    }
    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<()> {
        let value = self.evaluate(expression)?;
        println!("{}", value);
        Ok(())
    }
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<()> {
        let value = self.evaluate(initializer)?;
        self.environment.define(name.lexeme.clone(), &value);
        Ok(())
    }
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> Result<()> {
        self.execute_block(
            statements,
            Environment::new(Some(Rc::clone(&self.environment)), self.environment.is_repl),
        )?;
        Ok(())
    }
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<()> {
        loop {
            let evaluated_condition = self.evaluate(condition)?;
            if self.is_truthy(evaluated_condition) {
                self.execute(body)?
            } else {
                break;
            }
        }
        Ok(())
    }
}
