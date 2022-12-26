use super::{Token, TokenType};
use regex::Regex;

const SPECS: &'static [(TokenType, &str)] = &[
    (TokenType::Float, r"^\d+\.(\d+)?"),
    (TokenType::Integer, r"^\d+"),
    (TokenType::String, r"^'(?P<raw>[^']*)'"),
    (TokenType::String, "^\"(?P<raw>[^\"]*)\""),
    // Symbols
    (TokenType::Equal, r"^="),
    (TokenType::Plus, r"^\+"),
    (TokenType::Minus, r"^-"),
    (TokenType::Star, r"^\*"),
    (TokenType::Slash, r"^/"),
    (TokenType::OpenParen, r"^\("),
    (TokenType::CloseParen, r"^\)"),
    (TokenType::SemiColon, r"^;"),
    (TokenType::OpenBrace, r"^\{"),
    (TokenType::CloseBrace, r"^}"),
    // Keywords
    (TokenType::Let, r"^let"),
    (TokenType::Const, r"^const"),
    (TokenType::Func, r"^func"),
    (TokenType::Return, r"^return"),
    // Special tokens
    (TokenType::Identifier, r"^[a-zA-Z_][a-zA-Z0-9_]*"),
    (TokenType::Skipped, r"^\s+"),
    (TokenType::EOF, r"^\s+"),
];

pub struct Tokenizer {
    position: usize,
    source: String,
    compiled_specs: Vec<(TokenType, Regex)>,
}

impl Tokenizer {
    pub fn new(source: &str) -> Self {
        Tokenizer {
            position: 0,
            source: source.to_string(),
            compiled_specs: SPECS
                .to_owned()
                .into_iter()
                .map(|(token_type, regexp)| {
                    (
                        token_type,
                        Regex::new(regexp).expect("Invalid regular expression"),
                    )
                })
                .collect(),
        }
    }

    fn match_string(&self, re: &Regex, input: &str) -> (usize, Option<String>) {
        let captures = re.captures_iter(input);

        let length: usize;

        let find = re.find(input);
        match find {
            Some(m) => {
                length = m.end() - m.start();
            }
            None => return (0, None),
        };

        for capture in captures {
            match capture.name("raw") {
                Some(m) => return (length, Some(m.as_str().to_string())),
                None => return (length, Some(capture.get(0).unwrap().as_str().to_string())),
            }
        }

        (length, None)
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let substr = &self.source.get(self.position..);
        if substr.is_none() {
            return None;
        }

        for (token_type, regex) in &self.compiled_specs {
            let (length, value) = self.match_string(regex, substr.unwrap());

            self.position += length;

            match value {
                // If we have no match with the current regex, try the next one
                None => continue,
                Some(value) => match token_type {
                    // If the current token is skipped, call ourself to get the next one
                    TokenType::Skipped => return self.next(),
                    _ => {
                        return Some(Token {
                            kind: token_type.to_owned(),
                            value,
                        });
                    }
                },
            }
        }

        // If we reach this point, it means that we have no match for the current character
        if self.position < self.source.len() {
            panic!(
                "Unexpected character: {} at position {}",
                &self.source[self.position..self.position + 1],
                self.position
            );
        }

        None
    }
}
