pub mod tokenizer;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer,
    Float,
    String,
    Bool,

    // Symbols
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    OpenParen,
    CloseParen,
    SemiColon,
    OpenBrace,
    CloseBrace,
    Comma,
    Colon,

    // Keywords
    Let,
    Const,
    Func,
    Return,
    Print,      // print
    IntType,    // int
    FloatType,  // float
    StringType, // string
    BoolType,   // bool

    // Special tokens
    Identifier,
    Skipped,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub start_pos: usize,
    pub end_pos: usize,
}

/// Create a new EOF type token.
#[macro_export]
macro_rules! eof_token {
    () => {
        Token {
            kind: TokenType::EOF,
            value: "EOF".to_string(),
            start_pos: 0,
            end_pos: 0,
        }
    };
}
