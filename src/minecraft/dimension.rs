use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Deserialize, Serialize, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}
