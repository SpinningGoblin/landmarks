use serde::Deserialize;

use crate::{
    minecraft::{Dimension, Farm},
    serialization::{comma_separated_list, empty_string_as_none},
    Tag,
};

#[derive(Debug, Deserialize)]
pub struct LandmarkFilters {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub dimension: Option<Dimension>,
    #[serde(default, deserialize_with = "comma_separated_list")]
    pub tags: Vec<Tag>,
    #[serde(default, deserialize_with = "comma_separated_list")]
    pub farms: Vec<Farm>,
}
