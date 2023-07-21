use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Deserialize, Serialize, EnumIter, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Bedrock,
    Java,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match *self {
            Platform::Bedrock => "bedrock",
            Platform::Java => "java",
        };

        f.write_str(value)
    }
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bedrock" => Ok(Platform::Bedrock),
            "java" => Ok(Platform::Java),
            _ => Err(anyhow::Error::msg("invalid_platform")),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::minecraft::Platform;

    #[test]
    pub fn from_str() {
        let text = "bedrock";
        assert_eq!(Platform::Bedrock, Platform::from_str(text).unwrap());
    }
}
