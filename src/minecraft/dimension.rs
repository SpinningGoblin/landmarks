use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Deserialize, Serialize, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}

impl ToString for Dimension {
    fn to_string(&self) -> String {
        match *self {
            Dimension::Overworld => "overworld",
            Dimension::Nether => "nether",
            Dimension::End => "end",
        }
        .to_string()
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
