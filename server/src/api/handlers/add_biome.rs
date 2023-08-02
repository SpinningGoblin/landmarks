use serde::Deserialize;

use landmarks_core::minecraft::Biome;

#[derive(Clone, Debug, Deserialize)]
pub struct AddBiome {
    pub biome: Biome,
}
