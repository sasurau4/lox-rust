use super::expr::Expr;
use super::token::Token;

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;
    fn visit_print_stmt(&mut self, expression: &Expr) -> T;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> T;
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> T;
}

pub trait Acceptor<T> {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
    Block { statements: Vec<Stmt> },
}

impl<T> Acceptor<T> for Stmt {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, &initializer),
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
        }
    }
}
