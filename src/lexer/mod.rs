pub mod tokenizer;

#[derive(Debug)]
enum TokenType {
    Integer,
    Float,
    String,

    Skipped,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    value: String,
}
