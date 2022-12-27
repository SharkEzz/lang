#[derive(Debug)]
pub enum RuntimeVal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}

#[derive(Debug)]
pub enum RuntimeError {
    CannotRedefineConstant(String),
    InvalidOperandType,
    DivisionByZero,
}
