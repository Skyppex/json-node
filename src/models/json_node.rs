use std::collections::HashMap;
use crate::{parsing::JsonNodeParser, utils::SurroundWith};

use super::json_value_type::JsonValueType;

type JsonPropertyDictionary = HashMap<String, JsonNode>;

#[derive(Debug, PartialEq)]
pub enum JsonNode {
    JsonValue(JsonValueType),
    JsonObject(JsonPropertyDictionary),
    JsonArray(Vec<JsonNode>),
}

impl JsonNode {
    pub fn parse(json: &str) -> Result<JsonNode, json::Error> {
        JsonNodeParser::parse_node(json)
    }

    pub fn is_value(&self) -> bool {
        match self {
            JsonNode::JsonValue(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            JsonNode::JsonObject(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            JsonNode::JsonArray(_) => true,
            _ => false,
        }
    }

    pub fn to_json_string(&self) -> String {
        match self {
            JsonNode::JsonValue(value) => value.to_json_string(),
            JsonNode::JsonObject(object) => {
                object
                .iter()
                .map(|(key, node)| format!("\"{}\": {}", key, node.to_json_string()))
                .collect::<Vec<String>>()
                .join(",\n")
                .surround_with("{\n", "}\n")
            },
            JsonNode::JsonArray(array) => {
                array
                .iter()
                .map(|node| node.to_json_string())
                .collect::<Vec<String>>()
                .join(",\n")
                .surround_with("[\n", "]\n")
            },
        }
    }
}
