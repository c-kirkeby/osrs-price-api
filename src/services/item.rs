use crate::models::item::Item;
use anyhow::Result;
use sqlx::SqlitePool;

pub async fn get_item(pool: &SqlitePool, id: u32) -> Result<Item> {
    let item = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(item)
}

pub async fn get_items(pool: &SqlitePool) -> Result<Vec<Item>> {
    let items = sqlx::query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(pool)
        .await?;

    Ok(items)
}
