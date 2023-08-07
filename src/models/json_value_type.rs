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