mod controllers;
mod models;
mod services;
mod utils;

use anyhow;
use axum::{extract::Extension, routing::get, Router};
use dotenvy;
use reqwest::Client;
use sqlx;
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "osrs_price_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::debug!("Running server");

    dotenvy::dotenv().ok();

    let db = &env::var("DATABASE_URL").unwrap_or("sqlite:local.db".to_string());

    let pool = sqlx::sqlite::SqlitePool::connect(db).await?;
    tracing::debug!("Migrating");

    sqlx::migrate!().run(&pool).await?;

    tracing::debug!("Migration complete");

    let client = Client::builder()
        .user_agent(&env::var("USER_AGENT")?)
        .build()
        .expect("Could not create client");

    // @todo change this to run on a schedule or CLI
    tracing::debug!("Syncing");
    utils::sync::sync_mapping(&pool, &client).await?;
    utils::sync::sync_prices(&pool, &client).await?;
    tracing::debug!("Syncing complete");

    let app = Router::new()
        .route("/health", get(|| async { "Hello, world!" }))
        .route("/items", get(controllers::item::list_items))
        .route("/items/:id", get(controllers::item::show_item))
        .with_state(pool)
        .layer(Extension(client));

    tracing::debug!("Listening on 0.0.0.0:3400");

    axum::Server::bind(&"0.0.0.0:3400".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
