use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}
