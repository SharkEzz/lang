use std::fmt::Debug;

use super::stmt::Stmt;

pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Program({:#?})", self.statements)
    }
}
