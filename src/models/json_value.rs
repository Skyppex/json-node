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

    /// Extracts the inner `str` contained inside the node if it is the `JsonValue::String` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let string_value = JsonValue::String("Hello World!".to_owned());
    /// let non_string_value = JsonValue::Null;
    /// 
    /// assert_eq!(string_value.as_string(), Some("Hello World!"));
    /// assert_eq!(non_string_value.as_string(), None);
    /// ```
    pub fn as_string(&self) -> Option<&str> {
        match self {
            JsonValue::String(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `i64` contained inside the node if it is the `JsonValue::Integer` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let integer_value = JsonValue::Integer(42);
    /// let non_integer_value = JsonValue::Null;
    /// 
    /// assert_eq!(integer_value.as_integer(), Some(&42));
    /// assert_eq!(non_integer_value.as_integer(), None);
    /// ```
    pub fn as_integer(&self) -> Option<&i64> {
        match self {
            JsonValue::Integer(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `f64` contained inside the node if it is the `JsonValue::Float` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let float_value = JsonValue::Float(3.14);
    /// let non_float_value = JsonValue::Null;
    /// 
    /// assert_eq!(float_value.as_float(), Some(&3.14));
    /// assert_eq!(non_float_value.as_float(), None);
    /// ```
    pub fn as_float(&self) -> Option<&f64> {
        match self {
            JsonValue::Float(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `bool` contained inside the node if it is the `JsonValue::Boolean` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let bool_value = JsonValue::Boolean(true);
    /// let non_bool_value = JsonValue::Null;
    /// 
    /// assert_eq!(bool_value.as_boolean(), Some(&true));
    /// assert_eq!(non_bool_value.as_boolean(), None);
    /// ```
    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            JsonValue::Boolean(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `mut str` contained inside the node if it is the `JsonValue::String` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let mut string_value = JsonValue::String("Hello World!".to_owned());
    /// let mut non_string_value = JsonValue::Null;
    /// 
    /// assert_eq!(string_value.as_string_mut(), Some("Hello World!".to_string().as_mut_str()));
    /// assert_eq!(non_string_value.as_string_mut(), None);
    /// ```
    pub fn as_string_mut(&mut self) -> Option<&mut str> {
        match self {
            JsonValue::String(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `mut i64` contained inside the node if it is the `JsonValue::Integer` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let mut integer_value = JsonValue::Integer(42);
    /// let mut non_integer_value = JsonValue::Null;
    /// 
    /// assert_eq!(integer_value.as_integer_mut(), Some(&mut 42));
    /// assert_eq!(non_integer_value.as_integer_mut(), None);
    /// ```
    pub fn as_integer_mut(&mut self) -> Option<&mut i64> {
        match self {
            JsonValue::Integer(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `mut f64` contained inside the node if it is the `JsonValue::Float` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let mut float_value = JsonValue::Float(3.14);
    /// let mut non_float_value = JsonValue::Null;
    /// 
    /// assert_eq!(float_value.as_float_mut(), Some(&mut 3.14));
    /// assert_eq!(non_float_value.as_float_mut(), None);
    /// ```
    pub fn as_float_mut(&mut self) -> Option<&mut f64> {
        match self {
            JsonValue::Float(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the inner `mut bool` contained inside the node if it is the `JsonValue::Boolean` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let mut bool_value = JsonValue::Boolean(true);
    /// let mut non_bool_value = JsonValue::Null;
    /// 
    /// assert_eq!(bool_value.as_boolean_mut(), Some(&mut true));
    /// assert_eq!(non_bool_value.as_boolean_mut(), None);
    /// ```
    pub fn as_boolean_mut(&mut self) -> Option<&mut bool> {
        match self {
            JsonValue::Boolean(value) => Some(value),
            _ => None,
        }
    }

    /// Converts the `JsonValue` into a JSON representation of the value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::JsonValue;
    /// 
    /// let string_value = JsonValue::String("Hello World!".to_owned());
    /// 
    /// assert_eq!(string_value.to_json_string(), "\"Hello World!\"");
    /// ```
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