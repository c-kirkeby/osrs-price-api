use crate::models::item;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

pub async fn list_items(Extension(pool): Extension<sqlx::SqlitePool>) -> impl IntoResponse {
    let query = r#"
        SELECT id
             , is_members
             , alch_low
             , alch_high
             , buy_limit
             , value
             , icon
             , examine_text 
             , last_updated
          FROM items"#;

    let result = sqlx::query_as::<_, item::Item>(query)
        .fetch_all(&pool)
        .await;

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
    Path(id): Path<u64>,
    Extension(pool): Extension<sqlx::SqlitePool>,
    Extension(_client): Extension<reqwest::Client>,
) -> impl IntoResponse {
    let query = r#"
        SELECT id
             , is_members
             , alch_low
             , alch_high
             , buy_limit
             , value
             , icon
             , examine_text 
          FROM items 
         WHERE id = $1"#;

    let result = sqlx::query_as::<_, item::Item>(query)
        .bind(id as i64)
        .fetch_one(&pool)
        .await;

    // let upstream_item = client.get("https://prices.runescape.wiki/api/v1/osrs");

    match result {
        Ok(item) => (StatusCode::OK, Json(json!({ "data": item }))),
        Err(_) => (StatusCode::NOT_FOUND, Json(json!({ "data": null }))),
    }
}
