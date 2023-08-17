use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValueType {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl JsonValueType {
    pub fn is_string(&self) -> bool {
        match self {
            JsonValueType::String(_) => true,
            _ => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match self {
            JsonValueType::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            JsonValueType::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            JsonValueType::Boolean(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            JsonValueType::Null => true,
            _ => false,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            JsonValueType::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<&i64> {
        match self {
            JsonValueType::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<&f64> {
        match self {
            JsonValueType::Float(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            JsonValueType::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_json_string(&self) -> String {
        match self {
            JsonValueType::String(value) => format!("\"{}\"", value),
            JsonValueType::Integer(value) => value.to_string(),
            JsonValueType::Float(value) => value.to_string(),
            JsonValueType::Boolean(value) => value.to_string(),
            JsonValueType::Null => String::from("null"),
        }
    }
}

impl Display for JsonValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValueType::String(value) => write!(f, "{}", value),
            JsonValueType::Integer(value) => write!(f, "{}", value),
            JsonValueType::Float(value) => write!(f, "{}", value),
            JsonValueType::Boolean(value) => write!(f, "{}", value),
            JsonValueType::Null => write!(f, "null"),
        }
    }
}