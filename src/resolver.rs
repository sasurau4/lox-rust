use super::callable::FunctionType;
use super::error::{resolve_error, Error, Result};
use super::expr::Expr;
use super::expr::{Acceptor as ExprAcceptor, Visitor as ExprVisitor};
use super::interpreter::Interpreter;
use super::stmt::{Acceptor as StmtAcceptor, Stmt, Visitor as StmtVisitor};
use super::token::Literal;
use super::token::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    pub scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
        }
    }

    pub fn resolve_statements(&mut self, statements: &[Stmt]) -> Result<()> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, stmt: &Stmt) -> Result<()> {
        stmt.accept(self)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<()> {
        expr.accept(self)
    }

    fn resolve_function(
        &mut self,
        _name: &Token,
        params: &[Token],
        body: &[Stmt],
        func_type: FunctionType,
    ) -> Result<()> {
        let enclosing_function = self.current_function;
        self.current_function = func_type;

        self.begin_scope();
        for param in params {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve_statements(body)?;
        self.end_scope();
        self.current_function = enclosing_function;
        Ok(())
    }

    fn resolve_local(&mut self, expr: Expr, name: &Token) -> Result<()> {
        if self.scopes.is_empty() {
            return Ok(());
        }
        let scopes_count = self.scopes.len();
        for i in (scopes_count - 1)..=0 {
            if self.scopes.get(i).unwrap().contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, scopes_count - 1 - i)?;
                return Ok(());
            }
        }
        // Not found. Assume it is global.
        Ok(())
    }

    fn declare(&mut self, name: &Token) -> Result<()> {
        if self.scopes.is_empty() {
            return Ok(());
        }
        let mut scope = self.scopes.pop().unwrap();
        if scope.contains_key(&name.lexeme) {
            return Err(Error::ResolveError(
                name.clone(),
                String::from("Variable with this name already declared in this scope."),
            ));
        }
        scope.insert(name.lexeme.clone(), false);
        self.scopes.push(scope);
        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }
        let mut scope = self.scopes.pop().unwrap();
        scope.insert(name.lexeme.clone(), true);
        self.scopes.push(scope);
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }
}

impl<'a> ExprVisitor<Result<()>> for Resolver<'a> {
    fn visit_variable(&mut self, name: &Token) -> Result<()> {
        if let Some(scope) = self.scopes.iter().peekable().peek() {
            if let Some(var) = scope.get(&name.lexeme) {
                if var == &false {
                    resolve_error(
                        name.clone(),
                        "Cannot read local variable in its own initializer.",
                    );
                    return Err(Error::ResolveError(
                        name.clone(),
                        String::from("Cannot read local variable in its own initializer."),
                    ));
                }
            }
        }
        let expr = Expr::Variable { name: name.clone() };
        self.resolve_local(expr, name)?;
        Ok(())
    }

    fn visit_assign(&mut self, name: &Token, value: &Expr) -> Result<()> {
        self.resolve_expr(value)?;
        let expr = Expr::Assign {
            name: name.clone(),
            value: Box::new(value.clone()),
        };
        self.resolve_local(expr, name)?;
        Ok(())
    }
    fn visit_binary(&mut self, left: &Expr, _operator: &Token, right: &Expr) -> Result<()> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;
        Ok(())
    }
    fn visit_call(&mut self, callee: &Expr, _paren: &Token, arguments: &[Expr]) -> Result<()> {
        self.resolve_expr(callee)?;
        for arg in arguments {
            self.resolve_expr(arg)?;
        }
        Ok(())
    }
    fn visit_grouping(&mut self, expression: &Expr) -> Result<()> {
        self.resolve_expr(expression)?;
        Ok(())
    }
    fn visit_literal(&mut self, _expr: &Literal) -> Result<()> {
        Ok(())
    }
    fn visit_logical(&mut self, left: &Expr, _operator: &Token, right: &Expr) -> Result<()> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;
        Ok(())
    }
    fn visit_unary(&mut self, _operator: &Token, right: &Expr) -> Result<()> {
        self.resolve_expr(right)?;
        Ok(())
    }
}
impl<'a> StmtVisitor<Result<()>> for Resolver<'a> {
    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<()> {
        self.resolve_expr(expression)?;
        Ok(())
    }
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> Result<()> {
        self.begin_scope();
        self.resolve_statements(statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_var_stmt(&mut self, name: &Token, initializer: &Expr) -> Result<()> {
        self.declare(name)?;
        match initializer {
            Expr::Literal { value } => match value {
                Literal::None => {}
                _ => self.resolve_expr(initializer)?,
            },
            _ => self.resolve_expr(initializer)?,
        }
        self.define(name);
        Ok(())
    }
    fn visit_function_stmt(&mut self, name: &Token, params: &[Token], body: &[Stmt]) -> Result<()> {
        self.declare(name)?;
        self.define(name);

        self.resolve_function(name, params, body, FunctionType::Function)?;
        Ok(())
    }
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Box<Stmt>>,
    ) -> Result<()> {
        self.resolve_expr(condition)?;
        self.resolve_statement(then_branch)?;
        match else_branch {
            Some(eb) => self.resolve_statement(eb),
            _ => Ok(()),
        }
    }
    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<()> {
        self.resolve_expr(expression)?;
        Ok(())
    }
    fn visit_return_stmt(&mut self, keyword: &Token, v: &Expr) -> Result<()> {
        if self.current_function == FunctionType::None {
            return Err(Error::ResolveError(
                keyword.clone(),
                String::from("Cannot return from top-level code."),
            ));
        }
        match v {
            Expr::Literal { value } => match value {
                Literal::None => Ok(()),
                _ => self.resolve_expr(v),
            },
            _ => self.resolve_expr(v),
        }
    }
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<()> {
        self.resolve_expr(condition)?;
        self.resolve_statement(body)?;
        Ok(())
    }
}