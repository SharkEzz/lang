use crate::{
    ast::{Program, Stmt},
    lexer::{tokenizer::Tokenizer, Token},
};

pub struct Parser {
    tokenizer: Tokenizer,
    current: Option<Token>,
    previous: Option<Token>,
    lookahead: Option<Token>,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        Parser {
            tokenizer: Tokenizer::new(source),
            current: None,
            previous: None,
            lookahead: None,
        }
    }

    pub fn parse(&self) -> Option<Box<dyn Stmt>> {
        Some(Box::new(Program { stmts: vec![] }))
    }
}
