use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{config::neo4j::ConnectionConfig, persistence, worlds::CreateWorld};

use super::headers::USER_HEADER;

pub async fn worlds_for_user(
    State(neo4j_config): State<ConnectionConfig>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let user_header = headers
        .get(USER_HEADER)
        .ok_or((StatusCode::UNAUTHORIZED, "no_auth".to_string()))?;
    let user = user_header
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "no_auth".to_string()))?;
    let graph = neo4j_config.to_graph().await.unwrap();
    let worlds = persistence::worlds::all_for_user(&graph, user)
        .await
        .unwrap();

    Ok(Json(worlds))
}

#[derive(Debug, Deserialize)]
pub struct Create {
    details: CreateWorld,
}

pub async fn create_world(
    State(neo4j_config): State<ConnectionConfig>,
    headers: HeaderMap,
    Json(input): Json<Create>,
) -> Result<String, (StatusCode, String)> {
    let user_header = headers
        .get(USER_HEADER)
        .ok_or((StatusCode::UNAUTHORIZED, "no_auth".to_string()))?;
    let user = user_header
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "no_auth".to_string()))?;
    let graph = neo4j_config.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();
    let id = persistence::worlds::create(&transaction, user, &input.details)
        .await
        .unwrap();
    transaction.commit().await.unwrap();
    Ok(id.to_string())
}

#[derive(Debug, Deserialize)]
pub struct ShareWorld {
    user: String,
}

pub async fn share_world(
    State(neo4j_config): State<ConnectionConfig>,
    headers: HeaderMap,
    Path(world_id): Path<Uuid>,
    Json(input): Json<ShareWorld>,
) -> Result<String, (StatusCode, String)> {
    let user_header = headers
        .get(USER_HEADER)
        .ok_or((StatusCode::UNAUTHORIZED, "no_auth".to_string()))?;
    let user = user_header
        .to_str()
        .map_err(|_| (StatusCode::UNAUTHORIZED, "no_auth".to_string()))?;
    let graph = neo4j_config.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();

    persistence::worlds::share_world(&transaction, user, &world_id, &input.user)
        .await
        .unwrap();
    transaction.commit().await.unwrap();

    Ok("Ok".to_string())
}
