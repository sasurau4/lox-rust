use super::expr::Expr;

pub trait Visitor<T> {
    fn visit_expression_stmt(self, stmt: Stmt) -> T;
    fn visit_print_stmt(self, stmt: Stmt) -> T;
}

pub trait Acceptor<T> {
    fn accept(self, visitor: impl Visitor<T>) -> T;
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
}

impl<T> Acceptor<T> for Stmt {
    fn accept(self, visitor: impl Visitor<T>) -> T {
        match self {
            Stmt::Print { expression } => visitor.visit_print_stmt(Stmt::Print { expression }),
            Stmt::Expression { expression } => {
                visitor.visit_expression_stmt(Stmt::Expression { expression })
            }
        }
    }
}
