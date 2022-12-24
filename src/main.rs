mod ast;
mod lexer;

use lexer::tokenizer::Tokenizer;

fn main() {
    let tokenizer = Tokenizer::new("1.1 1 'coucou' 1.1");

    for token in tokenizer {
        println!("{:#?}", token);
    }
}
