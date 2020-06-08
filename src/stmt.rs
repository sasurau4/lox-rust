use super::expr::Expr;
use super::token::Token;

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> T;
}

pub trait Acceptor<T> {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> T;
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
}

impl<T> Acceptor<T> for Stmt {
    fn accept(&mut self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Print { expression } => visitor.visit_print_stmt(&Stmt::Print {
                expression: expression.clone(),
            }),
            Stmt::Expression { expression } => visitor.visit_expression_stmt(&Stmt::Expression {
                expression: expression.clone(),
            }),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, &initializer),
        }
    }
}
