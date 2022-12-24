pub mod tokenizer;

#[derive(Debug, Clone, Copy)]
enum TokenType {
    Integer,
    Float,
    String,

    // Special tokens
    Skipped,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    value: String,
}
