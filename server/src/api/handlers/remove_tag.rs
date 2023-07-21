use serde::Deserialize;

use crate::Tag;

#[derive(Deserialize, Clone, Debug)]
pub struct RemoveTag {
    pub tag: Tag,
}
