use axum::{
    routing::{get, post},
    Router,
};
use landmarks::{config::app_state::AppState, persistence};

#[tokio::main]
async fn main() {
    let app_state = AppState::load_from_env();
    let graph = app_state.to_graph().await.unwrap();

    persistence::minecraft::ensure_minecraft_nodes(&graph)
        .await
        .unwrap();
    let app = Router::new()
        .route("/ping", get(landmarks::api::handlers::ping))
        .route("/worlds", get(landmarks::api::handlers::worlds_for_user))
        .route("/worlds", post(landmarks::api::handlers::create_world))
        .route(
            "/worlds/:world_id/share",
            post(landmarks::api::handlers::share_world),
        )
        .route(
            "/worlds/:world_id/landmarks",
            post(landmarks::api::handlers::add_landmark_to_world),
        )
        .route(
            "/worlds/:world_id/landmarks",
            get(landmarks::api::handlers::landmarks_for_world),
        )
        .route(
            "/landmarks/:landmark_id",
            get(landmarks::api::handlers::landmark_by_id),
        )
        .with_state(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
