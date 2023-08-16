use aws_sdk_s3::primitives::ByteStream;
use chrono::Utc;
use uuid::Uuid;

use crate::config::AppState;

pub async fn save_world(
    world_id: &Uuid,
    app_state: &mut AppState,
) -> Result<String, anyhow::Error> {
    println!("Starting save of world");
    let graph = app_state.connection_config().to_graph().await.unwrap();
    let Ok(Some(world)) =
        landmarks_core::persistence::worlds::world_export_by_id(&graph, world_id).await
    else {
        println!("No world with id {world_id}");
        return Err(anyhow::Error::msg("no world"));
    };

    if let Some(backup_date) = &app_state.last_backed_up_date {
        println!("{backup_date:?}");
    } else {
        println!("Fresh start of app, no last backup date");
    }

    println!("retrieved world");
    let should_back_up = app_state.need_to_backup_world(&world);

    let backup_result = if should_back_up {
        println!("Starting backup of world");
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
        println!("Sending to S3");
        let result = put_object.send().await?;
        println!("Sent to S3");
        result.e_tag().unwrap().to_string()
    } else {
        "do not need to backup".to_string()
    };

    app_state.update_last_backed_up(Utc::now());

    Ok(backup_result)
}
