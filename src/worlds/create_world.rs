use serde::{Deserialize, Serialize};

use crate::{
    minecraft::{Platform, Seed},
    Tag,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateWorld {
    pub seed: Seed,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    pub platform: Platform,
    #[serde(default)]
    pub notes: Option<String>,
}
