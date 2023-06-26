use neo4rs::query;
use strum::IntoEnumIterator;

use crate::minecraft::{Biome, Dimension, Platform};

pub async fn ensure_minecraft_nodes(graph: &neo4rs::Graph) -> Result<(), anyhow::Error> {
    let mut biome_merges: Vec<String> = Vec::new();
    let mut biome_returns: Vec<String> = Vec::new();
    Biome::iter().for_each(|biome| {
        biome_merges.push(format!(
            "MERGE ({}: Biome {{name: '{}'}})",
            biome.to_string(),
            biome.to_string()
        ));
        biome_returns.push(biome.to_string());
    });

    let mut dimension_merges: Vec<String> = Vec::new();
    let mut dimension_returns: Vec<String> = Vec::new();
    Dimension::iter().for_each(|dimension| {
        dimension_merges.push(format!(
            "MERGE ({}: Dimension {{name: '{}'}})",
            dimension.to_string(),
            dimension.to_string()
        ));
        dimension_returns.push(dimension.to_string());
    });

    let mut platform_merges: Vec<String> = Vec::new();
    let mut platform_returns: Vec<String> = Vec::new();
    Platform::iter().for_each(|platform| {
        platform_merges.push(format!(
            "MERGE ({}: Platform {{name: '{}'}})",
            platform.to_string(),
            platform.to_string()
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
