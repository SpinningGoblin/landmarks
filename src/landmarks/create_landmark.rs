use serde::{Deserialize, Serialize};

use crate::{
    minecraft::{Biome, Coordinate, Dimension, Farm},
    Tag,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateLandmark {
    pub coordinate: Coordinate,
    #[serde(default)]
    pub biomes: Vec<Biome>,
    pub dimension: Dimension,
    pub farms: Vec<Farm>,
    pub tags: Vec<Tag>,
}
