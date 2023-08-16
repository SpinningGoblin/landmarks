use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use landmarks_core::{
    landmarks::{CreateLandmark, LandmarkLinkType},
    persistence,
};
use uuid::Uuid;

use crate::{api::auth::check_auth, config::app_state::AppState};

use super::{
    AddBiome, AddFarm, AddLandmarkLink, AddTag, RemoveBiome, RemoveFarm, RemoveTag,
    UpdateCoordinate, UpdateNotes,
};

pub async fn list_landmark_link_types() -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok(Json(
        LandmarkLinkType::all().collect::<Vec<LandmarkLinkType>>(),
    ))
}

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

pub async fn link_landmarks(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Json(add_landmark_link): Json<AddLandmarkLink>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if check_auth(&headers, &app_state).is_none() {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };

    let graph = app_state.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();

    persistence::landmarks::link_landmarks(
        &transaction,
        &add_landmark_link.landmark_id_1,
        &add_landmark_link.landmark_id_2,
        &add_landmark_link.link_type,
    )
    .await
    .unwrap();

    Ok("OK")
}

pub async fn add_biome_to_landmark(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(landmark_id): Path<Uuid>,
    Json(add_biome): Json<AddBiome>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(_) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };

    let graph = app_state.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();

    persistence::landmarks::add_biome(&transaction, landmark_id, add_biome.biome)
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    Ok("OK")
}

pub async fn remove_biome_from_landmark(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(landmark_id): Path<Uuid>,
    Json(remove_biome): Json<RemoveBiome>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(_) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };

    let graph = app_state.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();

    persistence::landmarks::remove_biome(&transaction, landmark_id, remove_biome.biome)
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    Ok("OK")
}

pub async fn add_tag_to_landmark(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(landmark_id): Path<Uuid>,
    Json(add_tag): Json<AddTag>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(_) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };

    let graph = app_state.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();

    persistence::landmarks::add_tag(&transaction, landmark_id, add_tag.tag)
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    Ok("OK")
}

pub async fn update_notes_on_landmark(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(landmark_id): Path<Uuid>,
    Json(update_notes): Json<UpdateNotes>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(_) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };

    let graph = app_state.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();

    persistence::landmarks::update_notes(&transaction, landmark_id, &update_notes.notes)
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    Ok("OK")
}

pub async fn update_coordinate_on_landmark(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(landmark_id): Path<Uuid>,
    Json(update_coordinate): Json<UpdateCoordinate>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(_) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };

    let graph = app_state.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();

    persistence::landmarks::update_coordinate(
        &transaction,
        landmark_id,
        &update_coordinate.coordinate,
    )
    .await
    .unwrap();

    transaction.commit().await.unwrap();

    Ok("OK")
}

pub async fn remove_tag_from_landmark(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(landmark_id): Path<Uuid>,
    Json(remove_tag): Json<RemoveTag>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(_) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };

    let graph = app_state.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();

    persistence::landmarks::remove_tag(&transaction, landmark_id, remove_tag.tag)
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    Ok("OK")
}

pub async fn add_farm_to_landmark(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(landmark_id): Path<Uuid>,
    Json(add_farm): Json<AddFarm>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(_) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };

    let graph = app_state.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();

    persistence::landmarks::add_farm(&transaction, landmark_id, add_farm.farm)
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    Ok("OK")
}

pub async fn remove_farm_from_landmark(
    State(app_state): State<AppState>,
    headers: HeaderMap,
    Path(landmark_id): Path<Uuid>,
    Json(remove_farm): Json<RemoveFarm>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let Some(_) = check_auth(&headers, &app_state) else {
        return Err((StatusCode::UNAUTHORIZED, "no_auth".to_string()));
    };

    let graph = app_state.to_graph().await.unwrap();
    let transaction = graph.start_txn().await.unwrap();

    persistence::landmarks::remove_farm(&transaction, landmark_id, remove_farm.farm)
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    Ok("OK")
}
