use crate::services;
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

pub async fn list_items(State(pool): State<sqlx::SqlitePool>) -> impl IntoResponse {
    let result = services::item::get_items(&pool).await;

    match result {
        Ok(items) => (StatusCode::OK, Json(json!({ "data": items }))),
        Err(error) => {
            println!("Error: {}", error);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "data": null })),
            );
        }
    }
}

pub async fn show_item(
    Path(id): Path<u32>,
    State(pool): State<sqlx::SqlitePool>,
    Extension(_client): Extension<reqwest::Client>,
) -> impl IntoResponse {
    let result = services::item::get_item(&pool, id).await;

    match result {
        Ok(item) => (StatusCode::OK, Json(json!({ "data": item }))),
        Err(_) => (StatusCode::NOT_FOUND, Json(json!({ "data": null }))),
    }
}
