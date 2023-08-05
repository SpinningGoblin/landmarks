use aws_sdk_s3::primitives::ByteStream;
use chrono::Utc;
use neo4rs::Graph;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_world(
    world_id: &Uuid,
    graph: &Graph,
    app_state: &mut AppState,
) -> Result<String, anyhow::Error> {
    let Ok(Some(world)) =
        landmarks_core::persistence::worlds::world_export_by_id(graph, world_id).await
    else {
        return Err(anyhow::Error::msg("no world"));
    };

    println!("retrieved world");
    let should_back_up = app_state.need_to_backup_world(&world);

    let backup_result = if should_back_up {
        let body = serde_json::to_string(&world).unwrap();
        let bytes = ByteStream::from(body.as_bytes().to_vec());

        println!("created byte stream");

        let client = app_state.aws_client().await;
        println!("have aws client");
        let put_object = client
            .put_object()
            .bucket("landmarks-backup")
            .key(format!("{world_id}.json"))
            .body(bytes)
            .content_type("application/json");
        let result = put_object.send().await?;
        result.e_tag().unwrap().to_string()
    } else {
        "do not need to backup".to_string()
    };

    app_state.update_last_backed_up(Utc::now());

    Ok(backup_result)
}
