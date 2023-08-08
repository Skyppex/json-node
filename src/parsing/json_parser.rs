use crate::{models::JsonValueType, parsing::tokens, JsonNode};

pub struct JsonNodeParser;

impl JsonNodeParser {
    pub fn parse_node(json_node_as_json_string: &str) -> Result<JsonNode, json::Error> {
        if let Some(node) = Self::parse_value(json_node_as_json_string) {
            return Ok(node);
        }

        if let Some(node) = Self::parse_array(json_node_as_json_string) {
            return Ok(node);
        }
        
        if let Some(node) = Self::parse_object(json_node_as_json_string) {
            return Ok(node);
        }

        Err(json::Error::UnexpectedEndOfJson)
    }

    fn parse_value(json: &str) -> Option<JsonNode> {
        println!("Parsing Value: {}", json);

        if let Some(node) = Self::parse_string(json) {
            return Some(node);
        }

        if let Some(node) = Self::parse_integer(json) {
            return Some(node);
        }

        if let Some(node) = Self::parse_float(json) {
            return Some(node);
        }

        if let Some(node) = Self::parse_boolean(json) {
            return Some(node);
        }

        if let Some(node) = Self::parse_null(json) {
            return Some(node);
        }

        None
    }

    fn parse_string(value: &str) -> Option<JsonNode> {
        let trim = value.trim();
        
        if trim.is_empty() {
            return None;
        }

        if trim.starts_with(tokens::DOUBLE_QUOTE) && trim.ends_with(tokens::DOUBLE_QUOTE) {
            let text = trim[1..trim.len() - 1].to_owned();
            return Some(JsonNode::JsonValue(JsonValueType::String(text)));
        }

        None
    }

    fn parse_integer(value: &str) -> Option<JsonNode> {
        let trim = value.trim();

        if trim.is_empty() {
            return None;
        }

        match trim.parse::<i64>() {
            Ok(num) => Some(JsonNode::JsonValue(JsonValueType::Integer(num))),
            Err(_) => None,
        }
    }

    fn parse_float(value: &str) -> Option<JsonNode> {
        let trim = value.trim();

        if trim.is_empty() {
            return None;
        }

        match trim.parse::<f64>() {
            Ok(num) => Some(JsonNode::JsonValue(JsonValueType::Float(num))),
            Err(_) => None,
        }
    }

    fn parse_boolean(value: &str) -> Option<JsonNode> {
        let trim = value.trim();

        if trim.is_empty() {
            return None;
        }

        if trim.eq_ignore_ascii_case(tokens::TRUE) {
            return Some(JsonNode::JsonValue(JsonValueType::Boolean(true)));
        }

        if trim.eq_ignore_ascii_case(tokens::FALSE) {
            return Some(JsonNode::JsonValue(JsonValueType::Boolean(false)));
        }

        None
    }

    fn parse_null(value: &str) -> Option<JsonNode> {
        let trim = value.trim();

        if trim.is_empty() {
            return None;
        }

        if trim.eq_ignore_ascii_case(tokens::NULL) {
            return Some(JsonNode::JsonValue(JsonValueType::Null));
        }

        None
    }

    fn parse_array(array: &str) -> Option<JsonNode> {
        let trim = array.trim();

        if trim.is_empty() {
            return None;
        }

        if trim.starts_with(tokens::LEFT_BRACKET) && trim.ends_with(tokens::RIGHT_BRACKET) {
            let no_brackets = trim[1..trim.len() - 1].trim();
            
            if no_brackets.replace(" ", "").replace("\t", "").is_empty() {
                return Some(JsonNode::JsonArray(Vec::new()));
            }

            let mut elements = Vec::new();

            let mut element = String::new();
            let mut level = 0;

            for char in no_brackets.chars() {
                if char == tokens::LEFT_BRACE || char == tokens::LEFT_BRACKET {
                    element += &char.to_string();
                    level += 1;
                } else if char == tokens::RIGHT_BRACE || char == tokens::RIGHT_BRACKET {
                    element += &char.to_string();
                    level -= 1;
                } else if char == tokens::COMMA && level == 0 {
                    elements.push(element.trim().to_owned());
                    element = String::new();
                } else {
                    element += &char.to_string();
                }
            }

            elements.push(element.trim().to_owned());

            let elements = elements.iter()
                .map(|value| value.trim())
                .map(|value| {
                    println!("Parsing: {}", value);
                    Self::parse_node(value).ok()
                })
                .collect::<Vec<Option<JsonNode>>>();

            let mut array = Vec::new();

            for e in elements.into_iter() {
                match e {
                    Some(node) => array.push(node),
                    None => return None,
                }
            }

            return Some(JsonNode::JsonArray(array));
        }

        None
    }

