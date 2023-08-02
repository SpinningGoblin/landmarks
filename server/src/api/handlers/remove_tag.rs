use serde::Deserialize;

use landmarks_core::Tag;

#[derive(Deserialize, Clone, Debug)]
pub struct RemoveTag {
    pub tag: Tag,
}
