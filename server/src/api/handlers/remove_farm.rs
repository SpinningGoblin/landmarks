use serde::Deserialize;

use landmarks_core::minecraft::Farm;

#[derive(Clone, Debug, Deserialize)]
pub struct RemoveFarm {
    pub farm: Farm,
}
