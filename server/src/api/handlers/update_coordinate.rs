use serde::Deserialize;

use landmarks_core::minecraft::Coordinate;

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateCoordinate {
    pub coordinate: Coordinate,
}
