mod controllers;
mod models;
mod utils;

use anyhow::Result;
use axum::{extract::Extension, routing::get, Router};
use reqwest::Client;
use sqlx::sqlite::SqlitePool;
use std::{env, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("[App] Running server");

    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    println!("[App] Migrating");

    sqlx::migrate!("db/migrations").run(&pool).await?;

    println!("[App] Migration complete");

    let client = Client::builder()
        .user_agent(&env::var("USER_AGENT")?)
        .build()
        .expect("Could not create client");

    println!("[App] Mapping");
    // @todo change this to run on a schedule or CLI
    utils::sync::sync_mapping(&pool, &client).await?;
    println!("[App] Mapping complete");

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
