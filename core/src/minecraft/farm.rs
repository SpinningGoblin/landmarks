use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::StringVisitor;

#[derive(Clone, Debug)]
pub struct Farm(pub String);

impl Display for Farm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Farm {
    type Err = crate::LandmarksError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Farm(s.to_string()))
    }
}

impl Serialize for Farm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Farm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(StringVisitor).map(Farm)
    }
}
