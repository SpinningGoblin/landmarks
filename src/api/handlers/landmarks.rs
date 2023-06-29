use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{config::neo4j::ConnectionConfig, landmarks::CreateLandmark, persistence};

use super::headers::USER_HEADER;

pub async fn add_landmark_to_world(
    State(neo4j_config): State<ConnectionConfig>,
    headers: HeaderMap,
    Path(world_id): Path<Uuid>,
    Json(input): Json<CreateLandmark>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_header = headers
        .get(USER_HEADER)
        .ok_or((StatusCode::UNAUTHORIZED, "no_auth".to_string()))?;
    let user = user_header
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "no_auth".to_string()))?;
    let graph = neo4j_config.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();
    let id = persistence::landmarks::create(&transaction, world_id, input, user)
        .await
        .unwrap();
    transaction.commit().await.unwrap();
    Ok(id.to_string())
}
