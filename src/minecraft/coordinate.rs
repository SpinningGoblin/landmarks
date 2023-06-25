use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Coordinate {
    pub x: String,
    pub y: String,
    pub z: String,
}
