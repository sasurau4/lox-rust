use super::callable::FunctionType;
use super::error::{resolve_error, Error, Result};
use super::expr::Expr;
use super::expr::{Acceptor as ExprAcceptor, Visitor as ExprVisitor};
use super::interpreter::Interpreter;
use super::lox_class::ClassType;
use super::stmt::{Acceptor as StmtAcceptor, Stmt, Visitor as StmtVisitor};
use super::token::Literal;
use super::token::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    pub scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
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
        let mut i = scopes_count - 1;
        #[allow(unused_comparisons)]
        #[allow(clippy::absurd_extreme_comparisons)]
        while i >= 0 {
            if let Some(scope) = self.scopes.get(i) {
                if let Some(_r) = scope.get(&name.lexeme) {
                    self.interpreter.resolve(expr, scopes_count - 1 - i)?;
                    return Ok(());
                }
            }
            i -= 1;
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
    fn visit_get(&mut self, object: &Expr, _name: &Token) -> Result<()> {
        self.resolve_expr(object)
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
    fn visit_set(&mut self, object: &Expr, _name: &Token, value: &Expr) -> Result<()> {
        self.resolve_expr(value)?;
        self.resolve_expr(object)
    }
    fn visit_this(&mut self, keyword: &Token) -> Result<()> {
        if self.current_class == ClassType::None {
            return Err(Error::ResolveError(
                keyword.clone(),
                "Cannot use 'this' outside of a class.".to_string(),
            ));
        }
        let expr = Expr::This {
            keyword: keyword.clone(),
        };
        self.resolve_local(expr, keyword)
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
    fn visit_class_stmt(&mut self, name: &Token, methods: &[Stmt]) -> Result<()> {
        let enclosing_class = self.current_class;
        self.current_class = ClassType::Class;
        self.declare(name)?;
        self.define(name);

        self.begin_scope();
        let mut scope = self.scopes.pop().unwrap();
        scope.insert("this".to_string(), true);
        self.scopes.push(scope);
        for method in methods {
            match method {
                Stmt::Function {
                    name: func_name,
                    params,
                    body,
                } => {
                    let declaration = if func_name.lexeme == "init" {
                        FunctionType::Initializer
                    } else {
                        FunctionType::Method
                    };
                    self.resolve_function(func_name, params, body, declaration)?
                }
                _ => unreachable!(),
            }
        }
        self.end_scope();
        self.current_class = enclosing_class;
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
        // If you check current_function before, you should implement the last check for
        // is_initializer inside LoxFuncti at 12.6.2 "returnint from init()" section
        if let Expr::Literal { value } = v {
            if value == &Literal::None {
                return Ok(());
            }
        }
        if self.current_function == FunctionType::Initializer {
            return Err(Error::ResolveError(
                keyword.clone(),
                "Cannot return a value from an initializer.".to_string(),
            ));
        }
        self.resolve_expr(v)
    }
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<()> {
        self.resolve_expr(condition)?;
        self.resolve_statement(body)?;
        Ok(())
    }
}
