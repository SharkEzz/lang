use std::fmt::Debug;

use super::expr::Expr;

pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Program({:#?})", self.statements)
    }
}

#[derive(Debug)]
pub enum Stmt {
    // name - is constant? - value
    VarDecl(String, bool, Box<Expr>),
    Statement(Box<Stmt>),
    Expression(Expr),
}
