use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Deserialize, Serialize, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}

impl Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match *self {
            Dimension::Overworld => "overworld",
            Dimension::Nether => "nether",
            Dimension::End => "end",
        };

        f.write_str(value)
    }
}

impl FromStr for Dimension {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "overworld" => Ok(Dimension::Overworld),
            "nether" => Ok(Dimension::Nether),
            "end" => Ok(Dimension::End),
            _ => Err(anyhow::Error::msg("invalid_dimension")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Dimension;

    #[test]
    pub fn to_string() {
        let dimension = Dimension::Overworld;

        assert_eq!("overworld", dimension.to_string());
    }
}
