use crate::json::JsonNode;
use crate::tokenizer::{Token, Tokenizer};
use std::collections::HashMap;

pub struct Parser;

impl Parser {
    pub fn parse(source: &str) -> JsonNode {
        let mut tokenizer = Tokenizer::new(source);
        return Self::parse_from(tokenizer.next().unwrap(), &mut tokenizer);
    }

    fn parse_from(token: Token, tokenizer: &mut Tokenizer) -> JsonNode {
        return match token {
            Token::BraceOn => Self::parse_object(tokenizer),
            Token::BracketOn => Self::parse_array(tokenizer),
            Token::Null => JsonNode::Null,
            Token::String(s) => JsonNode::String(s),
            Token::Number(n) => JsonNode::Number(n),
            Token::Boolean(b) => JsonNode::Boolean(b),
            _ => panic!("Unexpected token: {:?}", &token),
        };
    }

    fn parse_object(tokenizer: &mut Tokenizer) -> JsonNode {
        let mut object: HashMap<String, JsonNode> = HashMap::new();
        while let Some(tk) = tokenizer.next() {
            match tk {
                Token::BraceOff => break,
                Token::Comma => {}
                _ => {
                    let key: String = match Self::parse_from(tk, tokenizer) {
                        JsonNode::String(s) => s,
                        node @ _ => panic!("{:?} is not string.", node),
                    };
                    match tokenizer.next().unwrap() {
                        Token::Colon => {}
                        token @ _ => panic!("{:?} is not ':'", token),
                    }
                    let value: JsonNode = Self::parse_from(tokenizer.next().unwrap(), tokenizer);
                    object.insert(key, value);
                }
            }
        }
        return JsonNode::Object(object);
    }

    fn parse_array(tokenizer: &mut Tokenizer) -> JsonNode {
        let mut array: Vec<JsonNode> = vec![];
        while let Some(tk) = tokenizer.next() {
            match tk {
                Token::Comma => {
                    let next = tokenizer.next().unwrap();
                    array.push(Self::parse_from(next, tokenizer));
                }
                Token::BracketOff => break,
                _ => array.push(Self::parse_from(tk, tokenizer)),
            }
        }
        return JsonNode::Array(array);
    }
}
