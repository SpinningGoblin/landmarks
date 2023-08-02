use std::str::FromStr;

use crate::{
    minecraft::{Platform, Seed},
    LandmarksError, Tag,
};
use neo4rs::{query, Graph, Node, Query, Txn};
use uuid::Uuid;

use crate::{
    landmarks::Landmark,
    worlds::{CreateWorld, World, WorldMetadata},
};

fn world_match_query(world_id: &Uuid) -> Query {
    let world_match = format!(
        "MATCH (world:World {{ id: '{}' }}) RETURN world.id",
        world_id
    );
    query(&world_match)
}

pub async fn check_world_exists(
    transaction: &Txn,
    world_id: &Uuid,
) -> Result<bool, LandmarksError> {
    let mut result = transaction
        .execute(world_match_query(world_id))
        .await
        .map_err(|e| LandmarksError::GraphError {
            message: e.to_string(),
        })?;
    let world_row = result
        .next()
        .await
        .map_err(|e| LandmarksError::GraphError {
            message: e.to_string(),
        })?;
    Ok(world_row.is_some())
}

pub async fn world_export_by_id(
    graph: &Graph,
    world_id: &Uuid,
) -> Result<Option<World>, LandmarksError> {
    let world_match = format!("MATCH (world:World {{ id: '{}' }})", world_id);
    let detail_matches = r#"
        OPTIONAL MATCH (world)-[:HASLANDMARK]->(landmark:Landmark)
        OPTIONAL MATCH (world)-[:HASTAG]->(tag:Tag)
        MATCH (world)-[:ON]->(platform:Platform)
        MATCH (world)-[:CREATEDBY]->(creator:User)
        RETURN world, apoc.coll.toSet(collect(tag.name)) as tags, apoc.coll.toSet(collect(landmark.id)) as landmarks, platform.name as platform, creator.name as creator
        "#;

    let full_query = format!(
        "{world_match}
        {detail_matches}"
    );

    let mut world_result =
        graph
            .execute(query(&full_query))
            .await
            .map_err(|e| LandmarksError::GraphError {
                message: e.to_string(),
            })?;

    match world_result.next().await {
        Ok(Some(row)) => {
            let world_node: Node = row.get("world").ok_or(LandmarksError::GraphError {
                message: "no_world_node".to_string(),
            })?;
            let seed = world_node
                .get("seed")
                .map(Seed)
                .ok_or(LandmarksError::GraphError {
                    message: "no_world_seed".to_string(),
                })?;
            let name: Option<String> = world_node.get("name");
            let notes: Option<String> = world_node.get("notes");
            let id_value: String = world_node.get("id").ok_or(LandmarksError::GraphError {
                message: "no_world_id".to_string(),
            })?;
            let id = Uuid::parse_str(&id_value).map_err(|e| LandmarksError::InvalidUuid {
                message: e.to_string(),
            })?;
            let tag_values: Vec<String> = row.get("tags").ok_or(LandmarksError::GraphError {
                message: "no_tags_column".to_string(),
            })?;
            let tags = tag_values.into_iter().map(Tag).collect::<Vec<Tag>>();

            let platform_name: String = row.get("platform").ok_or(LandmarksError::GraphError {
                message: "no_platform_column".to_string(),
            })?;
            let platform = Platform::from_str(&platform_name)?;

            let creator: String = row.get("creator").ok_or(LandmarksError::GraphError {
                message: "no_creator_column".to_string(),
            })?;
            let landmark_ids: Vec<String> =
                row.get("landmarks").ok_or(LandmarksError::GraphError {
                    message: "no_landmarks_column".to_string(),
                })?;

            let mut landmarks: Vec<Landmark> = Vec::new();

            for landmark_id in landmark_ids {
                let id = Uuid::parse_str(&landmark_id).unwrap();
                let landmark = super::landmarks::landmark_by_id(graph, &id)
                    .await
                    .unwrap()
                    .unwrap();
                landmarks.push(landmark);
            }

            Ok(Some(World {
                metadata: WorldMetadata {
                    id,
                    seed,
                    name,
                    tags,
                    platform,
                    notes,
                    creator,
                },
                landmarks,
            }))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(LandmarksError::GraphError {
            message: e.to_string(),
        }),
    }
}

pub async fn all_for_user(graph: &Graph, user: &str) -> Result<Vec<WorldMetadata>, LandmarksError> {
    let user_match = format!("MATCH (user:User {{ name: '{}' }})", user);
    let world_matches = r#"
        MATCH (user)-[:CREATED]->(world:World)
        OPTIONAL MATCH (world)-[has_tag:HASTAG]->(tag:Tag)
        MATCH (world)-[on_platform:ON]->(platform:Platform)
        MATCH (world)-[:CREATEDBY]->(creator:User)
        RETURN world, COLLECT(tag.name) as tags, platform.name as platform, creator.name as creator
        "#;
    let shared_matches = r#"
        MATCH (world:World)-[:SHAREDWITH]->(user)
        OPTIONAL MATCH (world)-[has_tag:HASTAG]->(tag:Tag)
        MATCH (world)-[on_platform:ON]->(platform:Platform)
        MATCH (world)-[:CREATEDBY]->(creator:User)
        RETURN world, COLLECT(tag.name) as tags, platform.name as platform, creator.name as creator
        "#;
    let full_query = format!(
        "{user_match}
        {world_matches}
        UNION ALL
        {user_match}
        {shared_matches}"
    );

    let mut result =
        graph
            .execute(query(&full_query))
            .await
            .map_err(|e| LandmarksError::GraphError {
                message: e.to_string(),
            })?;
    let mut worlds: Vec<WorldMetadata> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let world_node: Node = row.get("world").ok_or(LandmarksError::GraphError {
            message: "no_world_node".to_string(),
        })?;
        let seed = world_node
            .get("seed")
            .map(Seed)
            .ok_or(LandmarksError::GraphError {
                message: "no_world_seed".to_string(),
            })?;
        let name: Option<String> = world_node.get("name");
        let notes: Option<String> = world_node.get("notes");
        let id_value: String = world_node.get("id").ok_or(LandmarksError::GraphError {
            message: "no_world_id".to_string(),
        })?;
        let id = Uuid::parse_str(&id_value).map_err(|e| LandmarksError::InvalidUuid {
            message: e.to_string(),
        })?;
        let tag_values: Vec<String> = row.get("tags").ok_or(LandmarksError::GraphError {
            message: "no_tags_column".to_string(),
        })?;
        let tags = tag_values.into_iter().map(Tag).collect::<Vec<Tag>>();

        let platform_name: String = row.get("platform").ok_or(LandmarksError::GraphError {
            message: "no_platform_column".to_string(),
        })?;
        let platform = Platform::from_str(&platform_name)?;

        let creator: String = row.get("creator").ok_or(LandmarksError::GraphError {
            message: "no_world_creator".to_string(),
        })?;

        worlds.push(WorldMetadata {
            id,
            seed,
            name,
            notes,
            tags,
            platform,
            creator,
        });
    }

    Ok(worlds)
}

