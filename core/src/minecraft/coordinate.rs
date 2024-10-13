use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Coordinate {
    pub x: serde_json::Number,
    pub y: serde_json::Number,
    pub z: serde_json::Number,
}
