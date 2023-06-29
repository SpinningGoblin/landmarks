use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum Biome {
    Ocean,
    DeepOcean,
    WarmOcean,
    LukewarmOcean,
    DeepLukewarmOcean,
    DeepColdOcean,
    FrozenOcean,
    DeepFrozenOcean,
    MushroomFields,
    JaggedPeaks,
    FrozenPeaks,
    StonyPeaks,
    Meadow,
    CherryGrove,
    Grove,
    SnowySlopes,
    WindsweptHills,
    WindsweptGravellyHills,
    WindsweptForest,
    Forest,
    FlowerForest,
    Taiga,
    OldGrowthPineTaiga,
    OldGrowthSpruceTaiga,
    SnowyTaiga,
    BirchForest,
    OldGrowthBirchForest,
    DarkForest,
    Jungle,
    SparseJungle,
    BambooJungle,
    River,
    FrozenRiver,
    Swamp,
    MangroveSwamp,
    Beach,
    SnowyBeach,
    StonyShore,
    Plains,
    SunflowerPlains,
    SnowyPlains,
    IceSpikes,
    Desert,
    Savanna,
    SavannaPlateau,
    WindsweptSavanna,
    Badlands,
    WoodedBadlands,
    ErodedBadlands,
    DeepDark,
    DripstoneCaves,
    LushCaves,
    TheVoid,
    NetherWastes,
    SoulSandValley,
    CrimsonForest,
    WarpedForest,
    BasaltDeltas,
    TheEnd,
    SmallEndIslands,
    EndMidlands,
    EndHighlands,
    EndBarrens,
    Other(String),
}

impl FromStr for Biome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let biome = match s {
            "ocean" => Biome::Ocean,
            "deep_ocean" => Biome::DeepOcean,
            "warm_ocean" => Biome::WarmOcean,
            "lukewarm_ocean" => Biome::LukewarmOcean,
            "deep_lukewarm_ocean" => Biome::DeepLukewarmOcean,
            "deep_cold_ocean" => Biome::DeepColdOcean,
            "frozen_ocean" => Biome::FrozenOcean,
            "deep_frozen_ocean" => Biome::DeepFrozenOcean,
            "mushroom_fields" => Biome::MushroomFields,
            "jagged_peaks" => Biome::JaggedPeaks,
            "frozen_peaks" => Biome::FrozenPeaks,
            "stony_peaks" => Biome::StonyPeaks,
            "meadow" => Biome::Meadow,
            "cherry_grove" => Biome::CherryGrove,
            "grove" => Biome::Grove,
            "snowy_slopes" => Biome::SnowySlopes,
            "windswept_hills" => Biome::WindsweptHills,
            "windswept_gravelly_hills" => Biome::WindsweptGravellyHills,
            "windswept_forest" => Biome::WindsweptForest,
            "forest" => Biome::Forest,
            "flower_forest" => Biome::FlowerForest,
            "taiga" => Biome::Taiga,
            "old_growth_pine_taiga" => Biome::OldGrowthPineTaiga,
            "old_growth_spruce_taiga" => Biome::OldGrowthSpruceTaiga,
            "snowy_taiga" => Biome::SnowyTaiga,
            "birch_forest" => Biome::BirchForest,
            "old_growth_birch_forest" => Biome::OldGrowthBirchForest,
            "dark_forest" => Biome::DarkForest,
            "jungle" => Biome::Jungle,
            "sparse_jungle" => Biome::SparseJungle,
            "bamboo_jungle" => Biome::BambooJungle,
            "river" => Biome::River,
            "frozen_river" => Biome::FrozenRiver,
            "swamp" => Biome::Swamp,
            "mangrove_swamp" => Biome::MangroveSwamp,
            "beach" => Biome::Beach,
            "snowy_beach" => Biome::SnowyBeach,
            "stony_shore" => Biome::StonyShore,
            "plains" => Biome::Plains,
            "sunflower_plains" => Biome::SunflowerPlains,
            "snowy_plains" => Biome::SnowyPlains,
            "ice_spikes" => Biome::IceSpikes,
            "desert" => Biome::Desert,
            "savanna" => Biome::Savanna,
            "savanna_plateau" => Biome::SavannaPlateau,
            "windswept_savanna" => Biome::WindsweptSavanna,
            "badlands" => Biome::Badlands,
            "wooded_badlands" => Biome::WoodedBadlands,
            "eroded_badlands" => Biome::ErodedBadlands,
            "deep_dark" => Biome::DeepDark,
            "dripstone_caves" => Biome::DripstoneCaves,
            "lush_caves" => Biome::LushCaves,
            "the_void" => Biome::TheVoid,
            "nether_wastes" => Biome::NetherWastes,
            "soul_sand_valley" => Biome::SoulSandValley,
            "crimson_forest" => Biome::CrimsonForest,
            "warped_forest" => Biome::WarpedForest,
            "basalt_deltas" => Biome::BasaltDeltas,
            "the_end" => Biome::TheEnd,
            "small_end_islands" => Biome::SmallEndIslands,
            "end_midlands" => Biome::EndMidlands,
            "end_barrens" => Biome::EndBarrens,
            "end_highlands" => Biome::EndHighlands,
            text => Biome::Other(text.to_string()),
        };

        Ok(biome)
    }
}

