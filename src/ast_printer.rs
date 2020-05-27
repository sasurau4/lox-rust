use super::expr::{Acceptor, Binary, Expr, Grouping, Literal, Unary, Visitor};
use super::token::Literal as LiteralToken;

#[derive(Debug, Clone, Copy)]
pub struct AstPrinter {}

impl<'a> AstPrinter {
    pub fn print(self, expr: Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(self, name: &str, exprs: Vec<Expr>) -> String {
        let mut string = String::new();
        string.push_str("(");
        string.push_str(name);
        for expr in exprs {
            string.push_str(" ");
            string.push_str(&expr.accept(self))
        }
        string.push_str(")");
        string
    }
}

impl Visitor<String> for AstPrinter {
    // fn visit_expr(self, expr: Expr) -> String {
    //     String::from("piyo")
    // }
    fn visit_binary(self, expr: Binary) -> String {
        self.parenthesize(expr.operator.lexeme, vec![*expr.left, *expr.right])
    }
    fn visit_grouping(self, expr: Grouping) -> String {
        self.parenthesize("group", vec![*expr.expression])
    }
    fn visit_literal(self, expr: Literal) -> String {
        match expr.value {
            LiteralToken::None => String::from("nil"),
            LiteralToken::Usize(u) => u.to_string(),
            LiteralToken::Float(f) => f.to_string(),
            LiteralToken::String(s) => String::from(s),
        }
    }
    fn visit_unary(self, expr: Unary) -> String {
        self.parenthesize(expr.operator.lexeme, vec![*expr.right])
    }
}
