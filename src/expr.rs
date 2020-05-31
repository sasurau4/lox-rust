use super::token;
use super::token::Token;

pub trait Visitor<'a, T> {
    fn visit_binary(self, left: Expr, operator: Token, right: Expr) -> T;
    fn visit_grouping(self, expr: Expr) -> T;
    fn visit_literal(self, expr: token::Literal) -> T;
    fn visit_unary(self, operator: Token, right: Expr) -> T;
}

pub trait Acceptor<'a, T> {
    fn accept(self, visitor: impl Visitor<'a, T>) -> T;
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    Literal {
        value: token::Literal<'a>,
    },
}

impl<'a, T> Acceptor<'a, T> for Expr<'a> {
    fn accept(self, visitor: impl Visitor<'a, T>) -> T {
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
