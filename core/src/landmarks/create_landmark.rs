use crate::{
    minecraft::{Biome, Coordinate, Dimension, Farm},
    Tag,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateLandmark {
    pub name: String,
    pub coordinate: Coordinate,
    #[serde(default)]
    pub biomes: Vec<Biome>,
    pub dimension: Dimension,
    #[serde(default)]
    pub farms: Vec<Farm>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub notes: Option<String>,
}
