use axum::{
    routing::{get, post},
    Router,
};
use landmarks::{config, persistence};

#[tokio::main]
async fn main() {
    let neo4j_config = config::neo4j::ConnectionConfig::load_env().unwrap();
    let graph = neo4j_config.to_graph().await.unwrap();

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
        .with_state(neo4j_config);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8082").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
