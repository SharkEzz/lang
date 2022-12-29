use std::fmt::Debug;

use super::stmt::Stmt;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Stmt>,
}
