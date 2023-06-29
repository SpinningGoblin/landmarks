use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Coordinate {
    pub x: i128,
    pub y: i128,
    pub z: i128,
}
