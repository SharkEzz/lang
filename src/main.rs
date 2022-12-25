mod ast;
mod lexer;
mod parser;

use parser::Parser;

fn main() {
    let mut parser = Parser::new("1 + 1");
    println!("{:#?}", parser.parse());
}
