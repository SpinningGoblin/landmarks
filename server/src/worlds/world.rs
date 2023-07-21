use serde::Serialize;

use crate::landmarks::Landmark;

use super::WorldMetadata;

#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub metadata: WorldMetadata,
    pub landmarks: Vec<Landmark>,
}
