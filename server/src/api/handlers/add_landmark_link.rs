use landmarks_core::landmarks::LandmarkLinkType;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize)]
pub struct AddLandmarkLink {
    pub landmark_id_1: Uuid,
    pub landmark_id_2: Uuid,
    #[serde(default)]
    pub link_type: Option<LandmarkLinkType>,
}
