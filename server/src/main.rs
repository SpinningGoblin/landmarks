use axum::{
    routing::{get, post},
    Router,
};
use landmarks::{
    api::handlers::{
        health_check::ping,
        landmarks::{
            add_biome_to_landmark, add_farm_to_landmark, add_landmark_to_world,
            add_tag_to_landmark, landmark_by_id, landmarks_for_world, link_landmarks,
            list_landmark_link_types, remove_biome_from_landmark, remove_farm_from_landmark,
            remove_tag_from_landmark, update_coordinate_on_landmark, update_notes_on_landmark,
        },
        minecraft::{list_biomes, list_dimensions, list_platforms},
        users::list_users,
        worlds::{create_world, share_world, world_export, worlds_for_user},
    },
    config::app_state::AppState,
};
use landmarks_core::persistence::minecraft::ensure_minecraft_nodes;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    println!("running the server");

    let app_state = AppState::load_from_env();
    let graph = app_state.connection_config().to_graph().await.unwrap();

    ensure_minecraft_nodes(&graph).await.unwrap();
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/dimensions", get(list_dimensions))
        .route("/biomes", get(list_biomes))
        .route("/platforms", get(list_platforms))
        .route("/worlds", get(worlds_for_user))
        .route("/worlds", post(create_world))
        .route("/worlds/:world_id/share", post(share_world))
        .route("/worlds/:world_id/landmarks", post(add_landmark_to_world))
        .route("/worlds/:world_id/landmarks", get(landmarks_for_world))
        .route("/worlds/:world_id/export", get(world_export))
        .route("/landmarks/link", post(link_landmarks))
        .route("/landmarks/link_types", get(list_landmark_link_types))
        .route("/landmarks/:landmark_id", get(landmark_by_id))
        .route(
            "/landmarks/:landmark_id/add_farm",
            post(add_farm_to_landmark),
        )
        .route("/landmarks/:landmark_id/add_tag", post(add_tag_to_landmark))
        .route(
            "/landmarks/:landmark_id/remove_farm",
            post(remove_farm_from_landmark),
        )
        .route(
            "/landmarks/:landmark_id/remove_tag",
            post(remove_tag_from_landmark),
        )
        .route(
            "/landmarks/:landmark_id/add_biome",
            post(add_biome_to_landmark),
        )
        .route(
            "/landmarks/:landmark_id/remove_biome",
            post(remove_biome_from_landmark),
        )
        .route(
            "/landmarks/:landmark_id/update_notes",
            post(update_notes_on_landmark),
        )
        .route(
            "/landmarks/:landmark_id/update_coordinate",
            post(update_coordinate_on_landmark),
        )
        .route("/users", get(list_users))
        .with_state(app_state)
        .layer(CorsLayer::very_permissive());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