    fn parse_object(object: &str) -> Option<JsonNode> {
        let trim = object.trim();

        if trim.is_empty() {
            return None;
        }

        if trim.starts_with(tokens::LEFT_BRACE) && trim.ends_with(tokens::RIGHT_BRACE) {
            let no_braces = trim[1..trim.len() - 1].trim();
            
            if no_braces.replace(" ", "").replace("\t", "").is_empty() {
                return Some(JsonNode::JsonObject(Vec::new()));
            }

            let mut properties = Vec::new();

            let mut property = String::new();
            let mut level = 0;

            for char in no_braces.chars() {
                if char == tokens::LEFT_BRACE || char == tokens::LEFT_BRACKET {
                    property += &char.to_string();
                    level += 1;
                } else if char == tokens::RIGHT_BRACE || char == tokens::RIGHT_BRACKET {
                    property += &char.to_string();
                    level -= 1;
                } else if char == tokens::COMMA && level == 0 {
                    properties.push(property.trim().to_owned());
                    property = String::new();
                } else {
                    property += &char.to_string();
                }
            }

            properties.push(property.trim().to_owned());

            let kvps = properties.iter()
                .map(|property| property.trim())
                .map(|property| {
                    let (mut key, value) = property.split_once(tokens::COLON).unwrap();

                    key = &key[1..key.len() - 1];
                    println!("Parsing: {}, {}", key, value);
                    (key.to_owned(), Self::parse_node(value).ok())
                })
                .collect::<Vec<(String, Option<JsonNode>)>>();

            let objects = kvps.iter().map(|(k, p)| (k.clone(), p.clone().unwrap())).collect::<Vec<(String, JsonNode)>>();

            return Some(JsonNode::JsonObject(objects));
        }

        None
    }
}




