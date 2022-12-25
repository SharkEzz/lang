use super::atom::Atom;

#[derive(Debug)]
pub enum Expr {
    // left - operator - right
    Binary(Box<Expr>, Atom, Box<Expr>),
    // left - right
    Assignment(Box<Expr>, Box<Expr>),
    Literal(Atom),
}
