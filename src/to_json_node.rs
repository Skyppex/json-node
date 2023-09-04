use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::error::Error;

use crate::{JsonNode, JsonPropertyMap, JsonValue};

/// A trait for converting a type into a `JsonNode`.
pub trait ToJsonNode {
    /// Converts the type into a `JsonNode`.
    ///
    /// # Implementing the Trait
    ///
    /// ```
    /// use json_node::{JsonNode, JsonValue, JsonPropertyMap, ToJsonNode};
    ///     
    /// // Define some struct you want to convert into a `JsonNode`.
    /// struct Person {
    ///     name: String,
    ///     age: i64,
    /// }
    /// 
    /// // Implement the trait for your struct.
    /// impl ToJsonNode for Person {
    ///     fn to_json_node(&self) -> JsonNode {
    ///         // Create a `JsonNode::Object` with the properties of your struct.
    ///         JsonNode::Object(JsonPropertyMap::from([
    ///             // The key is the name of the property. The value is the value of the property.
    ///             ("name".to_owned(), JsonNode::Value(JsonValue::String(self.name.clone()))),
    ///             // You can convert any type that implements `ToJsonNode` into a `JsonNode`.
    ///             ("age".to_owned(), self.age.to_json_node()),
    ///         ]))
    ///     }
    /// }
    /// 
    /// let person = Person {
    ///     name: "John Doe".to_owned(),
    ///     age: 42,
    /// };
    /// 
    /// let person_node = person.to_json_node();
    /// let person_node_json = person_node.to_json_string();
    /// assert_eq!(
    ///     person_node_json,
    ///     r#"{"name":"John Doe","age":42}"#
    /// );
    /// ```
    fn to_json_node(&self) -> JsonNode;
}

impl ToJsonNode for String {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValue::String(self.clone()))
    }
}

impl ToJsonNode for &str {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValue::String(self.to_string()))
    }
}

impl ToJsonNode for i32 {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValue::Integer(i64::from(*self)))
    }
}

impl ToJsonNode for i64 {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValue::Integer(*self))
    }
}

impl ToJsonNode for f32 {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValue::Float(f64::from(*self)))
    }
}

impl ToJsonNode for f64 {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValue::Float(*self))
    }
}

impl ToJsonNode for u32 {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValue::Integer(i64::from(*self)))
    }
}

impl ToJsonNode for bool {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValue::Boolean(*self))
    }
}

impl ToJsonNode for Option<String> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValue::String(value.clone())),
            None => JsonNode::Value(JsonValue::Null),
        }
    }
}

impl ToJsonNode for Option<&str> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValue::String(value.to_string())),
            None => JsonNode::Value(JsonValue::Null),
        }
    }
}

impl ToJsonNode for Option<i32> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValue::Integer(i64::from(*value))),
            None => JsonNode::Value(JsonValue::Null),
        }
    }
}

impl ToJsonNode for Option<i64> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValue::Integer(*value)),
            None => JsonNode::Value(JsonValue::Null),
        }
    }
}

impl ToJsonNode for Option<f32> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValue::Float(f64::from(*value))),
            None => JsonNode::Value(JsonValue::Null),
        }
    }
}

impl ToJsonNode for Option<f64> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValue::Float(*value)),
            None => JsonNode::Value(JsonValue::Null),
        }
    }
}

impl ToJsonNode for Option<u32> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValue::Integer(i64::from(*value))),
            None => JsonNode::Value(JsonValue::Null),
        }
    }
}

impl ToJsonNode for Option<bool> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValue::Boolean(*value)),
            None => JsonNode::Value(JsonValue::Null),
        }
    }
}

impl<E: Error> ToJsonNode for Result<String, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "ok".to_json_node()),
                ("value".to_string(), value.to_json_node()),
            ])),
            Err(_) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "error".to_json_node()),
                ("error".to_string(), "Could not convert to JSON".to_json_node()),
            ])),
        }
    }
}

impl<E: Error> ToJsonNode for Result<&str, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "ok".to_json_node()),
                ("value".to_string(), value.to_json_node()),
            ])),
            Err(_) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "error".to_json_node()),
                ("error".to_string(), "Could not convert to JSON".to_json_node()),
            ])),
        }
    }
}

impl<E: Error> ToJsonNode for Result<i32, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "ok".to_json_node()),
                ("value".to_string(), value.to_json_node()),
            ])),
            Err(_) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "error".to_json_node()),
                ("error".to_string(), "Could not convert to JSON".to_json_node()),
            ])),
        }
    }
}

impl<E: Error> ToJsonNode for Result<i64, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "ok".to_json_node()),
                ("value".to_string(), value.to_json_node()),
            ])),
            Err(_) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "error".to_json_node()),
                ("error".to_string(), "Could not convert to JSON".to_json_node()),
            ])),
        }
    }
}

impl<E: Error> ToJsonNode for Result<f32, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "ok".to_json_node()),
                ("value".to_string(), value.to_json_node()),
            ])),
            Err(_) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "error".to_json_node()),
                ("error".to_string(), "Could not convert to JSON".to_json_node()),
            ])),
        }
    }
}

