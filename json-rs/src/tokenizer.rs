use std::iter::Peekable;
use std::str::Chars;
use std::string::String;

#[derive(Debug)]
pub enum Token {
    Comma,
    Colon,
    BraceOn,
    BraceOff,
    BracketOn,
    BracketOff,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.source.next() {
            return Some(match c {
                ',' => Token::Comma,
                ':' => Token::Colon,
                '{' => Token::BraceOn,
                '}' => Token::BraceOff,
                '[' => Token::BracketOn,
                ']' => Token::BracketOff,
                '"' => Token::String(self.read_string(c)),
                '0'..='9' => Token::Number(self.read_number(c)),
                'f' | 't' | 'n' => {
                    let label = self.read_symbol(c);
                    match label.as_ref() {
                        "true" => Token::Boolean(true),
                        "false" => Token::Boolean(false),
                        "null" => Token::Null,
                        _ => panic!("Invalid token: {}", label),
                    }
                }
                _ => {
                    if c.is_whitespace() || c == '\n' {
                        continue;
                    } else {
                        panic!("Invalid character: {}", c);
                    }
                }
            });
        }
        return Some(Token::String(String::new()));
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        return Self {
            source: text.chars().peekable(),
        };
    }

    fn read_string(&mut self, first: char) -> String {
        let mut value: String = String::new();
        let mut flag = false;
        while let Some(c) = self.source.next() {
            if c == first {
                return value;
            }
            match c {
                '\\' => {
                    if flag {
                        value.push(c);
                        flag = false;
                    } else {
                        flag = true;
                    }
                }
                _ => {
                    value.push(c);
                }
            }
        }
        return value;
    }

    fn read_number(&mut self, first: char) -> f64 {
        let mut value: String = first.to_string();
        let mut point = false;
        while let Some(&c) = self.source.peek() {
            match c {
                '0'..='9' => {
                    value.push(c);
                    self.source.next();
                }
                '.' => {
                    if !point {
                        point = true;
                        value.push(c);
                        self.source.next();
                    } else {
                        panic!("number parse invalid.")
                    }
                }
                _ => break,
            }
        }
        return value.parse::<f64>().unwrap();
    }

    fn read_symbol(&mut self, first: char) -> String {
        let mut value = first.to_string();
        while let Some(&c) = self.source.peek() {
            match c {
                'a'..='z' => {
                    value.push(c);
                    self.source.next();
                }
                _ => break,
            }
        }
        return value;
    }
}
