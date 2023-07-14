use aws_sdk_s3::primitives::ByteStream;
use neo4rs::Graph;
use uuid::Uuid;

use crate::{config::app_state::AppState, persistence};

pub async fn backup(world_id: &Uuid, graph: &Graph, app_state: AppState) {
    if !app_state.should_backup_world(world_id) {
        return;
    }

    let Ok(world) = persistence::worlds::world_export_by_id(graph, world_id).await else {
        return;
    };

    let body = serde_json::to_string(&world).unwrap();
    let bytes = ByteStream::from(body.as_bytes().to_vec());

    let client = app_state.aws_client().await;
    let put_object = client
        .put_object()
        .bucket("landmarks-backup")
        .key(format!("{world_id}.json"))
        .body(bytes)
        .content_type("application/json");
    match put_object.send().await {
        Ok(it) => println!("{:?}", it.e_tag()),
        Err(e) => println!("{e:?}"),
    }
}
