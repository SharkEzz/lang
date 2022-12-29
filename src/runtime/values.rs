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
}
