use crate::{
    parser::Parser,
    runtime::{environment::Environment, interpreter::Interpreter},
};
use std::{cell::RefCell, env::args, fs, rc::Rc};

mod ast;
mod lexer;
mod parser;
mod runtime;

fn main() {
    let file = args().nth(1).expect("No file provided");
    let content = fs::read_to_string(file).expect("Unable to read file");

    let mut parser = Parser::new(content.as_str());
    let program = parser.parse();
    println!("{:#?}", program);
    println!("------------");

    let env = Rc::new(RefCell::new(Environment::new(None)));
    let runtime = Interpreter {};

    let result = runtime.evaluate_program(&program, env);
    println!("{:#?}", result);
}
