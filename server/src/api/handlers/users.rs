use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use landmarks_core::persistence;

use crate::{api::auth::check_auth, config::app_state::AppState};

pub async fn list_users(
    State(app_state): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_auth(&headers, &app_state).is_none() {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };

    let graph = app_state.connection_config().to_graph().await.unwrap();
    let users = persistence::users::list_users(&graph).await.unwrap();

    Ok(Json(users))
}
