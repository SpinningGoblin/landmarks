use std::str::FromStr;

use crate::{
    minecraft::{Platform, Seed},
    users::User,
    LandmarksError, Tag,
};
use neo4rs::{query, Graph, Query, Txn};
use serde::Deserialize;
use time::{format_description::well_known, OffsetDateTime};
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
    transaction: &mut Txn,
    world_id: &Uuid,
) -> Result<bool, LandmarksError> {
    let mut result = transaction.execute(world_match_query(world_id)).await?;
    let world_row = result.next(transaction).await?;
    Ok(world_row.is_some())
}

pub async fn set_world_updated_at_now(
    transaction: &mut Txn,
    world_id: &Uuid,
) -> Result<(), LandmarksError> {
    let now = time::OffsetDateTime::now_utc()
        .format(&well_known::Rfc3339)
        .unwrap();
    let world_match = format!("MATCH (world:World {{ id: '{}' }})", world_id);
    let full_query = format!(
        "{world_match}
        SET world.updated_at = '{now}'
        RETURN world.id",
    );
    transaction.run(query(&full_query)).await?;

    Ok(())
}

#[derive(Deserialize)]
struct WorldNode {
    pub seed: Seed,
    pub name: Option<String>,
    pub notes: Option<String>,
    pub id: Uuid,
    pub updated_at: String,
}

#[derive(Deserialize)]
struct WorldRow {
    pub world: WorldNode,
    pub tags: Vec<Tag>,
    pub platform: String,
    pub creator: String,
    pub shared_users: Vec<String>,
}

#[derive(Deserialize)]
struct FullWorldRow {
    pub world: WorldNode,
    pub tags: Vec<Tag>,
    pub platform: String,
    pub creator: String,
    pub shared_users: Vec<String>,
    pub landmarks: Vec<String>,
}

