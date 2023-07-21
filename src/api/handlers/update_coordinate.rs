use serde::Deserialize;

use crate::minecraft::Coordinate;

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateCoordinate {
    pub coordinate: Coordinate,
}
