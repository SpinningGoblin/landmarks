use crate::{
    minecraft::{Biome, Dimension, Farm},
    Tag,
};
use serde::{Deserialize, Serialize};

use super::{LandmarkLink, LandmarkMetadata};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Landmark {
    pub metadata: LandmarkMetadata,
    #[serde(default)]
    pub farms: Vec<Farm>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub biomes: Vec<Biome>,
    pub dimension: Dimension,
    #[serde(default)]
    pub links: Vec<LandmarkLink>,
}
