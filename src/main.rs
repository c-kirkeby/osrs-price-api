mod controllers;
mod models;

use anyhow::Result;
use axum::{extract::Extension, routing::get, Router};
use sqlx::sqlite::SqlitePool;
use std::{env, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("[App] Running server");

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    println!("[App] Migrating");

    sqlx::migrate!("db/migrations").run(&pool).await?;

    println!("[App] Migration complete");

    let app = Router::new()
        .route("/health", get(|| async { "Hello, world!" }))
        .route("/items", get(controllers::item::list_items))
        .route("/items/:id", get(controllers::item::show_item))
        .layer(Extension(pool));

    println!("[App] Listening on 0.0.0.0:3400");

    axum::Server::bind(&"0.0.0.0:3400".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
