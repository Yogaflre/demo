use std::collections::HashMap;
#[derive(Debug)]
pub enum JsonNode {
    Null,
    Number(f64),
    Boolean(bool),
    String(String),
    Array(Vec<JsonNode>),
    Object(HashMap<String, JsonNode>),
}

impl ToString for JsonNode {
    fn to_string(&self) -> std::string::String {
        let mut text = String::new();
        Self::write_json(self, &mut text);
        return text;
    }
}

impl JsonNode {
    fn write_json(json: &JsonNode, text: &mut String) {
        match json {
            JsonNode::Null => text.push_str("null"),
            JsonNode::Boolean(b) => {
                if *b {
                    text.push_str("true")
                } else {
                    text.push_str("false")
                };
            }
            JsonNode::Number(n) => text.push_str(&n.to_string()),
            JsonNode::String(s) => text.push_str(&format!("{:?}", s)),
            JsonNode::Object(o) => Self::write_object(o, text),
            JsonNode::Array(a) => Self::write_array(a, text),
        }
    }

    fn write_object(map: &HashMap<String, JsonNode>, text: &mut String) {
        text.push('{');

        map.iter().for_each(|(k, v)| {
            text.push_str(&format!("{:?}", k));
            text.push(':');
            Self::write_json(v, text);
            text.push(',');
        });
        text.pop();

        text.push('}');
    }

    fn write_array(array: &Vec<JsonNode>, text: &mut String) {
        text.push('[');

        array.iter().for_each(|json| {
            Self::write_json(json, text);
            text.push(',');
        });
        text.pop();

        text.push(']');
    }
}
