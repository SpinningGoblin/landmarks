use std::str::FromStr;

use neo4rs::{query, Graph, Node, Txn};
use uuid::Uuid;

use crate::{
    landmarks::{CreateLandmark, Landmark, LandmarkMetadata},
    minecraft::{Biome, Coordinate, Dimension, Farm},
    Tag,
};

pub async fn create(
    transaction: &Txn,
    world_id: Uuid,
    create: CreateLandmark,
    user: &str,
) -> Result<Uuid, anyhow::Error> {
    let id = Uuid::new_v4();

    let world_exists = super::worlds::check_world_exists(transaction, &world_id).await?;

    if !world_exists {
        return Err(anyhow::Error::msg("invalid_world_id"));
    }

    let user_match = format!("MATCH (user:User {{ name: '{}' }})", user);
    let world_match = format!("MATCH (world:World {{ id: '{}' }})", world_id.to_string());
    let basic_position = format!(
        "x: {}, y: {}, z: {}",
        &create.coordinate.x, &create.coordinate.y, &create.coordinate.z
    );
    // Putting the z into the y of the point because that's how distance would be measured in Minecraft.
    let coordinate_2d = format!(
        "coordinate_2d: point({{ x: {}, y: {}, crs: 'cartesian' }})",
        &create.coordinate.x, &create.coordinate.z
    );
    // Putting the z into the y of the point because that's how distance would be measured in Minecraft.
    let coordinate_3d = format!(
        "coordinate_3d: point({{ x: {}, y: {}, z: {}, crs: 'cartesian-3D' }})",
        &create.coordinate.x, &create.coordinate.y, &create.coordinate.z
    );
    let landmark_name = format!("name: '{}'", &create.name);
    let landmark_note = format!("notes: '{}'", &create.notes.unwrap_or_default());
    let landmark_create = format!(
        "CREATE (landmark:Landmark {{ id: '{}', {}, {}, {}, {}, {} }})",
        id.to_string(),
        basic_position,
        coordinate_2d,
        coordinate_3d,
        landmark_name,
        landmark_note
    );
    let dimension_match = format!(
        "MERGE (dimension:Dimension {{ name: '{}' }})",
        &create.dimension.to_string()
    );

    let mut tag_merges: Vec<String> = Vec::new();
    let mut tag_variables: Vec<String> = Vec::new();
    create.tags.iter().enumerate().for_each(|(index, tag)| {
        let tag_var = format!("tag_{index}");
        tag_merges.push(format!("MERGE ({}:Tag {{ name: '{}' }})", tag_var, tag));
        tag_variables.push(tag_var);
    });
    let tag_merge = tag_merges.join("\n");

    let mut biome_merges: Vec<String> = Vec::new();
    let mut biome_variables: Vec<String> = Vec::new();
    create.biomes.iter().enumerate().for_each(|(index, biome)| {
        let biome_var = format!("biome_{index}");
        biome_merges.push(format!(
            "MERGE ({}:Biome {{ name: '{}' }})",
            biome_var,
            biome.to_string()
        ));
        biome_variables.push(biome_var);
    });
    let biome_merge = biome_merges.join("\n");

    let mut farm_merges: Vec<String> = Vec::new();
    let mut farm_variables: Vec<String> = Vec::new();
    create.farms.iter().enumerate().for_each(|(index, farm)| {
        let farm_var = format!("farm_{index}");
        farm_merges.push(format!(
            "MERGE ({}:Farm {{ name: '{}' }})",
            farm_var,
            farm.to_string()
        ));
        farm_variables.push(farm_var);
    });
    let farm_merge = farm_merges.join("\n");

    let landmark_dimension_rel = "CREATE (landmark)-[:IN]->(dimension)";
    let landmark_world_rel = "CREATE (landmark)-[:PARTOF]->(world)";
    let world_landmark_rel = "CREATE (world)-[:HASLANDMARK]->(landmark)";
    let landmark_user_rel = "CREATE (landmark)-[:CREATEDBY]->(user)";
    let user_landmark_rel = "CREATE (user)-[:CREATED]->(landmark)";
    let landmark_tag_rels = tag_variables
        .iter()
        .map(|tag_var| format!("CREATE (landmark)-[:HASTAG]->({})", tag_var))
        .collect::<Vec<String>>()
        .join("\n");

    let landmark_biome_rels = biome_variables
        .iter()
        .map(|biome_var| format!("CREATE (landmark)-[:HASBIOME]->({})", biome_var))
        .collect::<Vec<String>>()
        .join("\n");

    let landmark_farm_rels = farm_variables
        .iter()
        .map(|farm_var| format!("CREATE (landmark)-[:HASFARM]->({})", farm_var))
        .collect::<Vec<String>>()
        .join("\n");

    let full_query = format!(
        "{user_match}
        {world_match}
        {dimension_match}
        {tag_merge}
        {biome_merge}
        {farm_merge}
        {landmark_create}
        {landmark_dimension_rel}
        {landmark_world_rel}
        {world_landmark_rel}
        {landmark_user_rel}
        {user_landmark_rel}
        {landmark_tag_rels}
        {landmark_biome_rels}
        {landmark_farm_rels}
        RETURN landmark.id"
    );

    transaction.run(query(&full_query)).await?;

    Ok(id)
}

