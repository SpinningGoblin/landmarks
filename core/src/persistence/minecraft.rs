use std::str::FromStr;

use neo4rs::{query, Graph};
use serde::Deserialize;
use strum::IntoEnumIterator;

use crate::{
    minecraft::{Biome, Dimension, Platform},
    LandmarksError,
};

#[derive(Deserialize)]
struct DimensionRow {
    pub dimension: String,
}

pub async fn list_dimensions(graph: &Graph) -> Result<Vec<Dimension>, LandmarksError> {
    let dimension_match = "MATCH (dimension:Dimension) RETURN dimension.name as dimension";

    let mut result = graph.execute(query(dimension_match)).await?;
    let mut dimensions: Vec<Dimension> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let dimension_row =
            row.to::<DimensionRow>()
                .map_err(|e| LandmarksError::GraphDeserializationError {
                    message: e.to_string(),
                })?;

        dimensions.push(Dimension::from_str(&dimension_row.dimension).unwrap());
    }

    Ok(dimensions)
}

#[derive(Deserialize)]
struct BiomeRow {
    pub biome: String,
}

pub async fn list_biomes(graph: &Graph) -> Result<Vec<Biome>, LandmarksError> {
    let biome_match = "MATCH (biome:Biome) RETURN biome.name as biome";

    let mut result = graph.execute(query(biome_match)).await?;
    let mut biomes: Vec<Biome> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let biome_row =
            row.to::<BiomeRow>()
                .map_err(|e| LandmarksError::GraphDeserializationError {
                    message: e.to_string(),
                })?;

        biomes.push(Biome::from_str(&biome_row.biome).unwrap());
    }

    Ok(biomes)
}

#[derive(Deserialize)]
struct PlatformRow {
    pub platform: String,
}

pub async fn list_platforms(graph: &Graph) -> Result<Vec<Platform>, LandmarksError> {
    let platform_match = "MATCH (platform:Platform) RETURN platform.name as platform";

    let mut result = graph.execute(query(platform_match)).await?;
    let mut platforms: Vec<Platform> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let platform_row =
            row.to::<PlatformRow>()
                .map_err(|e| LandmarksError::GraphDeserializationError {
                    message: e.to_string(),
                })?;

        platforms.push(Platform::from_str(&platform_row.platform).unwrap());
    }

    Ok(platforms)
}

pub async fn ensure_minecraft_nodes(graph: &Graph) -> Result<(), LandmarksError> {
    let mut biome_merges: Vec<String> = Vec::new();
    let mut biome_returns: Vec<String> = Vec::new();
    Biome::iter().for_each(|biome| {
        match biome {
            Biome::Other(_) => {}
            Biome::Ocean
            | Biome::DeepOcean
            | Biome::WarmOcean
            | Biome::LukewarmOcean
            | Biome::DeepLukewarmOcean
            | Biome::DeepColdOcean
            | Biome::FrozenOcean
            | Biome::DeepFrozenOcean
            | Biome::MushroomFields
            | Biome::JaggedPeaks
            | Biome::FrozenPeaks
            | Biome::StonyPeaks
            | Biome::Meadow
            | Biome::CherryGrove
            | Biome::Grove
            | Biome::SnowySlopes
            | Biome::WindsweptHills
            | Biome::WindsweptGravellyHills
            | Biome::WindsweptForest
            | Biome::Forest
            | Biome::FlowerForest
            | Biome::Taiga
            | Biome::OldGrowthPineTaiga
            | Biome::OldGrowthSpruceTaiga
            | Biome::SnowyTaiga
            | Biome::BirchForest
            | Biome::OldGrowthBirchForest
            | Biome::DarkForest
            | Biome::Jungle
            | Biome::SparseJungle
            | Biome::BambooJungle
            | Biome::River
            | Biome::FrozenRiver
            | Biome::Swamp
            | Biome::MangroveSwamp
            | Biome::Beach
            | Biome::SnowyBeach
            | Biome::StonyShore
            | Biome::Plains
            | Biome::SunflowerPlains
            | Biome::SnowyPlains
            | Biome::IceSpikes
            | Biome::Desert
            | Biome::Savanna
            | Biome::SavannaPlateau
            | Biome::WindsweptSavanna
            | Biome::Badlands
            | Biome::WoodedBadlands
            | Biome::ErodedBadlands
            | Biome::DeepDark
            | Biome::DripstoneCaves
            | Biome::LushCaves
            | Biome::TheVoid
            | Biome::NetherWastes
            | Biome::SoulSandValley
            | Biome::CrimsonForest
            | Biome::WarpedForest
            | Biome::BasaltDeltas
            | Biome::TheEnd
            | Biome::SmallEndIslands
            | Biome::EndMidlands
            | Biome::EndHighlands
            | Biome::EndBarrens => {
                biome_merges.push(format!("MERGE ({}: Biome {{name: '{}'}})", biome, biome));
                biome_returns.push(biome.to_string());
            }
        };
    });

    let mut dimension_merges: Vec<String> = Vec::new();
    let mut dimension_returns: Vec<String> = Vec::new();
    Dimension::iter().for_each(|dimension| {
        dimension_merges.push(format!(
            "MERGE ({}: Dimension {{name: '{}'}})",
            dimension, dimension
        ));
        dimension_returns.push(dimension.to_string());
    });

    let mut platform_merges: Vec<String> = Vec::new();
    let mut platform_returns: Vec<String> = Vec::new();
    Platform::iter().for_each(|platform| {
        platform_merges.push(format!(
            "MERGE ({}: Platform {{name: '{}'}})",
            platform, platform
        ));
        platform_returns.push(platform.to_string());
    });

    graph
        .run(query(&format!(
            "{}\nRETURN {}",
            biome_merges.join("\n"),
            biome_returns.join(",")
        )))
        .await?;

    graph
        .run(query(&format!(
            "{}\nRETURN {}",
            dimension_merges.join("\n"),
            dimension_returns.join(",")
        )))
        .await?;

    graph
        .run(query(&format!(
            "{}\nRETURN {}",
            platform_merges.join("\n"),
            platform_returns.join(",")
        )))
        .await?;

    Ok(())
}
