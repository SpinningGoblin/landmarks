use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{landmarks::Landmark, User};

use super::Metadata;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct World {
    pub id: Uuid,
    pub metadata: Metadata,
    pub landmarks: Vec<Landmark>,
    pub creator: User,
}
