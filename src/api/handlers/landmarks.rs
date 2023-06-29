use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{config::neo4j::ConnectionConfig, landmarks::CreateLandmark, persistence};

use super::headers::USER_HEADER;

pub async fn landmarks_for_world(
    State(neo4j_config): State<ConnectionConfig>,
    headers: HeaderMap,
    Path(world_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_header = headers
        .get(USER_HEADER)
        .ok_or((StatusCode::UNAUTHORIZED, "no_auth".to_string()))?;
    user_header
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "no_auth".to_string()))?;
    let graph = neo4j_config.to_graph().await.unwrap();
    let landmarks = persistence::landmarks::landmarks_for_world(&graph, &world_id)
        .await
        .unwrap();

    Ok(Json(landmarks))
}

pub async fn landmark_by_id(
    State(neo4j_config): State<ConnectionConfig>,
    headers: HeaderMap,
    Path(landmark_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_header = headers
        .get(USER_HEADER)
        .ok_or((StatusCode::UNAUTHORIZED, "no_auth".to_string()))?;
    user_header
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "no_auth".to_string()))?;
    let graph = neo4j_config.to_graph().await.unwrap();
    let landmark = persistence::landmarks::landmark_by_id(&graph, &landmark_id)
        .await
        .unwrap();

    if let Some(it) = landmark {
        Ok(Json(it))
    } else {
        Err((StatusCode::NOT_FOUND, "no_landmark_with_id".to_string()))
    }
}

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
