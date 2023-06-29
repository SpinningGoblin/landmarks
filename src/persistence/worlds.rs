use std::str::FromStr;

use neo4rs::{query, Graph, Node, Query, Txn};
use uuid::Uuid;

use crate::{
    minecraft::{Platform, Seed},
    worlds::{CreateWorld, WorldMetadata},
    Tag,
};

fn world_match_query(world_id: &Uuid) -> Query {
    let world_match = format!(
        "MATCH (world:World {{ id: '{}' }}) RETURN world.id",
        world_id.to_string()
    );
    query(&world_match)
}

pub async fn check_world_exists(transaction: &Txn, world_id: &Uuid) -> Result<bool, anyhow::Error> {
    let mut result = transaction.execute(world_match_query(world_id)).await?;
    let world_row = result.next().await?;
    Ok(world_row.is_some())
}

pub async fn all_for_user(graph: &Graph, user: &str) -> Result<Vec<WorldMetadata>, anyhow::Error> {
    let user_match = format!("MATCH (user:User {{ name: '{}' }})", user);
    let world_matches = r#"
        MATCH (user)-[:CREATED]->(world:World)
        MATCH (world)-[has_tag:HASTAG]->(tag:Tag)
        MATCH (world)-[on_platform:ON]->(platform:Platform)
        MATCH (world)-[:CREATEDBY]->(creator:User)
        RETURN world, COLLECT(tag.name) as tags, platform.name as platform, creator.name as creator
        "#;
    let shared_matches = r#"
        MATCH (user:User { name: 'derrick' })
        MATCH (world:World)-[:SHAREDWITH]->(user)
        MATCH (world)-[has_tag:HASTAG]->(tag:Tag)
        MATCH (world)-[on_platform:ON]->(platform:Platform)
        MATCH (world)-[:CREATEDBY]->(creator:User)
        OPTIONAL MATCH (world)-[:SHAREDWITH]->(shared:User)
        RETURN world, COLLECT(tag.name) as tags, platform.name as platform, creator.name as creator
        "#;
    let full_query = format!(
        "{user_match}
        {world_matches}
        UNION ALL
        {user_match}
        {shared_matches}"
    );
    println!("{full_query}");

    let mut result = graph.execute(query(&full_query)).await?;
    let mut worlds: Vec<WorldMetadata> = Vec::new();

    while let Ok(Some(row)) = result.next().await {
        let world_node: Node = row
            .get("world")
            .ok_or(anyhow::Error::msg("no_world_node"))?;
        let seed = world_node
            .get("seed")
            .map(Seed)
            .ok_or(anyhow::Error::msg("no_world_seed"))?;
        let name: Option<String> = world_node.get("name");
        let notes: Option<String> = world_node.get("notes");
        let id_value: String = world_node
            .get("id")
            .ok_or(anyhow::Error::msg("no_world_id"))?;
        let id = Uuid::parse_str(&id_value)?;
        let tag_values: Vec<String> = row
            .get("tags")
            .ok_or(anyhow::Error::msg("no_tags_column"))?;
        let tags = tag_values
            .into_iter()
            .map(|name| Tag(name))
            .collect::<Vec<Tag>>();

        let platform_name: String = row
            .get("platform")
            .ok_or(anyhow::Error::msg("no_platform_name"))?;
        let platform = Platform::from_str(&platform_name)?;

        let creator: String = row.get("creator").ok_or(anyhow::Error::msg("no_creator"))?;

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
) -> Result<Uuid, anyhow::Error> {
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

    transaction.run(query(&full_query)).await?;

    Ok(world_id)
}

pub async fn share_world(
    transaction: &Txn,
    creator: &str,
    world_id: &Uuid,
    user: &str,
) -> Result<(), anyhow::Error> {
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
    println!("{full_query}");
    transaction.run(query(&full_query)).await?;

    Ok(())
}
