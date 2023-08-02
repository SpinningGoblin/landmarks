use crate::{
    minecraft::{Platform, Seed},
    Tag,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WorldMetadata {
    pub id: Uuid,
    pub seed: Seed,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    pub platform: Platform,
    #[serde(default)]
    pub notes: Option<String>,
    pub creator: String,
}
