use crate::{
    parser::Parser,
    runtime::{environment::Environment, interpreter::Interpreter},
};
use std::{
    cell::RefCell,
    env::{self, args},
    fs,
    rc::Rc,
};

mod ast;
mod lexer;
mod parser;
mod runtime;

fn main() {
    let is_debug = match env::var("TR_DEBUG") {
        Ok(value) => value == "1",
        Err(_) => false,
    };

    let file = args().nth(1).expect("No file provided");
    let content = fs::read_to_string(file).expect("Unable to read file");

    let mut parser = Parser::new(content.as_str());
    let program = parser.parse();

    if is_debug {
        println!("{:#?}", program);
        println!("------------");
    }

    let env = Rc::new(RefCell::new(Environment::new(None)));
    let runtime = Interpreter {};

    let result = runtime.evaluate_program(&program, env);
    if is_debug {
        println!("{:#?}", result);
    }
}
