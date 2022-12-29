use crate::lexer::TokenType;

use super::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    // name, type, is_const, value
    VarDeclaration(String, TokenType, bool, Expr),
    // name, parameters, body
    FuncDeclaration(String, Vec<String>, Box<Stmt>),
    // { ... }
    Block(Vec<Stmt>),
    // return ...
    Return(Expr),
    Print(Expr),
    Expression(Expr),
}