#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};
    use crate::models::*;

    #[test]
    fn parse_string() {
        let json_string = "\"text\"";

        let json_node = JsonNode::parse(&json_string).unwrap();
        assert_eq!(json_node, JsonNode::JsonValue(JsonValueType::String("text".to_owned())));
    }

    #[test]
    fn parse_integer() {
        let json_integer = "123";

        let json_node = JsonNode::parse(&json_integer).unwrap();
        assert_eq!(json_node, JsonNode::JsonValue(JsonValueType::Integer(123)));
    }

    #[test]
    fn parse_float() {
        let json_float = "123.456";

        let json_node = JsonNode::parse(&json_float).unwrap();
        assert_eq!(json_node, JsonNode::JsonValue(JsonValueType::Float(123.456)));
    }

    #[test]
    fn parse_true() {
        let json_true = "true";

        let json_node = JsonNode::parse(&json_true).unwrap();
        assert_eq!(json_node, JsonNode::JsonValue(JsonValueType::Boolean(true)));
    }

    #[test]
    fn parse_false() {
        let json_false = "false";

        let json_node = JsonNode::parse(&json_false).unwrap();
        assert_eq!(json_node, JsonNode::JsonValue(JsonValueType::Boolean(false)));
    }

    #[test]
    fn parse_null() {
        let json_null = "null";

        let json_node = JsonNode::parse(&json_null).unwrap();
        assert_eq!(json_node, JsonNode::JsonValue(JsonValueType::Null));
    }

    #[test]
    fn parse_empty_object() {
        let json_empty_object = "{}";

        let json_node = JsonNode::parse(&json_empty_object).unwrap();
        assert_eq!(json_node, JsonNode::JsonObject(Vec::new()));
    }

    #[test]
    fn parse_filled_object() {
        let filled_json_object = r#"
        {
            "string": "value",
            "integer": 123,
            "float": 123.456,
            "true": true,
            "false": false,
            "null": null
        }"#;

        let json_object_node = JsonNode::parse(&filled_json_object).unwrap();
        let mut filled_map = HashMap::new();

        filled_map.insert("string".to_owned(), JsonNode::JsonValue(JsonValueType::String("value".to_owned())));
        filled_map.insert("integer".to_owned(), JsonNode::JsonValue(JsonValueType::Integer(123)));
        filled_map.insert("float".to_owned(), JsonNode::JsonValue(JsonValueType::Float(123.456)));
        filled_map.insert("true".to_owned(), JsonNode::JsonValue(JsonValueType::Boolean(true)));
        filled_map.insert("false".to_owned(), JsonNode::JsonValue(JsonValueType::Boolean(false)));
        filled_map.insert("null".to_owned(), JsonNode::JsonValue(JsonValueType::Null));

        match json_object_node {
            JsonNode::JsonObject(map) => {
                map.iter().for_each(|(k, v)| {
                    assert_eq!(v, filled_map.get(k).unwrap());
                });
            },
            _ => panic!("Expected JsonObject")
        }
    }

    #[test]
    fn parse_empty_array() {
        let json_empty_object = "[]";

        let json_node = JsonNode::parse(&json_empty_object).unwrap();
        assert_eq!(json_node, JsonNode::JsonArray(Vec::new()));
    }

    #[test]
    fn parse_filled_array() {
        let filled_json_object = r#"
        [
            "string",
            123,
            123.456,
            true,
            false,
            null
        ]"#;
        
        let json_array_node = JsonNode::parse(&filled_json_object).unwrap();
        let mut filled_array = Vec::new();

        filled_array.push(JsonNode::JsonValue(JsonValueType::String("string".to_owned())));
        filled_array.push(JsonNode::JsonValue(JsonValueType::Integer(123)));
        filled_array.push(JsonNode::JsonValue(JsonValueType::Float(123.456)));
        filled_array.push(JsonNode::JsonValue(JsonValueType::Boolean(true)));
        filled_array.push(JsonNode::JsonValue(JsonValueType::Boolean(false)));
        filled_array.push(JsonNode::JsonValue(JsonValueType::Null));

        assert_eq!(json_array_node, JsonNode::JsonArray(filled_array));
    }

    #[test]
    fn parse_sample_json() {
        let json = r#"
        {
            "name": "Jason",
            "age": 30,
            "isMale": true,
            "height": 1.8,
            "numbers": [1, 2, 3, 4, 5],
            "children": [
                {
                    "name": "Jason Jr.",
                    "age": 5,
                    "isMale": true,
                    "height": 1.2
                },
                {
                    "name": "Jasmine",
                    "age": 3,
                    "isMale": false,
                    "height": 1.1
                }
            ]
        }"#;

        let parsed_json_tree = JsonNode::parse(&json).unwrap();

        let constructed_json_tree = JsonNode::JsonObject(Vec::from([
            ("name".to_owned(), JsonNode::JsonValue(JsonValueType::String("Jason".to_owned()))),
            ("age".to_owned(), JsonNode::JsonValue(JsonValueType::Integer(30))),
            ("isMale".to_owned(), JsonNode::JsonValue(JsonValueType::Boolean(true))),
            ("height".to_owned(), JsonNode::JsonValue(JsonValueType::Float(1.8))),
            ("numbers".to_owned(), JsonNode::JsonArray(vec![
                JsonNode::JsonValue(JsonValueType::Integer(1)),
                JsonNode::JsonValue(JsonValueType::Integer(2)),
                JsonNode::JsonValue(JsonValueType::Integer(3)),
                JsonNode::JsonValue(JsonValueType::Integer(4)),
                JsonNode::JsonValue(JsonValueType::Integer(5))
            ])),
            ("children".to_owned(), JsonNode::JsonArray(vec![
                JsonNode::JsonObject(Vec::from([
                    ("name".to_owned(), JsonNode::JsonValue(JsonValueType::String("Jason Jr.".to_owned()))),
                    ("age".to_owned(), JsonNode::JsonValue(JsonValueType::Integer(5))),
                    ("isMale".to_owned(), JsonNode::JsonValue(JsonValueType::Boolean(true))),
                    ("height".to_owned(), JsonNode::JsonValue(JsonValueType::Float(1.2)))
                ])),
                JsonNode::JsonObject(Vec::from([
                    ("name".to_owned(), JsonNode::JsonValue(JsonValueType::String("Jasmine".to_owned()))),
                    ("age".to_owned(), JsonNode::JsonValue(JsonValueType::Integer(3))),
                    ("isMale".to_owned(), JsonNode::JsonValue(JsonValueType::Boolean(false))),
                    ("height".to_owned(), JsonNode::JsonValue(JsonValueType::Float(1.1)))
                ]))
            ]))
        ]));
        
        assert_eq!(parsed_json_tree, constructed_json_tree);
    }
}
