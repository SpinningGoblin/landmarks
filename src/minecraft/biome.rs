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
}

impl ToString for Biome {
    fn to_string(&self) -> String {
        match *self {
            Biome::Ocean => "ocean",
            Biome::DeepOcean => "deep_ocean",
            Biome::WarmOcean => "warm_ocean",
            Biome::LukewarmOcean => "lukewarm_ocean",
            Biome::DeepLukewarmOcean => "deep_lukewarm_ocean",
            Biome::DeepColdOcean => "deep_cold_ocean",
            Biome::FrozenOcean => "frozen_ocean",
            Biome::DeepFrozenOcean => "deep_frozen_ocean",
            Biome::MushroomFields => "mushroom_fields",
            Biome::JaggedPeaks => "jagged_peaks",
            Biome::FrozenPeaks => "frozen_peaks",
            Biome::StonyPeaks => "stony_peaks",
            Biome::Meadow => "meadow",
            Biome::CherryGrove => "cherry_grove",
            Biome::Grove => "grove",
            Biome::SnowySlopes => "snowy_slopes",
            Biome::WindsweptHills => "windswept_hills",
            Biome::WindsweptGravellyHills => "windswept_gravelly_hills",
            Biome::WindsweptForest => "windswept_forest",
            Biome::Forest => "forest",
            Biome::FlowerForest => "flower_forest",
            Biome::Taiga => "taiga",
            Biome::OldGrowthPineTaiga => "old_growth_pine_taiga",
            Biome::OldGrowthSpruceTaiga => "old_growth_spruce_taiga",
            Biome::SnowyTaiga => "snowy_taiga",
            Biome::BirchForest => "birch_forest",
            Biome::OldGrowthBirchForest => "old_growth_birch_forest",
            Biome::DarkForest => "dark_forest",
            Biome::Jungle => "jungle",
            Biome::SparseJungle => "sparse_jungle",
            Biome::BambooJungle => "bamboo_jungle",
            Biome::River => "river",
            Biome::FrozenRiver => "frozen_river",
            Biome::Swamp => "swamp",
            Biome::MangroveSwamp => "mangrove_swamp",
            Biome::Beach => "beach",
            Biome::SnowyBeach => "snowy_beach",
            Biome::StonyShore => "stony_shore",
            Biome::Plains => "plains",
            Biome::SunflowerPlains => "sunflower_plains",
            Biome::SnowyPlains => "snowy_plains",
            Biome::IceSpikes => "ice_spikes",
            Biome::Desert => "desert",
            Biome::Savanna => "savanna",
            Biome::SavannaPlateau => "savanna_plateau",
            Biome::WindsweptSavanna => "windswept_savanna",
            Biome::Badlands => "badlands",
            Biome::WoodedBadlands => "wooded_badlands",
            Biome::ErodedBadlands => "eroded_badlands",
            Biome::DeepDark => "deep_dark",
            Biome::DripstoneCaves => "dripstone_caves",
            Biome::LushCaves => "lush_caves",
            Biome::TheVoid => "the_void",
            Biome::NetherWastes => "nether_wastes",
            Biome::SoulSandValley => "soul_sand_valley",
            Biome::CrimsonForest => "crimson_forest",
            Biome::WarpedForest => "warped_forest",
            Biome::BasaltDeltas => "basalt_deltas",
            Biome::TheEnd => "the_end",
            Biome::SmallEndIslands => "small_end_islands",
            Biome::EndMidlands => "end_midlands",
            Biome::EndHighlands => "end_highlands",
            Biome::EndBarrens => "end_barrens",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Biome;

    #[test]
    pub fn serialized() {
        let biome = Biome::BambooJungle;

        let serialized = serde_json::to_string(&biome).unwrap();
        assert_eq!("\"bamboo_jungle\"", serialized);
    }

    #[test]
    pub fn to_string() {
        let biome = Biome::WindsweptGravellyHills;

        assert_eq!("windswept_gravelly_hills", biome.to_string());
    }
}
