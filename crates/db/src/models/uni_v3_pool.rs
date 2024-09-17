use crate::schema::uni_v3_pools;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Queryable, Selectable, Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = uni_v3_pools)]
pub struct UniV3Pool {
    pub id: Option<i32>,
    pub address: String,
    pub chain: String,
    pub token_a: String,
    pub token_a_decimals: i32,
    pub token_a_symbol: String,
    pub token_b: String,
    pub token_b_decimals: i32,
    pub token_b_symbol: String,
    pub liquidity: String,
    pub sqrt_price: String,
    pub fee: i32,
    pub tick: i32,
    pub tick_spacing: i32,
    pub tick_bitmap: Value,
    pub ticks: Value,
    pub exchange_name: String,
    pub exchange_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = uni_v3_pools)]
pub struct NewUniV3Pool {
    pub address: String,
    pub chain: String,
    pub token_a: String,
    pub token_a_decimals: i32,
    pub token_a_symbol: String,
    pub token_b: String,
    pub token_b_decimals: i32,
    pub token_b_symbol: String,
    pub liquidity: String,
    pub sqrt_price: String,
    pub fee: i32,
    pub tick: i32,
    pub tick_spacing: i32,
    pub tick_bitmap: Value,
    pub ticks: Value,
    pub exchange_name: String,
    pub exchange_type: String,
}
