use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum LandmarksError {
    #[error("invalid biome ({})", .0)]
    InvalidBiome(String),
    #[error("invalid dimension ({})", .0)]
    InvalidDimension(String),
    #[error("invalid platform ({})", .0)]
    InvalidPlatform(String),
    #[error("invalid landmark link type ({})", .0)]
    InvalidLandmarkLinkType(String),
    #[error("no world with id ({})", .0)]
    NoWorldWithId(Uuid),
    #[error("invalid persisted id ({})", .message)]
    InvalidUuid { message: String },
    #[error("graph deserialization error ({})", .message)]
    GraphDeserializationError { message: String },
    #[error("error from graph operation {}", .source)]
    Neo4jError { source: neo4rs::Error },
    #[error("missing required env variable {}", .name)]
    MissingEnvVariable { name: String },
}

impl From<neo4rs::Error> for LandmarksError {
    fn from(value: neo4rs::Error) -> Self {
        LandmarksError::Neo4jError { source: value }
    }
}
