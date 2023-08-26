use std::str::FromStr;

use crate::{
    landmarks::{LandmarkLink, LandmarkLinkType},
    minecraft::{Biome, Coordinate, Dimension, Farm},
    LandmarksError, Tag,
};
use chrono::Utc;
use neo4rs::{query, Graph, Node, Txn};
use uuid::Uuid;

use crate::landmarks::{CreateLandmark, Landmark, LandmarkMetadata};

use super::worlds::set_world_updated_at_now;

pub async fn link_landmarks(
    transaction: &Txn,
    landmark_id_1: &Uuid,
    landmark_id_2: &Uuid,
    link_type: &Option<LandmarkLinkType>,
) -> Result<(), LandmarksError> {
    let landmark_matches = format!(
        "
        MATCH (landmark_1:Landmark {{ id: '{landmark_id_1}' }})
        MATCH (landmark_2:Landmark {{ id: '{landmark_id_2}' }})
        "
    );
    let link_type_text = link_type
        .as_ref()
        .map(|lt| lt.to_string())
        .unwrap_or_default();
    let link_rel = format!("[:LINKEDTO {{ link_type: '{link_type_text}' }}]");
    let merge = format!(
        "
        MERGE (landmark_1)-{link_rel}->(landmark_2)
        MERGE (landmark_2)-{link_rel}->(landmark_1)
        "
    );

    let full_query = format!(
        "
        {landmark_matches}
        {merge}
        RETURN landmark_1.id, landmark_2.id
        "
    );

    transaction.run(query(&full_query)).await?;
    update_world_updated_at(transaction, landmark_id_1).await?;

    Ok(())
}

pub async fn add_biome(
    transaction: &Txn,
    landmark_id: Uuid,
    biome: Biome,
) -> Result<(), LandmarksError> {
    let landmark_match = format!("MATCH (landmark:Landmark {{ id: '{}' }})", landmark_id);
    let biome_merge = format!("MERGE (biome:Biome {{ name: '{}' }})", biome);
    let biome_rel_merge = "MERGE (landmark)-[:HASBIOME]->(biome)";

    let full_query = format!(
        "{landmark_match}
        {biome_merge}
        {biome_rel_merge}
        RETURN landmark.id",
    );

    transaction.run(query(&full_query)).await?;
    update_world_updated_at(transaction, &landmark_id).await?;
    Ok(())
}

pub async fn remove_biome(
    transaction: &Txn,
    landmark_id: Uuid,
    biome: Biome,
) -> Result<(), LandmarksError> {
    let landmark_match = format!("MATCH (landmark:Landmark {{ id: '{}' }})", landmark_id);
    let biome_match = format!("MATCH (biome:Biome {{ name: '{}' }})", biome);
    let biome_rel_match = "MATCH (landmark)-[r:HASBIOME]->(biome)";

    let full_query = format!(
        "{landmark_match}
        {biome_match}
        {biome_rel_match}
        DELETE r",
    );

    transaction.run(query(&full_query)).await?;
    update_world_updated_at(transaction, &landmark_id).await?;
    Ok(())
}

pub async fn add_tag(transaction: &Txn, landmark_id: Uuid, tag: Tag) -> Result<(), LandmarksError> {
    let landmark_match = format!("MATCH (landmark:Landmark {{ id: '{}' }})", landmark_id);
    let tag_merge = format!("MERGE (tag:Tag {{ name: '{}' }})", tag);
    let tag_rel_merge = "MERGE (landmark)-[:HASTAG]->(tag)";

    let full_query = format!(
        "{landmark_match}
        {tag_merge}
        {tag_rel_merge}
        RETURN landmark.id",
    );

    transaction.run(query(&full_query)).await?;
    update_world_updated_at(transaction, &landmark_id).await?;
    Ok(())
}

pub async fn remove_tag(
    transaction: &Txn,
    landmark_id: Uuid,
    tag: Tag,
) -> Result<(), LandmarksError> {
    let landmark_match = format!("MATCH (landmark:Landmark {{ id: '{}' }})", landmark_id);
    let tag_match = format!("MATCH (tag:Tag {{ name: '{}' }})", tag);
    let tag_rel_match = "MATCH (landmark)-[r:HASTAG]->(tag)";

    let full_query = format!(
        "{landmark_match}
        {tag_match}
        {tag_rel_match}
        DELETE r",
    );

    transaction.execute(query(&full_query)).await?;
    update_world_updated_at(transaction, &landmark_id).await?;
    Ok(())
}

