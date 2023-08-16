use serde::{Deserialize, Serialize};

use super::{LandmarkLinkType, LandmarkMetadata};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LandmarkLink {
    pub landmark_metadata: LandmarkMetadata,
    pub link_type: Option<LandmarkLinkType>,
}
