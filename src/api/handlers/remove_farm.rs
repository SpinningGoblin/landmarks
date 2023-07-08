use serde::Deserialize;

use crate::minecraft::Farm;

#[derive(Clone, Debug, Deserialize)]
pub struct RemoveFarm {
    pub farm: Farm,
}
