use super::token;
use super::token::Token;

pub trait Visitor<T> {
    fn visit_binary(self, left: Expr, operator: Token, right: Expr) -> T;
    fn visit_grouping(self, expr: Expr) -> T;
    fn visit_literal(self, expr: token::Literal) -> T;
    fn visit_unary(self, operator: Token, right: Expr) -> T;
}

pub trait Acceptor<T> {
    fn accept(self, visitor: impl Visitor<T>) -> T;
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: token::Literal,
    },
}

impl<T> Acceptor<T> for Expr {
    fn accept(self, visitor: impl Visitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(*left, operator, *right),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, *right),
            Expr::Grouping { expression } => visitor.visit_grouping(Expr::Grouping { expression }),
            Expr::Literal { value } => visitor.visit_literal(value),
        }
    }
}
