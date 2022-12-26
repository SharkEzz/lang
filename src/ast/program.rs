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

#[derive(Debug, PartialEq)]
pub enum Stmt {
    // name, is_const, value
    VarDeclaration(String, bool, Expr),
    Expression(Expr),
}
