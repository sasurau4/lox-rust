use super::expr::{Acceptor, Expr, Visitor};
use super::token::{Literal, Token};
use super::token_type::TokenType;

#[derive(Debug, Clone, Copy)]
pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(self, expr: Expr) -> Literal {
        self.evaluate(expr)
    }
    fn evaluate(self, expr: Expr) -> Literal {
        expr.accept(self)
    }

    fn is_truthy(self, literal: Literal) -> bool {
        match literal {
            Literal::None => false,
            Literal::Bool(b) => b,
            _ => true,
        }
    }
}

impl Visitor<Literal> for Interpreter {
    fn visit_grouping(self, expr: Expr) -> Literal {
        match expr {
            Expr::Grouping { expression } => self.evaluate(*expression),
            _ => unreachable!(),
        }
    }

    fn visit_unary(self, operator: Token, right: Expr) -> Literal {
        let right = self.evaluate(right);
        match operator.token_type {
            TokenType::Minus => Literal::Float(-1.4),
            TokenType::Bang => Literal::Bool(!self.is_truthy(right)),
            _ => Literal::None,
        }
    }

    fn visit_binary(self, left: Expr, operator: Token, right: Expr) -> Literal {
        let left = self.evaluate(left);
        let right = self.evaluate(right);

        match operator.token_type {
            TokenType::Minus => match (left, right) {
                (Literal::Isize(l), Literal::Isize(r)) => Literal::Isize(l - r),
                _ => unreachable!(),
            },
            TokenType::Plus => match (left, right) {
                (Literal::Isize(l), Literal::Isize(r)) => Literal::Isize(l + r),
                (Literal::String(l), Literal::String(r)) => Literal::String(format!("{}{}", l, r)),
                _ => unreachable!(),
            },
            TokenType::Slash => match (left, right) {
                (Literal::Isize(l), Literal::Isize(r)) => Literal::Float(l as f64 / r as f64),
                (Literal::Float(l), Literal::Float(r)) => Literal::Float(l / r),
                _ => unreachable!(),
            },
            TokenType::Star => match (left, right) {
                (Literal::Isize(l), Literal::Isize(r)) => Literal::Isize(l * r),
                (Literal::Float(l), Literal::Float(r)) => Literal::Float(l / r),
                _ => unreachable!(),
            },
            _ => Literal::None,
        }
    }

    fn visit_literal(self, expr: Literal) -> Literal {
        expr
    }
}