pub async fn create(
    transaction: &Txn,
    user: &str,
    create_world: &CreateWorld,
) -> Result<Uuid, LandmarksError> {
    let user_match = format!("MATCH (user:User {{ name: '{}' }})", user);
    let platform_match = format!(
        "MATCH (platform:Platform {{ name: '{}' }})",
        &create_world.platform.to_string()
    );
    // Match each tag and then we'll link them to the world.
    let mut tag_merges: Vec<String> = Vec::new();
    let mut tag_variables: Vec<String> = Vec::new();
    create_world
        .tags
        .iter()
        .enumerate()
        .for_each(|(index, tag)| {
            let tag_var = format!("tag_{index}");
            tag_merges.push(format!("MERGE ({}:Tag {{ name: '{}' }})", tag_var, tag));
            tag_variables.push(tag_var);
        });
    let tag_merge = tag_merges.join("\n");
    let world_id = Uuid::new_v4();
    let world_create = format!(
        "CREATE (world:World {{ name: '{}', seed: '{}', notes: '{}', id: '{}' }})",
        create_world.guaranteed_name(),
        &create_world.seed,
        create_world.notes.clone().unwrap_or_default(),
        world_id.to_string()
    );
    let world_platform_rel = "CREATE (world)-[:ON]->(platform)";
    let world_user_rel = "CREATE (world)-[:CREATEDBY]->(user)";
    let user_world_rel = "CREATE (user)-[:CREATED]->(world)";
    let world_tag_rels = tag_variables
        .iter()
        .map(|tag_var| format!("CREATE (world)-[:HASTAG]->({})", tag_var))
        .collect::<Vec<String>>()
        .join("\n");
    let full_query = format!(
        "{user_match}
        {platform_match}
        {tag_merge}
        {world_create}
        {world_platform_rel}
        {world_user_rel}
        {user_world_rel}
        {world_tag_rels}
        RETURN world.id"
    );

    transaction
        .run(query(&full_query))
        .await
        .map_err(|e| LandmarksError::GraphError {
            message: e.to_string(),
        })?;

    Ok(world_id)
}

pub async fn share_world(
    transaction: &Txn,
    creator: &str,
    world_id: &Uuid,
    user: &str,
) -> Result<(), LandmarksError> {
    let creator_world_query = format!(
        "MATCH (world:World {{ id: '{}' }})-[:CREATEDBY]->(:User {{ name: '{}' }})",
        world_id.to_string(),
        creator
    );
    let target_query = format!("MATCH (target:User {{ name: '{}' }})", user);
    let merge_query = "MERGE (world)-[:SHAREDWITH]->(target)";
    let full_query = format!(
        "{creator_world_query}
            {target_query}
            {merge_query}
            RETURN target.name"
    );
    transaction
        .run(query(&full_query))
        .await
        .map_err(|e| LandmarksError::GraphError {
            message: e.to_string(),
        })?;

    Ok(())
}
