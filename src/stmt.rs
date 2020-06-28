use super::expr::Expr;
use super::token::Token;

pub trait Visitor<T> {
    fn visit_expression_stmt(&mut self, expression: &Expr) -> T;
    fn visit_print_stmt(&mut self, expression: &Expr) -> T;
    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> T;
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> T;
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> T;
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> T;
    fn visit_function_stmt(&mut self, name: &Token, params: &[Token], body: &[Stmt]) -> T;
    fn visit_return_stmt(&mut self, keyword: &Token, value: &Expr) -> T;
    fn visit_class_stmt(&mut self, name: &Token, methods: &[Stmt]) -> T;
}

pub trait Acceptor<T> {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression {
        expression: Expr,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    Print {
        expression: Expr,
    },
    Return {
        keyword: Token,
        value: Expr,
    },
    Var {
        name: Token,
        initializer: Expr,
    },
    Block {
        statements: Vec<Stmt>,
    },
    Class {
        name: Token,
        // Note: only for Stmt::Funtion
        methods: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}

impl<T> Acceptor<T> for Stmt {
    fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
        match self {
            Stmt::Print { expression } => visitor.visit_print_stmt(expression),
            Stmt::Expression { expression } => visitor.visit_expression_stmt(expression),
            Stmt::Var { name, initializer } => visitor.visit_var_stmt(name, &initializer),
            Stmt::Block { statements } => visitor.visit_block_stmt(statements),
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => visitor.visit_if_stmt(condition, then_branch, else_branch),
            Stmt::While { condition, body } => visitor.visit_while_stmt(condition, body),
            Stmt::Function { name, params, body } => {
                visitor.visit_function_stmt(name, params, body)
            }
            Stmt::Return { keyword, value } => visitor.visit_return_stmt(keyword, value),
            Stmt::Class { name, methods } => visitor.visit_class_stmt(name, methods),
        }
    }
}
