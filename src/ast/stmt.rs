use super::expr::Expr;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    // name, is_const, value
    VarDeclaration(String, bool, Expr),
    // name, parameters, body
    FuncDeclaration(String, Vec<String>, Box<Stmt>),
    // { ... }
    Block(Vec<Stmt>),
    // return ...
    Return(Expr),
    Expression(Expr),
}
