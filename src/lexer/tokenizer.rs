use super::{Token, TokenType};
use regex::Regex;

const SPECS: [(TokenType, &str); 13] = [
    (TokenType::Float, r"^\d+\.(\d+)?"),
    (TokenType::Integer, r"^\d+"),
    (TokenType::String, r"^'(?P<raw>[^']*)'"),
    (TokenType::String, "^\"(?P<raw>[^\"]*)\""),
    (TokenType::Equal, r"^="),
    (TokenType::Plus, r"^+"),
    (TokenType::Minus, r"^-"),
    (TokenType::Star, r"^*"),
    (TokenType::Slash, r"^/"),
    (TokenType::OpenParen, r"^\("),
    (TokenType::CloseParen, r"^\)"),
    // Keywords
    (TokenType::Let, r"^let"),
    // Special tokens
    (TokenType::Skipped, r"^\s+"),
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
                .iter()
                .map(|(token_type, regexp)| {
                    (
                        *token_type,
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
            None => length = 1,
        }

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
        let current_str = &self.source[self.position..];

        for (token_type, regexp) in self.compiled_specs.iter() {
            let (length, token_value) = self.match_string(&regexp, current_str);
            self.position += length;
            if token_value.is_none() {
                continue;
            }

            match token_value {
                None => continue,
                Some(value) => match token_type {
                    TokenType::Skipped => {
                        return self.next();
                    }
                    _ => {
                        return Some(Token {
                            kind: *token_type,
                            value,
                        });
                    }
                },
            }
        }

        if self.position < self.source.len() {
            panic!(
                "Unexpected token: {} at position {}",
                &self.source[self.position..],
                self.position
            )
        }

        None
    }
}
