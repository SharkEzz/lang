use crate::lexer::Token;

use super::atom::Atom;

#[derive(Debug, PartialEq)]
pub enum Expr {
    // left, operator, right
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(Atom),
}
