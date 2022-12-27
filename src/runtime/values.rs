#[derive(Debug, Clone)]
pub enum RuntimeVal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}

#[derive(Debug)]
pub enum RuntimeError {
    ConstantReassignment(String),
    VarRedeclaration(String),
    InvalidOperandType,
    DivisionByZero,
    UndefinedVariable(String),
}