impl<E: Error> ToJsonNode for Result<f64, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "ok".to_json_node()),
                ("value".to_string(), value.to_json_node()),
            ])),
            Err(_) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "error".to_json_node()),
                ("error".to_string(), "Could not convert to JSON".to_json_node()),
            ])),
        }
    }
}

impl<E: Error> ToJsonNode for Result<u32, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "ok".to_json_node()),
                ("value".to_string(), value.to_json_node()),
            ])),
            Err(_) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "error".to_json_node()),
                ("error".to_string(), "Could not convert to JSON".to_json_node()),
            ])),
        }
    }
}

impl<E: Error> ToJsonNode for Result<bool, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "ok".to_json_node()),
                ("value".to_string(), value.to_json_node()),
            ])),
            Err(_) => JsonNode::Object(JsonPropertyMap::from([
                ("type".to_string(), "error".to_json_node()),
                ("error".to_string(), "Could not convert to JSON".to_json_node()),
            ])),
        }
    }
}

impl<T: ToJsonNode, const COUNT: usize> ToJsonNode for [T; COUNT] {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Array(self.iter().map(|value| value.to_json_node()).collect())
    }
}

impl<T: ToJsonNode> ToJsonNode for [T] {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Array(self.iter().map(|value| value.to_json_node()).collect())
    }
}

impl<T: ToJsonNode> ToJsonNode for Vec<T> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Array(self.iter().map(|value| value.to_json_node()).collect())
    }
}

impl<T: ToJsonNode> ToJsonNode for VecDeque<T> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Array(self.iter().map(|value| value.to_json_node()).collect())
    }
}

impl<T: ToJsonNode> ToJsonNode for LinkedList<T> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Array(self.iter().map(|value| value.to_json_node()).collect())
    }
}

impl<T: ToJsonNode> ToJsonNode for HashSet<T> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Array(self.iter().map(|value| value.to_json_node()).collect())
    }
}

impl<T: ToJsonNode> ToJsonNode for BTreeSet<T> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Array(self.iter().map(|value| value.to_json_node()).collect())
    }
}

impl<T: ToJsonNode> ToJsonNode for BinaryHeap<T> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Array(self.iter().map(|value| value.to_json_node()).collect())
    }
}

impl<T: ToJsonNode> ToJsonNode for Vec<(String, T)> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Object(
            self.iter()
                .map(|(key, value)| (key.clone(), value.to_json_node()))
                .collect(),
        )
    }
}

impl<T: ToJsonNode> ToJsonNode for VecDeque<(String, T)> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Object(
            self.iter()
                .map(|(key, value)| (key.clone(), value.to_json_node()))
                .collect(),
        )
    }
}

impl<T: ToJsonNode> ToJsonNode for LinkedList<(String, T)> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Object(
            self.iter()
                .map(|(key, value)| (key.clone(), value.to_json_node()))
                .collect(),
        )
    }
}

impl<T: ToJsonNode> ToJsonNode for HashSet<(String, T)> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Object(
            self.iter()
                .map(|(key, value)| (key.clone(), value.to_json_node()))
                .collect(),
        )
    }
}

impl<T: ToJsonNode> ToJsonNode for BTreeSet<(String, T)> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Object(
            self.iter()
                .map(|(key, value)| (key.clone(), value.to_json_node()))
                .collect(),
        )
    }
}

impl<T: ToJsonNode> ToJsonNode for BinaryHeap<(String, T)> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Object(
            self.iter()
                .map(|(key, value)| (key.clone(), value.to_json_node()))
                .collect(),
        )
    }
}

impl<V: ToJsonNode> ToJsonNode for HashMap<String, V> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Object(
            self.iter()
                .map(|(key, value)| (key.clone(), value.to_json_node()))
                .collect::<JsonPropertyMap>(),
        )
    }
}

impl<V: ToJsonNode> ToJsonNode for BTreeMap<String, V> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Object(
            self.iter()
                .map(|(key, value)| (key.clone(), value.to_json_node()))
                .collect::<JsonPropertyMap>(),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::{JsonNode, JsonValue, JsonPropertyMap, ToJsonNode};
        
        // Define some struct you want to convert into a `JsonNode`.
        struct Person {
            name: String,
            age: i64,
        }
        
        // Implement the trait for your struct.
        impl ToJsonNode for Person {
            fn to_json_node(&self) -> JsonNode {
                // Create a `JsonNode::Object` with the properties of your struct.
                JsonNode::Object(JsonPropertyMap::from([
                    // The key is the name of the property. The value is the value of the property.
                    ("name".to_owned(), JsonNode::Value(JsonValue::String(self.name.clone()))),
                    // You can convert any type that implements `ToJsonNode` into a `JsonNode`.
                    ("age".to_owned(), self.age.to_json_node()),
                ]))
            }
        }

        let person = Person {
            name: "John Doe".to_owned(),
            age: 42,
        };

        let person_node = person.to_json_node();
        let person_node_json = person_node.to_json_string();
        assert_eq!(
            person_node_json,
            r#"{"name":"John Doe","age":42}"#
        );
    }
}
