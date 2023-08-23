mod controllers;
mod models;
mod utils;

use anyhow;
use axum::{extract::Extension, routing::get, Router};
use dotenvy;
use reqwest::Client;
use sqlx;
use std::{env, error};

#[tokio::main]
async fn main() -> anyhow::Result<(), Box<dyn error::Error>> {
    println!("[App] Running server");

    dotenvy::dotenv().ok();

    let db = &env::var("DATABASE_URL").unwrap_or("sqlite:local.db".to_string());

    let pool = sqlx::sqlite::SqlitePool::connect(db).await?;
    println!("[App] Migrating");

    sqlx::migrate!().run(&pool).await?;

    println!("[App] Migration complete");

    let client = Client::builder()
        .user_agent(&env::var("USER_AGENT")?)
        .build()
        .expect("Could not create client");

    // @todo change this to run on a schedule or CLI
    println!("[App] Syncing");
    utils::sync::sync_mapping(&pool, &client).await?;
    utils::sync::sync_prices(&pool, &client).await?;
    println!("[App] Syncing complete");

    let app = Router::new()
        .route("/health", get(|| async { "Hello, world!" }))
        .route("/items", get(controllers::item::list_items))
        .route("/items/:id", get(controllers::item::show_item))
        .layer(Extension(client))
        .layer(Extension(pool));

    println!("[App] Listening on 0.0.0.0:3400");

    axum::Server::bind(&"0.0.0.0:3400".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
