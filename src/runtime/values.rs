#[derive(Debug)]
pub enum RuntimeVal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}