pub async fn add_farm(
    transaction: &Txn,
    landmark_id: Uuid,
    farm: Farm,
) -> Result<(), LandmarksError> {
    let landmark_match = format!("MATCH (landmark:Landmark {{ id: '{}' }})", landmark_id);
    let farm_merge = format!("MERGE (farm:Farm {{ name: '{}' }})", farm);
    let farm_rel_merge = "MERGE (landmark)-[:HASFARM]->(farm)";

    let full_query = format!(
        "{landmark_match}
        {farm_merge}
        {farm_rel_merge}
        RETURN landmark.id",
    );

    transaction.run(query(&full_query)).await?;
    update_world_updated_at(transaction, &landmark_id).await?;
    Ok(())
}

pub async fn remove_farm(
    transaction: &Txn,
    landmark_id: Uuid,
    farm: Farm,
) -> Result<(), LandmarksError> {
    let landmark_match = format!("MATCH (landmark:Landmark {{ id: '{}' }})", landmark_id);
    let farm_match = format!("MATCH (farm:Farm {{ name: '{}' }})", farm);
    let farm_rel_match = "MATCH (landmark)-[r:HASFARM]->(farm)";

    let full_query = format!(
        "{landmark_match}
        {farm_match}
        {farm_rel_match}
        DELETE r",
    );

    transaction.run(query(&full_query)).await?;
    update_world_updated_at(transaction, &landmark_id).await?;
    Ok(())
}

pub async fn update_coordinate(
    transaction: &Txn,
    landmark_id: Uuid,
    coordinate: &Coordinate,
) -> Result<(), LandmarksError> {
    let landmark_match = format!("MATCH (landmark:Landmark {{ id: '{}' }})", landmark_id);
    let basic_position = format!(
        "x: {}, y: {}, z: {}",
        coordinate.x, coordinate.y, coordinate.z
    );
    // Putting the z into the y of the point because that's how distance would be measured in Minecraft.
    let coordinate_2d = format!(
        "coordinate_2d: point({{ x: {}, y: {}, crs: 'cartesian' }})",
        coordinate.x, coordinate.z
    );
    // Putting the z into the y of the point because that's how distance would be measured in Minecraft.
    let coordinate_3d = format!(
        "coordinate_3d: point({{ x: {}, y: {}, z: {}, crs: 'cartesian-3D' }})",
        coordinate.x, coordinate.y, coordinate.z
    );
    let full_query = format!(
        "{landmark_match}
        SET landmark += {{ {basic_position}, {coordinate_2d}, {coordinate_3d} }}
        RETURN landmark.id"
    );
    transaction.run(query(&full_query)).await?;
    update_world_updated_at(transaction, &landmark_id).await?;
    Ok(())
}

pub async fn update_notes(
    transaction: &Txn,
    landmark_id: Uuid,
    notes: &str,
) -> Result<(), LandmarksError> {
    let landmark_match = format!("MATCH (landmark:Landmark {{ id: '{}' }})", landmark_id);
    let full_query = format!(
        "{landmark_match}
        SET landmark.notes = '{}'
        RETURN landmark.id",
        notes
    );
    transaction.run(query(&full_query)).await?;
    update_world_updated_at(transaction, &landmark_id).await?;
    Ok(())
}

pub async fn update_world_updated_at(
    transaction: &Txn,
    landmark_id: &Uuid,
) -> Result<(), LandmarksError> {
    let landmark_match = format!("MATCH (landmark:Landmark {{ id: '{}' }})", landmark_id);
    let now = Utc::now().to_string();
    let full_query = format!(
        "
        {landmark_match}
        MATCH (landmark)-[:PARTOF]->(world:World)
        SET world.updated_at = '{now}'
        RETURN world.id as world_id
        "
    );
    let mut result = transaction.execute(query(&full_query)).await?;

    if result.next().await?.is_none() {
        println!("Couldn't find world for landmark {landmark_id}");
    };

    Ok(())
}

pub async fn create(
    transaction: &Txn,
    world_id: Uuid,
    create: CreateLandmark,
    user: &str,
) -> Result<Uuid, LandmarksError> {
    let id = Uuid::new_v4();

    let world_exists = super::worlds::check_world_exists(transaction, &world_id).await?;

    if !world_exists {
        return Err(LandmarksError::NoWorldWithId(world_id));
    }

    let user_match = format!("MATCH (user:User {{ name: '{}' }})", user);
    let world_match = format!("MATCH (world:World {{ id: '{}' }})", world_id);
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
    set_world_updated_at_now(transaction, &world_id).await?;

    Ok(id)
}

