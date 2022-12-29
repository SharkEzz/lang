use std::fmt::Display;

use crate::ast::stmt::Stmt;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeVal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Func(String, Vec<String>, Stmt),
    Block(Box<RuntimeVal>),
    Return(Box<RuntimeVal>),
    Undefined,
}

impl Display for RuntimeVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeVal::Int(val) => write!(f, "{val}"),
            RuntimeVal::Float(val) => write!(f, "{val}"),
            RuntimeVal::Bool(val) => write!(f, "{val}"),
            RuntimeVal::String(val) => write!(f, "{val}"),
            _ => write!(f, "{:#?}", self),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeError {
    ConstantReassignment(String),
    VarRedeclaration(String),
    FuncRedeclaration(String),
    InvalidOperandType,
    DivisionByZero,
    UndefinedVariable(String),
    UndefinedFunction(String),
    InvalidFuncCallParametersCount(String),
    InvalidType,
}
