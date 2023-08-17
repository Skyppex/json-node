use crate::{JsonPropertyMap, JsonValue};
use crate::parsing::JsonNodeParser;
use crate::utils::SurroundWith;
use crate::Result;

#[derive(Debug, PartialEq, Clone)]
pub enum JsonNode {
    Value(JsonValue),
    Object(JsonPropertyMap),
    Array(Vec<JsonNode>),
}

impl JsonNode {
    /// Parse a JSON string slice into a `JsonNode` structure.
    /// 
    /// # Arguments
    /// 
    /// * `json` - The JSON you wish to be parsed.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonValue};
    /// 
    /// // Create a valid JSON string.
    /// let json = "10";
    /// // Manually create a tree with the expected structure and value.
    /// let expected = JsonNode::Value(JsonValue::Integer(10));
    /// 
    /// // Parse the json string into a node tree.
    /// let node_tree = JsonNode::parse(json).unwrap();
    /// 
    /// assert_eq!(node_tree, expected);
    /// ```
    pub fn parse(json: &str) -> Result<JsonNode> {
        JsonNodeParser::parse_node(json, None)
    }

    /// Checks if the node is the `JsonNode::Value` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonValue};
    /// 
    /// // Create a value node.
    /// let value_node = JsonNode::Value(JsonValue::Boolean(true));
    /// // Create a non-value node.
    /// let non_value_node = JsonNode::Array(Vec::new());
    /// 
    /// assert!(value_node.is_value());
    /// assert!(!non_value_node.is_value())
    /// ```
    /// 
    /// # Remarks
    /// 
    /// Note that this function with return true even if the value type is `JsonValue::Null`.
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonValue};
    /// 
    /// let null_value_node = JsonNode::Value(JsonValue::Null);
    /// assert!(null_value_node.is_value());
    /// ```
    pub fn is_value(&self) -> bool {
        match self {
            JsonNode::Value(_) => true,
            _ => false,
        }
    }

    /// Checks if the node is the JsonNode::Object discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonValue, JsonPropertyMap};
    /// 
    /// // Create an object node.
    /// let object_node = JsonNode::Object(JsonPropertyMap::new());
    /// // Create a non-object node.
    /// let non_object_node = JsonNode::Value(JsonValue::Null);
    /// 
    /// assert!(object_node.is_value());
    /// assert!(!non_object_node.is_value())
    /// ```
    pub fn is_object(&self) -> bool {
        match self {
            JsonNode::Object(_) => true,
            _ => false,
        }
    }

    /// Checks if the node is the JsonNode::Array discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonValue};
    /// // Create an array node.
    /// let array_node = JsonNode::Array(Vec::new());
    /// // Create a non-array node.
    /// let non_array_node = JsonNode::Value(JsonValue::Null);
    /// 
    /// assert!(array_node.is_value());
    /// assert!(!non_array_node.is_value())
    /// ```
    pub fn is_array(&self) -> bool {
        match self {
            JsonNode::Array(_) => true,
            _ => false,
        }
    }

    /// Extracts the `JsonValue` contained inside the node if it is the `JsonNode::Value` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonValue};
    /// 
    /// // Create a value node.
    /// let value_node = JsonNode::Value(JsonValue::Integer(42));
    /// 
    /// // Extract `JsonValue`.
    /// let as_value_some = value_node.as_value(); // Option<&JsonValue>
    /// 
    /// assert_eq!(as_value_some, Some(&JsonValue::Integer(42)));
    /// 
    /// // Create a non-value node.
    /// let non_value_node = JsonNode::Array(Vec::new());
    /// 
    /// // Fail to extract `JsonValue`.
    /// let as_value_none = non_value_node.as_value();
    /// 
    /// assert_eq!(as_value_none, None);
    /// ```
    pub fn as_value(&self) -> Option<&JsonValue> {
        match self {
            JsonNode::Value(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the `JsonPropertyMap` contained inside the node if it is the `JsonNode::Object` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonValue, JsonPropertyMap};
    /// 
    /// // Create an object node.
    /// let object_node = JsonNode::Object(JsonPropertyMap::new());
    /// 
    /// // Extract `JsonPropertyMap`.
    /// let as_object_some = object_node.as_object(); // Option<&JsonPropertyMap>
    /// 
    /// assert_eq!(as_object_some, Some(&JsonPropertyMap::new()));
    /// 
    /// // Create a non-object node.
    /// let non_object_node = JsonNode::Value(JsonValue::Null);
    /// 
    /// // Fail to extract `JsonPropertyMap`.
    /// let as_object_none = non_object_node.as_object();
    /// 
    /// assert_eq!(as_object_none, None);
    /// ```
    pub fn as_object(&self) -> Option<&JsonPropertyMap> {
        match self {
            JsonNode::Object(object) => Some(object),
            _ => None,
        }
    }

    /// Extracts the `Vec<JsonNode>` contained inside the node if it is the `JsonNode::Array` discriminant.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonValue};
    /// 
    /// // Create an array node.
    /// let array_node = JsonNode::Array(Vec::new());
    /// 
    /// // Extract `Vec<JsonNode>`.
    /// let as_array_some = array_node.as_array(); // Option<&Vec<JsonNode>>
    /// 
    /// assert_eq!(as_array_some, Some(&Vec::new()));
    /// 
    /// // Create a non-array node.
    /// let non_array_node = JsonNode::Value(JsonValue::Null);
    /// 
    /// // Fail to extract `Vec<JsonNode>`.
    /// let as_array_none = non_array_node.as_array();
    /// 
    /// assert_eq!(as_array_none, None);
    /// ```
    pub fn as_array(&self) -> Option<&Vec<JsonNode>> {
        match self {
            JsonNode::Array(array) => Some(array),
            _ => None,
        }
    }

    /// Extracts the `JsonValue` contained inside the node if it is the `JsonNode::Value` discriminant as a mutable value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonValue};
    /// 
    /// // Create a value node.
    /// let mut value_node = JsonNode::Value(JsonValue::Integer(42));
    /// 
    /// // Extract `JsonValue`.
    /// let as_value_some = value_node.as_value_mut(); // Option<&mut JsonValue>
    /// 
    /// assert_eq!(as_value_some, Some(&mut JsonValue::Integer(42)));
    /// 
    /// // Create a non-value node.
    /// let mut non_value_node = JsonNode::Array(Vec::new());
    /// 
    /// // Fail to extract `JsonValue`.
    /// let as_value_none = non_value_node.as_value_mut();
    /// 
    /// assert_eq!(as_value_none, None);
    /// ```
    pub fn as_value_mut(&mut self) -> Option<&mut JsonValue> {
        match self {
            JsonNode::Value(value) => Some(value),
            _ => None,
        }
    }

    /// Extracts the `JsonPropertyMap` contained inside the node if it is the `JsonNode::Object` discriminant as a mutable value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonValue, JsonPropertyMap};
    /// 
    /// // Create an object node.
    /// let mut object_node = JsonNode::Object(JsonPropertyMap::new());
    /// 
    /// // Extract `JsonPropertyMap`.
    /// let as_object_some = object_node.as_object_mut(); // Option<&mut JsonPropertyMap>
    /// 
    /// assert_eq!(as_object_some, Some(&mut JsonPropertyMap::new()));
    /// 
    /// // Create a non-object node.
    /// let mut non_object_node = JsonNode::Value(JsonValue::Null);
    /// 
    /// // Fail to extract `JsonPropertyMap`.
    /// let as_object_none = non_object_node.as_object_mut();
    /// 
    /// assert_eq!(as_object_none, None);
    /// ```
    pub fn as_object_mut(&mut self) -> Option<&mut JsonPropertyMap> {
        match self {
            JsonNode::Object(object) => Some(object),
            _ => None,
        }
    }

    /// Extracts the `Vec<JsonNode>` contained inside the node if it is the `JsonNode::Array` discriminant as a mutable value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonValue, JsonPropertyMap};
    /// 
    /// // Create an array node.
    /// let mut array_node = JsonNode::Array(Vec::new());
    /// 
    /// // Extract `Vec<JsonNode>`.
    /// let as_array_some = array_node.as_array_mut(); // Option<&mut Vec<JsonNode>>
    /// 
    /// assert_eq!(as_array_some, Some(&mut Vec::new()));
    /// 
    /// // Create a non-array node.
    /// let mut non_array_node = JsonNode::Value(JsonValue::Null);
    /// 
    /// // Fail to extract `JsonPropertyMap`.
    /// let as_array_none = non_array_node.as_array_mut();
    /// 
    /// assert_eq!(as_array_none, None);
    /// ```
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<JsonNode>> {
        match self {
            JsonNode::Array(array) => Some(array),
            _ => None,
        }
    }

    /// Convert the node tree to a JSON string.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use json_node::{JsonNode, JsonValue};
    /// 
    /// // Create a JsonNode tree.
    /// let node_tree = JsonNode::Array(Vec::from([
    ///     JsonNode::Value(JsonValue::Integer(0)),
    ///     JsonNode::Value(JsonValue::Float(0.5)),
    ///     JsonNode::Value(JsonValue::Integer(1)),
    ///     JsonNode::Value(JsonValue::Null),
    ///     JsonNode::Value(JsonValue::Boolean(false))
    /// ]));
    /// 
    /// let json_string = node_tree.to_json_string();
    /// 
    /// assert_eq!(json_string, "[0,0.5,1,null,false]".to_owned());
    /// ```
    /// 
    /// # Remarks
    /// 
    /// This function does zero formatting. The entire JSON string is returned without any spaces or new-lines.
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

    /// Turns the node tree into an iterator which iterates over evey `JsonValue` in the tree in a depth first manner.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use crate::{JsonNode, JsonValue};
    ///     
    /// let node_tree = JsonNode::Array(Vec::from([
    ///     JsonNode::Array(Vec::from([                     // First element is an array with the value `1` inside.
    ///         JsonNode::Value(JsonValue::Integer(1)),
    ///     ])),
    ///     JsonNode::Value(JsonValue::Integer(2)),         // Second element is the value `2`.
    ///     JsonNode::Array(Vec::from([
    ///         JsonNode::Value(JsonValue::Integer(3))      // Third element is an array with the value `3` inside.
    ///     ]))
    /// ]));
    /// 
    /// let sequence = node_tree.into_iter().collect::<Vec<&JsonNode>>();
    /// 
    /// let expected = vec![
    ///     &JsonNode::Value(JsonValue::Integer(1)),
    ///     &JsonNode::Value(JsonValue::Integer(2)),
    ///     &JsonNode::Value(JsonValue::Integer(3))
    /// ];
    /// 
    /// assert_eq!(sequence, expected);
    /// ```
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
            return None; // Termination point for iteration. If the iterator has recursed, this allows the parent iterator to continue.
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
                            self.node = None;
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
    use crate::JsonValue;
    use super::JsonNode;

    #[test]
    fn json_node_is_value() {
        let mut node = JsonNode::Value(JsonValue::String("Hello, World!".to_string()));
        assert!(node.is_value());

        node = JsonNode::Value(JsonValue::Integer(123));
        assert!(node.is_value());

        node = JsonNode::Value(JsonValue::Float(123.456));
        assert!(node.is_value());

        node = JsonNode::Value(JsonValue::Boolean(true));
        assert!(node.is_value());

        node = JsonNode::Value(JsonValue::Null);
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

#[cfg(test)]
mod doc_tests{

    #[test]
    fn parse_doc() {
        use super::{JsonNode, JsonValue};
        
        // Create a valid JSON string.
        let json = "10";
        // Manually create a tree with the expected structure and value.
        let expected = JsonNode::Value(JsonValue::Integer(10));

        // Parse the json string into a node tree.
        let node_tree = JsonNode::parse(json).unwrap();
        
        assert_eq!(node_tree, expected);
    }

    #[test]
    fn into_iter() {
        use crate::{JsonNode, JsonValue};
        
        let node_tree = JsonNode::Array(Vec::from([
            JsonNode::Array(Vec::from([                     // First element is an array with the value `1` inside.
                JsonNode::Value(JsonValue::Integer(1)),
            ])),
            JsonNode::Value(JsonValue::Integer(2)),         // Second element is the value `2`.
            JsonNode::Array(Vec::from([
                JsonNode::Value(JsonValue::Integer(3))      // Third element is an array with the value `3` inside.
            ]))
        ]));
        
        let sequence = node_tree.into_iter().collect::<Vec<&JsonNode>>();

        let expected = vec![
            &JsonNode::Value(JsonValue::Integer(1)),
            &JsonNode::Value(JsonValue::Integer(2)),
            &JsonNode::Value(JsonValue::Integer(3))
        ];

        assert_eq!(sequence, expected);
    }
}
