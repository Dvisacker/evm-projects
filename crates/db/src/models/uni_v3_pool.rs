use crate::schema::uni_v3_pools;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde_json::Value as JsonValue;

use super::db_pool::DbPool;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = uni_v3_pools)]
pub struct DbUniV3Pool {
    pub id: i32,
    pub address: String,
    pub chain: String,
    pub token_a: String,
    pub token_a_decimals: i32,
    pub token_a_symbol: String,
    pub token_b: String,
    pub token_b_decimals: i32,
    pub token_b_symbol: String,
    pub liquidity: Option<String>,
    pub sqrt_price: Option<String>,
    pub fee: Option<i32>,
    pub tick: Option<i32>,
    pub tick_spacing: Option<i32>,
    pub tick_bitmap: Option<JsonValue>,
    pub ticks: Option<JsonValue>,
    pub exchange_name: Option<String>,
    pub exchange_type: Option<String>,
    pub factory_address: Option<String>,
    pub active: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub tag: Option<String>,
}

impl From<DbUniV3Pool> for DbPool {
    fn from(pool: DbUniV3Pool) -> Self {
        DbPool::UniV3(pool)
    }
}

#[derive(Insertable, Debug)]
#[diesel(table_name = uni_v3_pools)]
pub struct NewDbUniV3Pool {
    pub address: String,
    pub chain: String,
    pub token_a: String,
    pub token_a_decimals: i32,
    pub token_a_symbol: String,
    pub token_b: String,
    pub token_b_decimals: i32,
    pub token_b_symbol: String,
    pub liquidity: Option<String>,
    pub sqrt_price: Option<String>,
    pub fee: Option<i32>,
    pub tick: Option<i32>,
    pub tick_spacing: Option<i32>,
    pub tick_bitmap: Option<JsonValue>,
    pub ticks: Option<JsonValue>,
    pub exchange_name: Option<String>,
    pub exchange_type: Option<String>,
    pub factory_address: Option<String>,
    pub active: Option<bool>,
    pub tag: Option<String>,
}
