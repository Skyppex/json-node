# Json Node
A way to work with JSON as a node tree.

## Installation
Navigate to your project folder and run:
```
cargo add json-node
```
Now you're set

## Usage
Use the `JsonNode` type to parse your JSON into nodes. There are three types of nodes: `Object`, `Array` and `JsonValue`. A `Value` can have several different types of values: `String`, `Integer`, `Float`, `Boolean`, `Null`. These are all modeled with enums to ensure type safety, even when a `Array` contains different types of `Value`s.
After parsing you can dig for what you need manually using the `match` statement or `if let` as you would with enums normally. Or you can iterate through every value under a node by calling `into_iter()`.

```rust
use json_node::{JsonNode, JsonValueType};
let json = "[1, 2, 3, 4, 5]";
let tree = JsonNode::parse(json).unwrap();

match tree {
    JsonNode::Array(arr) => {
        for element in arr.iter() {
            println!("{:?}", element);
        }
    }
    _ => println!("Expected node to be an Array.")
}
```
OR
```rust
use json_node::{JsonNode, JsonValueType};
let json = "[1, 2, 3, 4, 5]";
let tree = JsonNode::parse(json).unwrap();

for value in tree.into_iter() {
    match value {
        JsonNode::Value(value) => {
            match value {
                JsonValueType::Integer(num) => println!("{num}");
                _ => println!("Expected integer value.")
            }
        }
        _ => println!("Expected value.")
    }
}
```
They both output:
```
1
2
3
4
5
```

You can also use this API to build a JSON string from nodes if needed. Although it won't be nicely formatted.