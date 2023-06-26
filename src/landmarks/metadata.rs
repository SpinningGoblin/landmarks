use serde::{Deserialize, Serialize};

use crate::minecraft::{Biome, Dimension};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Metadata {
    #[serde(default)]
    pub biomes: Vec<Biome>,
    pub dimension: Dimension,
}
