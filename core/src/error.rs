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
    #[error("no world with id ({})", .0)]
    NoWorldWithId(Uuid),
    #[error("invalid persisted id ({})", .message)]
    InvalidUuid { message: String },
    #[error("error from graph operation {}", .message)]
    GraphError { message: String },
}
