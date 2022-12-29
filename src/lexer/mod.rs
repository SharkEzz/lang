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
    Comment,
    Identifier,
    Skipped,
    EOL,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub line: usize,
    pub column: usize,
    pub end_column: usize,
}

/// Create a new EOF type token.
#[macro_export]
macro_rules! eof_token {
    ($l:expr, $c:expr, $e:expr) => {
        Token {
            kind: TokenType::EOF,
            value: "EOF".to_string(),
            line: $l,
            column: $c,
            end_column: $e,
        }
    };
}