pub async fn world_export_by_id(
    graph: &Graph,
    world_id: &Uuid,
) -> Result<Option<World>, LandmarksError> {
    let world_match = format!("MATCH (world:World {{ id: '{}' }})", world_id);
    let detail_matches = r"
        OPTIONAL MATCH (world)-[:HASLANDMARK]->(landmark:Landmark)
        OPTIONAL MATCH (world)-[:HASTAG]->(tag:Tag)
        OPTIONAL MATCH (world)-[:SHAREDWITH]->(shared_user:User)
        MATCH (world)-[:ON]->(platform:Platform)
        MATCH (world)-[:CREATEDBY]->(creator:User)
        RETURN world,
        apoc.coll.toSet(collect(tag.name)) as tags,
        apoc.coll.toSet(collect(landmark.id)) as landmarks,
        apoc.coll.toSet(collect(shared_user.name)) as shared_users,
        platform.name as platform,
        creator.name as creator
        ";

    let full_query = format!(
        "{world_match}
        {detail_matches}"
    );

    let mut world_result = graph.execute(query(&full_query)).await?;

    match world_result.next().await {
        Ok(Some(row)) => {
            let world_row = row.to::<FullWorldRow>().map_err(|e| {
                LandmarksError::GraphDeserializationError {
                    message: e.to_string(),
                }
            })?;
            let updated_at: Option<OffsetDateTime> =
                OffsetDateTime::parse(&world_row.world.updated_at, &well_known::Rfc3339).ok();

            let platform = Platform::from_str(&world_row.platform).map_err(|e| {
                LandmarksError::GraphDeserializationError {
                    message: e.to_string(),
                }
            })?;

            let mut landmarks: Vec<Landmark> = Vec::new();

            for landmark_id in world_row.landmarks {
                let id = Uuid::from_str(&landmark_id).unwrap();
                let landmark = super::landmarks::landmark_by_id(graph, &id).await?.unwrap();
                landmarks.push(landmark);
            }

            let shared_users = world_row
                .shared_users
                .into_iter()
                .map(|name| User { name })
                .collect::<Vec<User>>();

            Ok(Some(World {
                metadata: WorldMetadata {
                    id: world_row.world.id,
                    seed: world_row.world.seed,
                    name: world_row.world.name,
                    tags: world_row.tags,
                    platform,
                    notes: world_row.world.notes,
                    creator: User {
                        name: world_row.creator,
                    },
                    shared_users,
                    updated_at,
                },
                landmarks,
            }))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(LandmarksError::from(e)),
    }
}

pub async fn all_for_user(graph: &Graph, user: &str) -> Result<Vec<WorldMetadata>, LandmarksError> {
    let user_match = format!("MATCH (user:User {{ name: '{}' }})", user);
    let world_matches = r"
        MATCH (user)-[:CREATED]->(world:World)
        OPTIONAL MATCH (world)-[has_tag:HASTAG]->(tag:Tag)
        MATCH (world)-[on_platform:ON]->(platform:Platform)
        MATCH (world)-[:CREATEDBY]->(creator:User)
        OPTIONAL MATCH (world)-[:SHAREDWITH]->(shared_user:User)
        RETURN world,
        apoc.coll.toSet(collect(tag.name)) as tags,
        apoc.coll.toSet(collect(shared_user.name)) as shared_users,
        platform.name as platform,
        creator.name as creator
        ";
    let shared_matches = r"
        MATCH (world:World)-[:SHAREDWITH]->(user)
        OPTIONAL MATCH (world)-[has_tag:HASTAG]->(tag:Tag)
        MATCH (world)-[on_platform:ON]->(platform:Platform)
        MATCH (world)-[:CREATEDBY]->(creator:User)
        OPTIONAL MATCH (world)-[:SHAREDWITH]->(shared_user:User)
        RETURN world,
        apoc.coll.toSet(collect(tag.name)) as tags,
        apoc.coll.toSet(collect(shared_user.name)) as shared_users,
        platform.name as platform,
        creator.name as creator
        ";
    let full_query = format!(
        "{user_match}
        {world_matches}
        UNION ALL
        {user_match}
        {shared_matches}"
    );

    let mut result = graph.execute(query(&full_query)).await?;
    let mut worlds: Vec<WorldMetadata> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let world_row =
            row.to::<WorldRow>()
                .map_err(|e| LandmarksError::GraphDeserializationError {
                    message: e.to_string(),
                })?;

        let updated_at: Option<OffsetDateTime> =
            OffsetDateTime::parse(&world_row.world.updated_at, &well_known::Rfc3339).ok();

        let shared_users = world_row
            .shared_users
            .into_iter()
            .map(|name| User { name })
            .collect::<Vec<User>>();

        worlds.push(WorldMetadata {
            id: world_row.world.id,
            seed: world_row.world.seed,
            name: world_row.world.name,
            notes: world_row.world.notes,
            tags: world_row.tags,
            platform: Platform::from_str(&world_row.platform).unwrap(),
            creator: User {
                name: world_row.creator,
            },
            shared_users,
            updated_at,
        });
    }

    Ok(worlds)
}

pub async fn create(
    transaction: &mut Txn,
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
    let created_at = time::OffsetDateTime::now_utc()
        .format(&well_known::Rfc3339)
        .unwrap();
    let notes = create_world.notes.clone().unwrap_or_default();
    let seed = create_world.seed.clone();
    let name = create_world.guaranteed_name();
    let world_create = format!(
        "CREATE (world:World {{ name: '{name}', seed: '{seed}', notes: '{notes}', id: '{world_id}', updated_at: '{created_at}' }})",
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

    transaction.run(query(&full_query)).await?;

    Ok(world_id)
}

pub async fn share_world(
    transaction: &mut Txn,
    creator: &str,
    world_id: &Uuid,
    user: &str,
) -> Result<(), LandmarksError> {
    let creator_world_query = format!(
        "MATCH (world:World {{ id: '{}' }})-[:CREATEDBY]->(:User {{ name: '{}' }})",
        world_id, creator
    );
    let target_query = format!("MATCH (target:User {{ name: '{}' }})", user);
    let merge_query = "MERGE (world)-[:SHAREDWITH]->(target)";
    let full_query = format!(
        "{creator_world_query}
            {target_query}
            {merge_query}
            RETURN target.name"
    );
    transaction.run(query(&full_query)).await?;
    set_world_updated_at_now(transaction, world_id).await?;

    Ok(())
}
