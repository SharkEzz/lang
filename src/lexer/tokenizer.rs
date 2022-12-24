use super::{Token, TokenType};
use regex::Regex;

const SPECS: [(TokenType, &str); 5] = [
    (TokenType::Float, r"^\d+\.(\d+)?"),
    (TokenType::Integer, r"^\d+"),
    (TokenType::String, r"^'(?P<raw>[^']*)'"),
    (TokenType::String, "^\"(?P<raw>[^\"]*)\""),
    (TokenType::Skipped, r"^\s+"),
];

pub struct Tokenizer {
    position: usize,
    source: String,
    compiled_specs: Vec<(TokenType, Regex)>,
}

impl Tokenizer {
    pub fn new(source: &str) -> Tokenizer {
        Tokenizer {
            position: 0,
            source: String::from(source.trim()),
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

    fn match_string(&self, re: &Regex, input: &str) -> Option<(usize, String)> {
        let captures = re.captures_iter(input);

        let length: usize;

        let find = re.find(input);
        match find {
            Some(m) => {
                length = m.end() - m.start();
            }
            None => return None,
        }

        for capture in captures {
            match capture.name("raw") {
                Some(m) => return Some((length, m.as_str().to_string())),
                None => return Some((length, capture.get(0).unwrap().as_str().to_string())),
            }
        }

        None
    }
}

impl Iterator for Tokenizer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let current_str = &self.source[self.position..];

        for (token_type, regexp) in self.compiled_specs.iter() {
            let token_value = self.match_string(&regexp, current_str);
            if token_value.is_none() {
                continue;
            }

            let (length, value) = token_value.unwrap();
            self.position += length;

            match token_type {
                TokenType::Skipped => {
                    return self.next();
                }
                _ => {
                    return Some(Token {
                        kind: *token_type,
                        value,
                    });
                }
            }
        }

        None
    }
}
