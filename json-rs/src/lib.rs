mod json;
mod parser;
mod tokenizer;

/**
 * simple json parser test
 */
#[cfg(test)]
mod tests {
    use crate::json::JsonNode;
    use crate::parser::Parser;
    #[test]
    fn parse_json() {
        let example = r#"{
            "success": {
                "status_code": 200,
                "message": "Welcome to json-rs."
            },
            "bool": true,
            "none": null,
            "float": 4214.51321,
            "list": [
                "hello",
                1,
                false
            ]
        }"#;
        let json: JsonNode = Parser::parse(example);
        println!("json_node:\n{:?}", json);
        println!("json_string:\n{}", json.to_string());
    }

    #[test]
    fn parse_list() {
        let example = "[1,2,3,4,5]";
        let json: JsonNode = Parser::parse(example);
        println!("json_node:\n{:?}", json);
        println!("json_string:\n{}", json.to_string());
    }

    #[test]
    fn parse_number() {
        let example = "123";
        let json: JsonNode = Parser::parse(example);
        println!("json_node:\n{:?}", json);
        println!("json_string:\n{}", json.to_string());
    }

    #[test]
    fn parse_str() {
        let example = "\"hello\"";
        let json: JsonNode = Parser::parse(example);
        println!("json_node:\n{:?}", json);
        println!("json_string:\n{}", json.to_string());
    }

    #[test]
    fn parse_bool() {
        let example = "true";
        let json: JsonNode = Parser::parse(example);
        println!("json_node:\n{:?}", json);
        println!("json_string:\n{}", json.to_string());
    }

    #[test]
    fn parse_null() {
        let example = "null";
        let json: JsonNode = Parser::parse(example);
        println!("json_node:\n{:?}", json);
        println!("json_string:\n{}", json.to_string());
    }

    #[test]
    fn parse_error() {
        let example = "{test}";
        let json: JsonNode = Parser::parse(example);
        println!("json_node:\n{:?}", json);
        println!("json_string:\n{}", json.to_string());
    }
}
