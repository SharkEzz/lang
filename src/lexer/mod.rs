pub mod tokenizer;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    Integer,
    Float,
    String,

    Equal,
    Plus,
    Minus,
    Star,
    Slash,

    OpenParen,
    CloseParen,

    // Identifiers
    Let,

    // Special tokens
    Skipped,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
}