pub async fn landmarks_for_world(
    graph: &Graph,
    world_id: &Uuid,
) -> Result<Vec<LandmarkMetadata>, anyhow::Error> {
    let world_match = format!("MATCH (world:World {{ id: '{}' }})", world_id.to_string());
    let landmark_match = format!("MATCH (world)-[:HASLANDMARK]->(landmark:Landmark)");
    let full_query = format!(
        "{world_match}
        {landmark_match}
        RETURN landmark"
    );

    let mut result = graph.execute(query(&full_query)).await?;
    let mut landmarks: Vec<LandmarkMetadata> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let landmark_node: Node = row
            .get("landmark")
            .ok_or(anyhow::Error::msg("no_landmark_node"))?;
        let name: String = landmark_node.get("name").unwrap_or_default();
        let notes: Option<String> = landmark_node.get("notes");
        let id_value: String = landmark_node
            .get("id")
            .ok_or(anyhow::Error::msg("no_landmark_id"))?;
        let id = Uuid::parse_str(&id_value)?;
        let x: i64 = landmark_node.get("x").unwrap();
        let y: i64 = landmark_node.get("y").unwrap();
        let z: i64 = landmark_node.get("z").unwrap();

        landmarks.push(LandmarkMetadata {
            id,
            coordinate: Coordinate { x, y, z },
            name,
            notes,
        })
    }

    Ok(landmarks)
}

pub async fn landmark_by_id(
    graph: &Graph,
    landmark_id: &Uuid,
) -> Result<Option<Landmark>, anyhow::Error> {
    let landmark_match = format!(
        "MATCH (landmark:Landmark {{ id: '{}' }})",
        landmark_id.to_string()
    );
    let selects_and_return = r#"
        OPTIONAL MATCH (landmark)-[:HASTAG]->(tag:Tag)
        OPTIONAL MATCH (landmark)-[:IN]->(dimension:Dimension)
        OPTIONAL MATCH (landmark)-[:HASFARM]->(farm:Farm)
        OPTIONAL MATCH (landmark)-[:HASBIOME]->(biome:Biome)
        RETURN landmark,
        apoc.coll.toSet(collect(tag.name)) as tags,
        apoc.coll.toSet(collect(farm.name)) as farms,
        apoc.coll.toSet(collect(biome.name)) as biomes,
        dimension.name as dimension
        "#;
    let full_query = format!(
        "{landmark_match}
        {selects_and_return}"
    );

    let mut result = graph.execute(query(&full_query)).await.unwrap();

    match result.next().await {
        Ok(Some(row)) => {
            let landmark_node: Node = row
                .get("landmark")
                .ok_or(anyhow::Error::msg("no_landmark_node"))?;
            let name: String = landmark_node.get("name").unwrap_or_default();
            let notes: Option<String> = landmark_node.get("notes");
            let id_value: String = landmark_node
                .get("id")
                .ok_or(anyhow::Error::msg("no_landmark_id"))?;
            let id = Uuid::parse_str(&id_value)?;
            let x: i64 = landmark_node.get("x").unwrap();
            let y: i64 = landmark_node.get("y").unwrap();
            let z: i64 = landmark_node.get("z").unwrap();

            let tag_values: Vec<String> = row
                .get("tags")
                .ok_or(anyhow::Error::msg("no_tags_column"))?;
            let tags = tag_values
                .into_iter()
                .map(|name| Tag(name))
                .collect::<Vec<Tag>>();

            let farm_values: Vec<String> = row
                .get("farms")
                .ok_or(anyhow::Error::msg("no_farms_column"))?;
            let farms = farm_values
                .into_iter()
                .map(|name| Farm(name))
                .collect::<Vec<Farm>>();

            let biome_values: Vec<String> = row
                .get("biomes")
                .ok_or(anyhow::Error::msg("no_biomes_column"))?;
            let biomes = biome_values
                .into_iter()
                .map(|name| Biome::from_str(&name).unwrap())
                .collect::<Vec<Biome>>();

            let dimension_value: String = row
                .get("dimension")
                .ok_or(anyhow::Error::msg("no_dimension_column"))?;
            let dimension = Dimension::from_str(&dimension_value).unwrap();

            Ok(Some(Landmark {
                metadata: LandmarkMetadata {
                    id,
                    coordinate: Coordinate { x, y, z },
                    name,
                    notes,
                },
                farms,
                tags,
                biomes,
                dimension,
            }))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(anyhow::Error::new(e)),
    }
}
