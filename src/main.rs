use crate::parser::Parser;

mod ast;
mod lexer;
mod parser;

fn main() {
    let mut parser = Parser::new("const test = 1 + (1 * 1)");

    println!("{:#?}", parser.parse());
}
