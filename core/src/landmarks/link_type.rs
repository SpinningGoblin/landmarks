use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum LandmarkLinkType {
    CloseDistance,
    EndPortal,
    MineCart,
    NetherPortal,
    WalkingPath,
    IceBoatPath,
    MineCartAndIceBoatPath,
}

impl LandmarkLinkType {
    pub fn all() -> LandmarkLinkTypeIter {
        LandmarkLinkType::iter()
    }
}

impl Display for LandmarkLinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            LandmarkLinkType::CloseDistance => "close_distance",
            LandmarkLinkType::EndPortal => "end_portal",
            LandmarkLinkType::MineCart => "mine_cart",
            LandmarkLinkType::NetherPortal => "nether_portal",
            LandmarkLinkType::WalkingPath => "walking_path",
            LandmarkLinkType::IceBoatPath => "ice_boat_path",
            LandmarkLinkType::MineCartAndIceBoatPath => "mine_cart_and_ice_boat_path",
        };

        f.write_str(text)
    }
}

impl FromStr for LandmarkLinkType {
    type Err = crate::LandmarksError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "close_distance" => Ok(LandmarkLinkType::CloseDistance),
            "end_portal" => Ok(LandmarkLinkType::EndPortal),
            "mine_cart" => Ok(LandmarkLinkType::MineCart),
            "nether_portal" => Ok(LandmarkLinkType::NetherPortal),
            "walking_path" => Ok(LandmarkLinkType::WalkingPath),
            "ice_boat_path" => Ok(LandmarkLinkType::IceBoatPath),
            "mine_cart_and_ice_boat_path" => Ok(LandmarkLinkType::MineCartAndIceBoatPath),
            _ => Err(crate::LandmarksError::InvalidLandmarkLinkType(
                s.to_string(),
            )),
        }
    }
}
