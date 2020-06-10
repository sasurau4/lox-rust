use super::token;
use super::token::Token;

pub trait Visitor<T> {
    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping(&mut self, expression: &Expr) -> T;
    fn visit_literal(&mut self, expr: &token::Literal) -> T;
    fn visit_logical(&mut self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> T;
    fn visit_variable(&mut self, name: &Token) -> T;
    fn visit_assign(&mut self, name: &Token, value: &Expr) -> T;
}

pub trait Acceptor<T> {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
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
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Assign {
        name: Token,
        value: Box<Expr>,
    },
}

impl<T> Acceptor<T> for Expr {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary(left, operator, right),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical(left, operator, right),
            Expr::Variable { name } => visitor.visit_variable(name),
            Expr::Assign { name, value } => visitor.visit_assign(name, value),
        }
    }
}