pub async fn landmarks_for_world(
    graph: &Graph,
    world_id: &Uuid,
) -> Result<Vec<LandmarkMetadata>, LandmarksError> {
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
        let landmark_node: Node =
            row.get("landmark")
                .ok_or(LandmarksError::GraphDeserializationError {
                    message: "no_landmark_node".to_string(),
                })?;
        let name: String = landmark_node.get("name").unwrap_or_default();
        let notes: Option<String> = landmark_node.get("notes");
        let id_value: String =
            landmark_node
                .get("id")
                .ok_or(LandmarksError::GraphDeserializationError {
                    message: "no_landmark_id".to_string(),
                })?;
        let id = Uuid::parse_str(&id_value).map_err(|e| LandmarksError::InvalidUuid {
            message: e.to_string(),
        })?;
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

pub async fn linked_landmarks(
    graph: &Graph,
    landmark_id: &Uuid,
) -> Result<Vec<LandmarkLink>, LandmarksError> {
    let landmark_match = format!(
        "MATCH (landmark:Landmark {{ id: '{}' }})",
        landmark_id.to_string()
    );
    let link_match = "MATCH (landmark)-[link:LINKEDTO]->(linked_landmark:Landmark)";
    let query_return = "RETURN link.link_type as link_type, linked_landmark as landmark";
    let full_query = format!(
        "
        {landmark_match}
        {link_match}
        {query_return}
        "
    );

    let mut result = graph.execute(query(&full_query)).await?;
    let mut links: Vec<LandmarkLink> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let landmark_node: Node =
            row.get("landmark")
                .ok_or(LandmarksError::GraphDeserializationError {
                    message: "no_landmark".to_string(),
                })?;
        let name: String = landmark_node.get("name").unwrap_or_default();
        let notes: Option<String> = landmark_node.get("notes");
        let id_value: String =
            landmark_node
                .get("id")
                .ok_or(LandmarksError::GraphDeserializationError {
                    message: "no_landmark_id".to_string(),
                })?;
        let id = Uuid::parse_str(&id_value).map_err(|e| LandmarksError::InvalidUuid {
            message: e.to_string(),
        })?;
        let x: i64 = landmark_node.get("x").unwrap();
        let y: i64 = landmark_node.get("y").unwrap();
        let z: i64 = landmark_node.get("z").unwrap();
        let link_type_value: String =
            row.get("link_type")
                .ok_or(LandmarksError::GraphDeserializationError {
                    message: "no_link_type".to_string(),
                })?;
        let link_type: LandmarkLinkType = LandmarkLinkType::from_str(&link_type_value)
            .map_err(|_| LandmarksError::InvalidLandmarkLinkType(link_type_value))?;
        links.push(LandmarkLink {
            landmark_metadata: LandmarkMetadata {
                id,
                coordinate: Coordinate { x, y, z },
                name,
                notes,
            },
            link_type: Some(link_type),
        });
    }

    Ok(links)
}

pub async fn landmark_by_id(
    graph: &Graph,
    landmark_id: &Uuid,
) -> Result<Option<Landmark>, LandmarksError> {
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

    let mut result = graph.execute(query(&full_query)).await?;

    match result.next().await {
        Ok(Some(row)) => {
            let landmark_node: Node =
                row.get("landmark")
                    .ok_or(LandmarksError::GraphDeserializationError {
                        message: "no_landmark_node".to_string(),
                    })?;
            let name: String = landmark_node.get("name").unwrap_or_default();
            let notes: Option<String> = landmark_node.get("notes");
            let id_value: String =
                landmark_node
                    .get("id")
                    .ok_or(LandmarksError::GraphDeserializationError {
                        message: "no_landmark_id".to_string(),
                    })?;
            let id = Uuid::parse_str(&id_value).map_err(|e| LandmarksError::InvalidUuid {
                message: e.to_string(),
            })?;
            let x: i64 = landmark_node.get("x").unwrap();
            let y: i64 = landmark_node.get("y").unwrap();
            let z: i64 = landmark_node.get("z").unwrap();

            let tag_values: Vec<String> =
                row.get("tags")
                    .ok_or(LandmarksError::GraphDeserializationError {
                        message: "no_tags_column".to_string(),
                    })?;
            let tags = tag_values.into_iter().map(Tag).collect::<Vec<Tag>>();

            let farm_values: Vec<String> =
                row.get("farms")
                    .ok_or(LandmarksError::GraphDeserializationError {
                        message: "no_farms_column".to_string(),
                    })?;
            let farms = farm_values.into_iter().map(Farm).collect::<Vec<Farm>>();

            let biome_values: Vec<String> =
                row.get("biomes")
                    .ok_or(LandmarksError::GraphDeserializationError {
                        message: "no_biomes_column".to_string(),
                    })?;
            let biomes = biome_values
                .into_iter()
                .map(|name| Biome::from_str(&name).unwrap())
                .collect::<Vec<Biome>>();

            let dimension_value: String =
                row.get("dimension")
                    .ok_or(LandmarksError::GraphDeserializationError {
                        message: "no_dimension_column".to_string(),
                    })?;
            let dimension = Dimension::from_str(&dimension_value).unwrap();

            let links = linked_landmarks(graph, landmark_id).await?;

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
                links,
            }))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(LandmarksError::from(e)),
    }
}
