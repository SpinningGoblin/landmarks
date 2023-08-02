use crate::{
    minecraft::{Platform, Seed},
    Tag,
};
use serde::{Deserialize, Serialize};

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

impl CreateWorld {
    pub fn guaranteed_name(&self) -> String {
        self.name.clone().unwrap_or(self.seed.to_string())
    }
}
