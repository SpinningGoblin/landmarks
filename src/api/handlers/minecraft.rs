use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{config::app_state::AppState, persistence};

pub async fn list_dimensions(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let graph = app_state.to_graph().await.unwrap();
    let dimensions = persistence::minecraft::list_dimensions(&graph)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(dimensions))
}

pub async fn list_biomes(
    State(app_state): State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let graph = app_state.to_graph().await.unwrap();
    let biomes = persistence::minecraft::list_biomes(&graph)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(biomes))
}
