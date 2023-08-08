use std::error::Error;
use std::collections::{VecDeque, LinkedList, HashSet, BTreeSet, BinaryHeap, HashMap, BTreeMap};

use crate::{JsonNode, JsonValueType};

pub trait ToJsonNode {
    fn to_json_node(&self) -> JsonNode;
}

impl ToJsonNode for String {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValueType::String(self.clone()))
    }
}

impl ToJsonNode for &str {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValueType::String(self.to_string()))
    }
}

impl ToJsonNode for i32 {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValueType::Integer(i64::from(*self)))
    }
}

impl ToJsonNode for i64 {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValueType::Integer(*self))
    }
}

impl ToJsonNode for f32 {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValueType::Float(f64::from(*self)))
    }
}

impl ToJsonNode for f64 {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValueType::Float(*self))
    }
}

impl ToJsonNode for u32 {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValueType::Integer(i64::from(*self)))
    }
}

impl ToJsonNode for bool {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Value(JsonValueType::Boolean(*self))
    }
}

impl ToJsonNode for Option<String> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValueType::String(value.clone())),
            None => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl ToJsonNode for Option<&str> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValueType::String(value.to_string())),
            None => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl ToJsonNode for Option<i32> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValueType::Integer(i64::from(*value))),
            None => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl ToJsonNode for Option<i64> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValueType::Integer(*value)),
            None => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl ToJsonNode for Option<f32> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValueType::Float(f64::from(*value))),
            None => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl ToJsonNode for Option<f64> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValueType::Float(*value)),
            None => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl ToJsonNode for Option<u32> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValueType::Integer(i64::from(*value))),
            None => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl ToJsonNode for Option<bool> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Some(value) => JsonNode::Value(JsonValueType::Boolean(*value)),
            None => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl<E: Error> ToJsonNode for Result<String, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Value(JsonValueType::String(value.clone())),
            Err(_) => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl<E: Error> ToJsonNode for Result<&str, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Value(JsonValueType::String(value.to_string())),
            Err(_) => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl<E: Error> ToJsonNode for Result<i32, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Value(JsonValueType::Integer(i64::from(*value))),
            Err(_) => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl<E: Error> ToJsonNode for Result<i64, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Value(JsonValueType::Integer(*value)),
            Err(_) => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl<E: Error> ToJsonNode for Result<f32, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Value(JsonValueType::Float(f64::from(*value))),
            Err(_) => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl<E: Error> ToJsonNode for Result<f64, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Value(JsonValueType::Float(*value)),
            Err(_) => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl<E: Error> ToJsonNode for Result<u32, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Value(JsonValueType::Integer(i64::from(*value))),
            Err(_) => JsonNode::Value(JsonValueType::Null),
        }
    }
}

impl<E: Error> ToJsonNode for Result<bool, E> {
    fn to_json_node(&self) -> JsonNode {
        match self {
            Ok(value) => JsonNode::Value(JsonValueType::Boolean(*value)),
            Err(_) => JsonNode::Value(JsonValueType::Null),
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
                .collect::<Vec<(String, JsonNode)>>(),
        )
    }
}

impl<V: ToJsonNode> ToJsonNode for BTreeMap<String, V> {
    fn to_json_node(&self) -> JsonNode {
        JsonNode::Object(
            self.iter()
                .map(|(key, value)| (key.clone(), value.to_json_node()))
                .collect::<Vec<(String, JsonNode)>>(),
        )
    }
}