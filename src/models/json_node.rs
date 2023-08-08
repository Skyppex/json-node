use crate::JsonNodeError;
use crate::parsing::JsonNodeParser;
use crate::utils::SurroundWith;

use crate::models::JsonValueType;

type JsonPropertyDictionary = Vec<(String, JsonNode)>;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonNode {
    Value(JsonValueType),
    Object(JsonPropertyDictionary),
    Array(Vec<JsonNode>),
}

impl JsonNode {
    pub fn parse(json: &str) -> Result<JsonNode, JsonNodeError> {
        JsonNodeParser::parse_node(json, None)
    }

    pub fn is_value(&self) -> bool {
        match self {
            JsonNode::Value(_) => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            JsonNode::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            JsonNode::Array(_) => true,
            _ => false,
        }
    }

    pub fn as_value(&self) -> Option<&JsonValueType> {
        match self {
            JsonNode::Value(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&JsonPropertyDictionary> {
        match self {
            JsonNode::Object(object) => Some(object),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&Vec<JsonNode>> {
        match self {
            JsonNode::Array(array) => Some(array),
            _ => None,
        }
    }

    pub fn as_value_mut(&mut self) -> Option<&mut JsonValueType> {
        match self {
            JsonNode::Value(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_object_mut(&mut self) -> Option<&mut JsonPropertyDictionary> {
        match self {
            JsonNode::Object(object) => Some(object),
            _ => None,
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<JsonNode>> {
        match self {
            JsonNode::Array(array) => Some(array),
            _ => None,
        }
    }

    pub fn to_json_string(&self) -> String {
        match self {
            JsonNode::Value(value) => value.to_json_string(),
            JsonNode::Object(object) => {
                object
                .iter()
                .map(|(key, node)| format!("\"{}\": {}", key, node.to_json_string()))
                .collect::<Vec<String>>()
                .join(",")
                .surround_with("{", "}")
            },
            JsonNode::Array(array) => {
                array
                .iter()
                .map(|node| node.to_json_string())
                .collect::<Vec<String>>()
                .join(",")
                .surround_with("[", "]")
            },
        }
    }
}

impl<'a> IntoIterator for &'a JsonNode {
    type Item = &'a JsonNode;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            node: Some(self),
            array_index: None,
            object_index: None,
            child: None,
        }
    }
}

pub struct Iter<'a> {
    node: Option<&'a JsonNode>,
    array_index: Option<usize>,
    object_index: Option<usize>,
    child: Option<Box<Iter<'a>>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a JsonNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = &mut self.child {
            if let Some(node) = iter.next() {
                return Some(node);
            }

            self.child = None;
        }

        if let None = self.node {
            return None; // Termination point for iteration
        }

        match self.node.unwrap() {
            JsonNode::Array(nodes) => {
                match self.array_index {
                    Some(mut index) => {
                        index = index + 1;
                        self.array_index = Some(index);
                        self.child = Some(Box::new(nodes[index].into_iter()));
                        let next = self.next();

                        if index == nodes.len() - 1 {
                            self.array_index = None;
                            self.node = None;
                        }

                        return next;
                    },
                    None => {
                        self.array_index = Some(0);
                        self.child = Some(Box::new(nodes[0].into_iter()));
                        let next = self.next();

                        if nodes.len() == 1 {
                            self.array_index = None;
                        }

                        return next;
                    },
                }
            },
            JsonNode::Object(properties) => {
                match self.object_index {
                    Some(mut index) => {
                        index = index + 1;
                        self.object_index = Some(index);
                        self.child = Some(Box::new(properties[index].1.into_iter()));
                        let next = self.next();

                        if index == properties.len() - 1 {
                            self.object_index = None;
                            self.node = None;
                        }

                        return next;
                    },
                    None => {
                        self.object_index = Some(0);
                        self.child = Some(Box::new(properties[0].1.into_iter()));
                        let next = self.next();

                        if properties.len() == 1 {
                            self.object_index = None;
                        }

                        return next;
                    },
                }
            },
            JsonNode::Value(_) => {
                let node = self.node.unwrap();
                self.node = None;
                Some(node)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::JsonValueType;
    use super::JsonNode;

    #[test]
    fn json_node_is_value() {
        let mut node = JsonNode::Value(JsonValueType::String("Hello, World!".to_string()));
        assert!(node.is_value());

        node = JsonNode::Value(JsonValueType::Integer(123));
        assert!(node.is_value());

        node = JsonNode::Value(JsonValueType::Float(123.456));
        assert!(node.is_value());

        node = JsonNode::Value(JsonValueType::Boolean(true));
        assert!(node.is_value());

        node = JsonNode::Value(JsonValueType::Null);
        assert!(node.is_value());
    }

    #[test]
    fn iterate_works() {
        let json = r#"
        {
            "name": "Jason",
            "age": 30,
            "isMale": true,
            "height": 1.8,
            "numbers": [1, 2, 3, 4, 5],
            "children": [
                {
                    "name": "Jason Jr.",
                    "age": 5,
                    "isMale": true,
                    "height": 1.2
                },
                {
                    "name": "Jasmine",
                    "age": 3,
                    "isMale": false,
                    "height": 1.1
                }
            ]
        }"#;

        let node = JsonNode::parse(json).unwrap();
        for e in node.into_iter() {
            println!("{:?}", e)
        }
    }
}