impl Display for Biome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Biome::Ocean => "ocean".to_string(),
            Biome::DeepOcean => "deep_ocean".to_string(),
            Biome::WarmOcean => "warm_ocean".to_string(),
            Biome::LukewarmOcean => "lukewarm_ocean".to_string(),
            Biome::DeepLukewarmOcean => "deep_lukewarm_ocean".to_string(),
            Biome::DeepColdOcean => "deep_cold_ocean".to_string(),
            Biome::FrozenOcean => "frozen_ocean".to_string(),
            Biome::DeepFrozenOcean => "deep_frozen_ocean".to_string(),
            Biome::MushroomFields => "mushroom_fields".to_string(),
            Biome::JaggedPeaks => "jagged_peaks".to_string(),
            Biome::FrozenPeaks => "frozen_peaks".to_string(),
            Biome::StonyPeaks => "stony_peaks".to_string(),
            Biome::Meadow => "meadow".to_string(),
            Biome::CherryGrove => "cherry_grove".to_string(),
            Biome::Grove => "grove".to_string(),
            Biome::SnowySlopes => "snowy_slopes".to_string(),
            Biome::WindsweptHills => "windswept_hills".to_string(),
            Biome::WindsweptGravellyHills => "windswept_gravelly_hills".to_string(),
            Biome::WindsweptForest => "windswept_forest".to_string(),
            Biome::Forest => "forest".to_string(),
            Biome::FlowerForest => "flower_forest".to_string(),
            Biome::Taiga => "taiga".to_string(),
            Biome::OldGrowthPineTaiga => "old_growth_pine_taiga".to_string(),
            Biome::OldGrowthSpruceTaiga => "old_growth_spruce_taiga".to_string(),
            Biome::SnowyTaiga => "snowy_taiga".to_string(),
            Biome::BirchForest => "birch_forest".to_string(),
            Biome::OldGrowthBirchForest => "old_growth_birch_forest".to_string(),
            Biome::DarkForest => "dark_forest".to_string(),
            Biome::Jungle => "jungle".to_string(),
            Biome::SparseJungle => "sparse_jungle".to_string(),
            Biome::BambooJungle => "bamboo_jungle".to_string(),
            Biome::River => "river".to_string(),
            Biome::FrozenRiver => "frozen_river".to_string(),
            Biome::Swamp => "swamp".to_string(),
            Biome::MangroveSwamp => "mangrove_swamp".to_string(),
            Biome::Beach => "beach".to_string(),
            Biome::SnowyBeach => "snowy_beach".to_string(),
            Biome::StonyShore => "stony_shore".to_string(),
            Biome::Plains => "plains".to_string(),
            Biome::SunflowerPlains => "sunflower_plains".to_string(),
            Biome::SnowyPlains => "snowy_plains".to_string(),
            Biome::IceSpikes => "ice_spikes".to_string(),
            Biome::Desert => "desert".to_string(),
            Biome::Savanna => "savanna".to_string(),
            Biome::SavannaPlateau => "savanna_plateau".to_string(),
            Biome::WindsweptSavanna => "windswept_savanna".to_string(),
            Biome::Badlands => "badlands".to_string(),
            Biome::WoodedBadlands => "wooded_badlands".to_string(),
            Biome::ErodedBadlands => "eroded_badlands".to_string(),
            Biome::DeepDark => "deep_dark".to_string(),
            Biome::DripstoneCaves => "dripstone_caves".to_string(),
            Biome::LushCaves => "lush_caves".to_string(),
            Biome::TheVoid => "the_void".to_string(),
            Biome::NetherWastes => "nether_wastes".to_string(),
            Biome::SoulSandValley => "soul_sand_valley".to_string(),
            Biome::CrimsonForest => "crimson_forest".to_string(),
            Biome::WarpedForest => "warped_forest".to_string(),
            Biome::BasaltDeltas => "basalt_deltas".to_string(),
            Biome::TheEnd => "the_end".to_string(),
            Biome::SmallEndIslands => "small_end_islands".to_string(),
            Biome::EndMidlands => "end_midlands".to_string(),
            Biome::EndHighlands => "end_highlands".to_string(),
            Biome::EndBarrens => "end_barrens".to_string(),
            Biome::Other(text) => text.clone(),
        };

        f.write_str(&value)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use strum::IntoEnumIterator;

    use super::Biome;

    #[test]
    pub fn to_and_from_string() {
        Biome::iter().for_each(|biome| {
            let text = biome.to_string();
            let from_text = Biome::from_str(&text).unwrap();
            assert_eq!(biome, from_text);
        });
    }

    #[test]
    pub fn to_and_from_string_other() {
        let biome = Biome::Other("custom biome".to_string());
        let text = biome.to_string();
        let compare_biome = Biome::from_str(&text).unwrap();
        assert_eq!(biome, compare_biome);
    }

    #[test]
    pub fn to_string() {
        let biome = Biome::WindsweptGravellyHills;

        assert_eq!("windswept_gravelly_hills", biome.to_string());
    }
}
