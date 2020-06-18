use super::environment::Environment;
use super::error::Result;
use super::interpreter::Interpreter;
use super::stmt::Stmt;
use super::token::{Literal, Token};
use std::fmt;
use std::time::SystemTime;

use super::object::Object;

pub trait LoxCallable {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object>;
    fn arity(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct LoxFunction {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
}

impl LoxFunction {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> LoxFunction {
        LoxFunction { name, params, body }
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Result<Object> {
        let environement = Environment::new(
            Some(interpreter.globals.clone()),
            interpreter.globals.is_repl,
        );
        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            environement.define(param.lexeme.clone(), arg)
        }
        interpreter.execute_block(&self.body, environement)?;
        Ok(Object::Literal(Literal::None))
    }
    fn arity(&self) -> usize {
        self.params.len()
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme)
    }
}

#[derive(Debug, Clone)]
pub struct Clock {}

impl LoxCallable for Clock {
    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Object>) -> Result<Object> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Ok(Object::Literal(Literal::Isize(n.as_secs() as isize))),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }
    fn arity(&self) -> usize {
        0
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<native fn>")
    }
}
