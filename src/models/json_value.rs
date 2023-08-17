use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

impl JsonValue {
    /// Checks if the value is the `JsonValue::String` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let string_value = JsonValue::String("Hello World!".to_owned());
    /// let non_string_value = JsonValue::Null;
    /// 
    /// assert!(string_value.is_string());
    /// assert!(!non_string_value.is_string());
    /// ```
    pub fn is_string(&self) -> bool {
        match self {
            JsonValue::String(_) => true,
            _ => false,
        }
    }

    /// Checks if the value is the `JsonValue::Integer` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let integer_value = JsonValue::Integer(42);
    /// let non_integer_value = JsonValue::Null;
    /// 
    /// assert!(integer_value.is_integer());
    /// assert!(!non_integer_value.is_integer());
    /// ```
    pub fn is_integer(&self) -> bool {
        match self {
            JsonValue::Integer(_) => true,
            _ => false,
        }
    }

    /// Checks if the value is the `JsonValue::Float` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let float_value = JsonValue::Float(3.14);
    /// let non_float_value = JsonValue::Null;
    /// 
    /// assert!(float_value.is_float());
    /// assert!(!non_float_value.is_float());
    /// ```
    pub fn is_float(&self) -> bool {
        match self {
            JsonValue::Float(_) => true,
            _ => false,
        }
    }

    /// Checks if the value is the `JsonValue::Boolean` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let bool_value = JsonValue::Boolean(true);
    /// let non_bool_value = JsonValue::Null;
    /// 
    /// assert!(bool_value.is_bool());
    /// assert!(!non_bool_value.is_bool());
    /// ```
    pub fn is_bool(&self) -> bool {
        match self {
            JsonValue::Boolean(_) => true,
            _ => false,
        }
    }

    /// Checks if the value is the `JsonValue::Null` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let null_value = JsonValue::Null;
    /// let non_null_value = JsonValue::Integer(42);
    /// 
    /// assert!(null_value.is_null());
    /// assert!(!non_null_value.is_null());
    /// ```
    pub fn is_null(&self) -> bool {
        match self {
            JsonValue::Null => true,
            _ => false,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            JsonValue::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<&i64> {
        match self {
            JsonValue::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<&f64> {
        match self {
            JsonValue::Float(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            JsonValue::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string_mut(&mut self) -> Option<&mut str> {
        match self {
            JsonValue::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_integer_mut(&mut self) -> Option<&mut i64> {
        match self {
            JsonValue::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_float_mut(&mut self) -> Option<&mut f64> {
        match self {
            JsonValue::Float(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_boolean_mut(&mut self) -> Option<&mut bool> {
        match self {
            JsonValue::Boolean(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_json_string(&self) -> String {
        match self {
            JsonValue::String(value) => format!("\"{}\"", value),
            JsonValue::Integer(value) => value.to_string(),
            JsonValue::Float(value) => value.to_string(),
            JsonValue::Boolean(value) => value.to_string(),
            JsonValue::Null => String::from("null"),
        }
    }
}

impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::String(value) => write!(f, "{}", value),
            JsonValue::Integer(value) => write!(f, "{}", value),
            JsonValue::Float(value) => write!(f, "{}", value),
            JsonValue::Boolean(value) => write!(f, "{}", value),
            JsonValue::Null => write!(f, "null"),
        }
    }
}