use super::token;
use super::token::Token;

pub trait Visitor<T> {
    // fn visit_expr(self, expr: Expr) -> T;
    fn visit_binary(self, expr: Binary) -> T;
    fn visit_grouping(self, expr: Grouping) -> T;
    fn visit_literal(self, expr: Literal) -> T;
    fn visit_unary(self, expr: Unary) -> T;
}

pub trait Acceptor<T> {
    fn accept(self, visitor: impl Visitor<T>) -> T;
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    Binary(Binary<'a>),
    Unary(Unary<'a>),
    Grouping(Grouping<'a>),
    Literal(Literal<'a>),
}

impl<'a, T> Acceptor<T> for Expr<'a> {
    fn accept(self, visitor: impl Visitor<T>) -> T {
        match self {
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Binary<'a> {
    pub left: Box<Expr<'a>>,
    pub operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

impl<'a, T> Acceptor<T> for Binary<'a> {
    fn accept(self, visitor: impl Visitor<T>) -> T {
        visitor.visit_binary(self)
    }
}

#[derive(Debug, Clone)]
pub struct Grouping<'a> {
    pub expression: Box<Expr<'a>>,
}

impl<'a, T> Acceptor<T> for Grouping<'a> {
    fn accept(self, visitor: impl Visitor<T>) -> T {
        visitor.visit_grouping(self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Literal<'a> {
    pub value: token::Literal<'a>,
}

impl<'a, T> Acceptor<T> for Literal<'a> {
    fn accept(self, visitor: impl Visitor<T>) -> T {
        visitor.visit_literal(self)
    }
}

#[derive(Debug, Clone)]
pub struct Unary<'a> {
    pub operator: Token<'a>,
    pub right: Box<Expr<'a>>,
}

impl<'a, T> Acceptor<T> for Unary<'a> {
    fn accept(self, visitor: impl Visitor<T>) -> T {
        visitor.visit_unary(self)
    }
}
