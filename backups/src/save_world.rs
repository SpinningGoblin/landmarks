use aws_sdk_s3::primitives::ByteStream;
use neo4rs::Graph;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_world(
    world_id: &Uuid,
    graph: &Graph,
    app_state: &AppState,
) -> Result<String, anyhow::Error> {
    let Ok(world) = landmarks_core::persistence::worlds::world_export_by_id(graph, world_id).await
    else {
        return Err(anyhow::Error::msg("no world"));
    };

    println!("retrieved world");

    let body = serde_json::to_string(&world).unwrap();
    let bytes = ByteStream::from(body.as_bytes().to_vec());

    println!("created byte strea,");

    let client = app_state.aws_client().await;
    println!("have aws client");
    let put_object = client
        .put_object()
        .bucket("landmarks-backup")
        .key(format!("{world_id}.json"))
        .body(bytes)
        .content_type("application/json");
    let result = put_object.send().await?;
    Ok(result.e_tag().unwrap().to_string())
}
