use crate::parser::Parser;
use std::{env::args, fs};

mod ast;
mod lexer;
mod parser;

fn main() {
    let file = args().nth(1).expect("No file provided");
    let content = fs::read_to_string(file).expect("Unable to read file");

    let mut parser = Parser::new(content.as_str());

    println!("{:#?}", parser.parse());
}
