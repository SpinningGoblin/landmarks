use serde::Deserialize;

use crate::Tag;

#[derive(Deserialize, Clone, Debug)]
pub struct AddTag {
    pub tag: Tag,
}
