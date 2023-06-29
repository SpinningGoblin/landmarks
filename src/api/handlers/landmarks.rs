use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    api::auth::check_auth, config::app_state::AppState, landmarks::CreateLandmark, persistence,
};

pub async fn landmarks_for_world(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(world_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(_) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };
    let graph = app_state.to_graph().await.unwrap();
    let landmarks = persistence::landmarks::landmarks_for_world(&graph, &world_id)
        .await
        .unwrap();

    Ok(Json(landmarks))
}

pub async fn landmark_by_id(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(landmark_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(_) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };
    let graph = app_state.to_graph().await.unwrap();
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
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(world_id): Path<Uuid>,
    Json(input): Json<CreateLandmark>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(user) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };
    let graph = app_state.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();
    let id = persistence::landmarks::create(&transaction, world_id, input, &user)
        .await
        .unwrap();
    transaction.commit().await.unwrap();
    Ok(id.to_string())
}
