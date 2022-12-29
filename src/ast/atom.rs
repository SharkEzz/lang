#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
}
