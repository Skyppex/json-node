use std::collections::HashMap;

use crate::models::{JsonNode, JsonValueType};

use super::tokens;

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
            let no_brackets = trim[1..trim.len() - 2].trim();

            let array_values = no_brackets.split(tokens::COMMA)
            .map(|value| {
                Self::parse_node(value)
                .ok()
            })
            .collect::<Vec::<Option<JsonNode>>>();

            if array_values.iter().any(|value| value.is_none()) {
                return None;
            }

            let nodes = array_values
                .into_iter()
                .map(|n| n.unwrap())
                .collect();

            return Some(JsonNode::JsonArray(nodes));
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
                return Some(JsonNode::JsonObject(HashMap::new()));
            }

            let split = no_braces.split(tokens::COMMA);
            let elements: Vec<&str> = split.collect();

            let kvps = elements.iter()
            .map(|property| {
                let (key, value) = property.split_once(tokens::COLON).unwrap();
                (key.to_owned(), Self::parse_node(value).ok())
            })
            .collect::<Vec<(String, Option<JsonNode>)>>();

            let mut objects = HashMap::new();

            for (k, v) in kvps.into_iter() {
                if v.is_none() {
                    return None;
                }

                objects.insert(k, v.unwrap());
            }

            return Some(JsonNode::JsonObject(objects));
        }

        None
    }
}




#[cfg(test)]
mod tests {
    use std::collections::HashMap;
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
        assert_eq!(json_node, JsonNode::JsonObject(HashMap::new()));
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
                assert_eq!(map, filled_map);
            },
            _ => panic!("Expected JsonObject")
        }

        // assert_eq!(json_object_node, JsonNode::JsonObject(filled_map));
    }
}
