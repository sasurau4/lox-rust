use super::expr::{Acceptor, Expr, Visitor};
use super::token::{Literal, Token};

#[derive(Debug, Clone, Copy)]
pub struct AstPrinter {}

impl AstPrinter {
    pub fn print(self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(self, name: String, exprs: Vec<Expr>) -> String {
        let mut string = String::new();
        string.push_str("(");
        string.push_str(&name);
        for expr in exprs {
            string.push_str(" ");
            string.push_str(&expr.accept(self))
        }
        string.push_str(")");
        string
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary(self, left: Expr, operator: Token, right: Expr) -> String {
        self.parenthesize(operator.lexeme, vec![left, right])
    }
    fn visit_grouping(self, expr: Expr) -> String {
        match expr {
            Expr::Grouping { expression } => {
                self.parenthesize(String::from("group"), vec![*expression])
            }
            _ => unreachable!(),
        }
    }
    fn visit_literal(self, expr: Literal) -> String {
        match expr {
            Literal::None => String::from("nil"),
            Literal::Usize(u) => u.to_string(),
            Literal::Float(f) => f.to_string(),
            Literal::String(s) => String::from(s),
            Literal::Bool(b) => b.to_string(),
        }
    }
    fn visit_unary(self, operator: Token, right: Expr) -> String {
        self.parenthesize(operator.lexeme, vec![right])
    }
}
