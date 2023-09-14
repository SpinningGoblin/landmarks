use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::StringVisitor;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Tag(pub String);

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Tag {
    type Err = crate::LandmarksError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tag(s.to_string()))
    }
}

impl Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(StringVisitor).map(Tag)
    }
}

#[cfg(test)]
mod tests {
    use super::Tag;

    #[derive(serde::Deserialize, serde::Serialize)]
    struct TestWrapper {
        tag: Tag,
    }

    #[test]
    fn serialization() {
        let tag = Tag("base".to_string());
        let wrapper = TestWrapper { tag };

        let serialized = serde_json::to_string(&wrapper).unwrap();
        assert_eq!("{\"tag\":\"base\"}", &serialized);
    }

    #[test]
    fn deserialization() {
        let serialized = "{\"tag\":\"base\"}";

        let wrapper: TestWrapper = serde_json::from_str(serialized).unwrap();
        assert_eq!(Tag("base".to_string()), wrapper.tag);
    }
}
