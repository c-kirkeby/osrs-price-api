use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Serialize, Deserialize)]
struct Mapping {
    examine: String,
    members: bool,
    id: i32,
    lowalch: Option<i32>,
    limit: Option<i32>,
    value: i32,
    highalch: Option<i32>,
    icon: String,
    name: String,
}

pub async fn sync_mapping(
    pool: &SqlitePool,
    client: &reqwest::Client,
) -> Result<(), Box<dyn std::error::Error>> {
    let upstream_response = client
        .get("https://prices.runescape.wiki/api/v1/osrs/mapping")
        .send()
        .await?
        .text()
        .await?;

    let mappings: Vec<Mapping> = serde_json::from_str(&upstream_response)?;

    let mut transaction = pool.begin().await?;

    let upsert_query = r#"
        INSERT INTO items (
            id,
            is_members,
            examine_text,
            alch_low,
            alch_high,
            buy_limit,
            icon,
            value,
            last_updated
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT (id) DO UPDATE SET
            is_members = excluded.is_members,
            examine_text = excluded.examine_text,
            alch_low = excluded.alch_low,
            alch_high = excluded.alch_high,
            buy_limit = excluded.buy_limit,
            icon = excluded.icon,
            value = excluded.value,
            last_updated = strftime('%s', 'now')    
    "#;

    for mapping in mappings {
        sqlx::query(upsert_query)
            .bind(mapping.id)
            .bind(mapping.members)
            .bind(mapping.examine.clone())
            .bind(mapping.lowalch)
            .bind(mapping.highalch)
            .bind(mapping.limit)
            .bind(mapping.icon.clone())
            .bind(mapping.value)
            .execute(&mut *transaction)
            .await?;
    }

    transaction.commit().await?;

    Ok(())
}
