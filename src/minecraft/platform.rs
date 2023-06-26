use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Deserialize, Serialize, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    Bedrock,
    Java,
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        match *self {
            Platform::Bedrock => "bedrock",
            Platform::Java => "java",
        }
        .to_string()
    }
}
