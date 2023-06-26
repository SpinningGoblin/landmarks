use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::StringVisitor;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Tag(pub String);

#[allow(dead_code)]
impl Tag {
    pub fn eq_ignore_ascii_case(&self, other: &Tag) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
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
