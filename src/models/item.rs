// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Item {
    id: u32,
    is_members: bool,
    alch_low: u32,
    alch_high: u32,
    buy_limit: u32,
    value: u32,
    buy_price: u32,
    buy_price_timestamp: u32,
    sell_price: u32,
    sell_price_timestamp: u32,
    icon: String,
    examine_text: String,
    last_updated: u32,
}
