use crate::{parser::Parser, runtime::environment::Environment};
use std::{env::args, fs};

mod ast;
mod lexer;
mod parser;
mod runtime;

fn main() {
    let file = args().nth(1).expect("No file provided");
    let content = fs::read_to_string(file).expect("Unable to read file");

    let mut parser = Parser::new(content.as_str());
    let program = parser.parse();

    let environment = Environment::new(None);

    println!("{:#?}", program);
}
