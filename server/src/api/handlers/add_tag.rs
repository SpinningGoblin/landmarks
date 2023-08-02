use serde::Deserialize;

use landmarks_core::Tag;

#[derive(Deserialize, Clone, Debug)]
pub struct AddTag {
    pub tag: Tag,
}
