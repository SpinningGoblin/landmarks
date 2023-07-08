use serde::Deserialize;

use crate::minecraft::Farm;

#[derive(Clone, Debug, Deserialize)]
pub struct AddFarm {
    pub farm: Farm,
}
