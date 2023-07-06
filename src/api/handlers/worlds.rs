use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{api::auth::check_auth, config::app_state::AppState, persistence, worlds::CreateWorld};

pub async fn worlds_for_user(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(user) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };

    let graph = app_state.to_graph().await.unwrap();
    let worlds = persistence::worlds::all_for_user(&graph, &user)
        .await
        .unwrap();

    Ok(Json(worlds))
}

pub async fn create_world(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<CreateWorld>,
) -> Result<String, (StatusCode, String)> {
    let Some(user) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };
    let graph = app_state.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();
    let id = persistence::worlds::create(&transaction, &user, &input)
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
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(world_id): Path<Uuid>,
    Json(input): Json<ShareWorld>,
) -> Result<String, (StatusCode, String)> {
    let Some(user) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };
    let graph = app_state.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();

    persistence::worlds::share_world(&transaction, &user, &world_id, &input.user)
        .await
        .unwrap();
    transaction.commit().await.unwrap();

    Ok("Ok".to_string())
}
