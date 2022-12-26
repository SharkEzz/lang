use super::expr::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    // name, is_const, value
    VarDeclaration(String, bool, Expr),
    Expression(Expr),
}
