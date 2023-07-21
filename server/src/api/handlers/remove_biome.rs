use serde::Deserialize;

use crate::minecraft::Biome;

#[derive(Clone, Debug, Deserialize)]
pub struct RemoveBiome {
    pub biome: Biome,
}
