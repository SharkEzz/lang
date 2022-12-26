pub mod tokenizer;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer,
    Float,
    String,

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

    // Keywords
    Let,
    Const,
    Func,
    Return,

    // Special tokens
    Identifier,
    Skipped,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}

/// Create a new EOF type token.
#[macro_export]
macro_rules! eof_token {
    () => {
        Token {
            kind: TokenType::EOF,
            value: "EOF".to_string(),
        }
    };
}
