use crate::lexer::TokenType;

use super::atom::Atom;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    // left, operator, right
    Binary(Box<Expr>, TokenType, Box<Expr>),
    Assignment(Box<Expr>, TokenType, Box<Expr>),
    Identifier(String),
    CallExpr(String, Vec<Expr>),
    Literal(Atom),
}
