use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::minecraft::Coordinate;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LandmarkMetadata {
    pub id: Uuid,
    pub coordinate: Coordinate,
    pub name: String,
    pub notes: Option<String>,
}
