use crate::{
    minecraft::{Platform, Seed},
    users::User,
    Tag,
};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
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
    pub creator: User,
    pub shared_users: Vec<User>,
    pub updated_at: Option<OffsetDateTime>,
}
