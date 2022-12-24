pub enum Types {
    Program,

    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
}

pub trait Stmt {
    fn get_type(&self) -> Types;
}

pub trait Expr: Stmt {}

pub struct Program {
    pub stmts: Vec<Box<dyn Stmt>>,
}

impl Stmt for Program {
    fn get_type(&self) -> Types {
        Types::Program
    }
}

pub struct IntegerLiteral {
    pub value: i32,
}

impl Stmt for IntegerLiteral {
    fn get_type(&self) -> Types {
        Types::IntegerLiteral
    }
}

impl Expr for IntegerLiteral {}

pub struct FloatLiteral {
    pub value: f32,
}

impl Stmt for FloatLiteral {
    fn get_type(&self) -> Types {
        Types::FloatLiteral
    }
}

impl Expr for FloatLiteral {}

pub struct StringLiteral {
    pub value: String,
}

impl Stmt for StringLiteral {
    fn get_type(&self) -> Types {
        Types::StringLiteral
    }
}

impl Expr for StringLiteral {}
