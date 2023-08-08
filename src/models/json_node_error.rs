use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Clone)]
pub enum JsonNodeError {
    EmptyJsonNode(Option<String>),
    CouldntParseNode(String),
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
        }
    }
}

impl Error for JsonNodeError { }