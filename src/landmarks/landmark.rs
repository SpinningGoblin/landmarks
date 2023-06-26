use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    minecraft::{Coordinate, Farm},
    Tag,
};

use super::Metadata;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Landmark {
    pub id: Uuid,
    pub coordinate: Coordinate,
    pub metadata: Metadata,
    #[serde(default)]
    pub farms: Vec<Farm>,
    #[serde(default)]
    pub tags: Vec<Tag>,
}
