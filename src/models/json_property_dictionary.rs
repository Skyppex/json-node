use std::ops::{Index, IndexMut};

use crate::{JsonNode, JsonNodeError};

#[derive(Debug, Clone, PartialEq)]
pub struct JsonPropertyDictionary(Vec<(String, JsonNode)>);

impl JsonPropertyDictionary {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn get(&self, key: &str) -> Option<&JsonNode> {
        self.0.iter()
              .find(|(k, _)| k == key)
              .map(|(_, v)| v)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut JsonNode> {
        self.0.iter_mut()
              .find(|(k, _)| k == key)
              .map(|(_, v)| v)
    }

    pub fn add(&mut self, key: String, value: JsonNode) {
        if self.contains_key(&key) {
            return;
        }

        self.0.push((key, value));
    }

    pub fn remove(&mut self, key: &str) -> crate::Result<JsonNode> {
        if self.0.iter().filter(|(k, _)| k == key).count() > 1 {
            return Err(JsonNodeError::MultiplePropertiesWithSameKey(key.to_string()))
        }

        self.0.iter()
              .position(|(k, _)| k == key)
              .map(|i| self.0.remove(i).1)
              .ok_or(JsonNodeError::KeyNotFound(key.to_string()))
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.0.iter().any(|(k, _)| k == key)
    }

    pub fn keys(&self) -> Vec<&String> {
        self.0.iter().map(|(k, _)| k).collect()
    }

    pub fn values(&self) -> Vec<&JsonNode> {
        self.0.iter().map(|(_, v)| v).collect()
    }

    pub fn values_mut(&mut self) -> Vec<&mut JsonNode> {
        self.0.iter_mut().map(|(_, v)| v).collect()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter(&self) -> std::slice::Iter<(String, JsonNode)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<(String, JsonNode)> {
        self.0.iter_mut()
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn to_json_string(&self) -> String {
        let mut result = "{".to_string();

        for (key, value) in &self.0 {
            result.push_str(&format!("\"{}\": {},", key, value.to_json_string()));
        }

        result.pop(); // Pops the trailing comma
        result.push('}');

        result
    }
}

impl Index<usize> for JsonPropertyDictionary {
    type Output = (String, JsonNode);

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for JsonPropertyDictionary {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl FromIterator<(String, JsonNode)> for JsonPropertyDictionary {
    fn from_iter<T: IntoIterator<Item = (String, JsonNode)>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl From<Vec<(String, JsonNode)>> for JsonPropertyDictionary {
    fn from(value: Vec<(String, JsonNode)>) -> Self {
        Self(value)
    }
}

impl<const COUNT: usize> From<[(String, JsonNode); COUNT]> for JsonPropertyDictionary {
    fn from(value: [(String, JsonNode); COUNT]) -> Self {
        Self(value.to_vec())
    }
}
