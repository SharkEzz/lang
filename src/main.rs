mod ast;
mod lexer;
mod parser;

use parser::Parser;

fn main() {
    let parser = Parser::new("1.1 1 'coucou' 1.1");
    parser.parse();
}
