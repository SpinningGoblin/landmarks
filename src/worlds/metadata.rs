use serde::{Deserialize, Serialize};

use crate::{
    minecraft::{Platform, Seed},
    Tag,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub seed: Seed,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    pub platform: Platform,
    #[serde(default)]
    pub notes: Option<String>,
}

impl From<Seed> for Metadata {
    fn from(value: Seed) -> Self {
        Self {
            name: Some(value.0.clone()),
            seed: value,
            tags: Vec::new(),
            platform: Platform::Bedrock,
            notes: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::minecraft::Seed;

    use super::Metadata;

    #[test]
    pub fn serialize() {
        let seed = Seed("1112222".to_string());
        let metadata = Metadata::from(seed);
        let expected = "{\"seed\":\"1112222\",\"name\":\"1112222\",\"tags\":[],\"platform\":\"Bedrock\",\"notes\":null}";
        let serialized = serde_json::to_string(&metadata).unwrap();
        assert_eq!(expected, serialized);
    }
}
