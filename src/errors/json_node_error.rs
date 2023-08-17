use std::{error::Error, fmt::Display};

pub type Result<T> = std::result::Result<T, JsonNodeError>;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonNodeError {
    EmptyJsonNode(Option<String>),
    CouldntParseNode(String),
    MultiplePropertiesWithSameKey(String),
    KeyNotFound(String),
}

impl Display for JsonNodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonNodeError::EmptyJsonNode(parent_node) => {
                if let Some(parent_node) = parent_node {
                    return write!(f, "{}", parent_node);
                }
                
                write!(f, "{}", "Json node has no parent".to_string())
            },
            JsonNodeError::CouldntParseNode(node) => write!(f, "{}", node),
            JsonNodeError::MultiplePropertiesWithSameKey(key) => write!(f, "{}", key),
            JsonNodeError::KeyNotFound(key) => write!(f, "{}", key),
        }
    }
}

impl Error for JsonNodeError { }