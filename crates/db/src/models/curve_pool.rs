use crate::schema::curve_pools;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = curve_pools)]
pub struct DbCurvePool {
    pub id: i32,
    pub address: String,
    pub chain: String,
    pub token_a: String,
    pub token_a_decimals: i32,
    pub token_a_symbol: String,
    pub token_a_balance: String,
    pub token_b: String,
    pub token_b_decimals: i32,
    pub token_b_symbol: String,
    pub token_b_balance: String,
    pub token_c: Option<String>,
    pub token_c_decimals: Option<i32>,
    pub token_c_symbol: Option<String>,
    pub token_c_balance: Option<String>,
    pub token_d: Option<String>,
    pub token_d_decimals: Option<i32>,
    pub token_d_symbol: Option<String>,
    pub token_d_balance: Option<String>,
    pub exchange_name: Option<String>,
    pub exchange_type: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub tag: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = curve_pools)]
pub struct NewDbCurvePool {
    pub address: String,
    pub chain: String,
    pub token_a: String,
    pub token_a_decimals: i32,
    pub token_a_symbol: String,
    pub token_a_balance: String,
    pub token_b: String,
    pub token_b_decimals: i32,
    pub token_b_symbol: String,
    pub token_b_balance: String,
    pub token_c: Option<String>,
    pub token_c_decimals: Option<i32>,
    pub token_c_symbol: Option<String>,
    pub token_c_balance: Option<String>,
    pub token_d: Option<String>,
    pub token_d_decimals: Option<i32>,
    pub token_d_symbol: Option<String>,
    pub token_d_balance: Option<String>,
    pub exchange_name: Option<String>,
    pub exchange_type: Option<String>,
    pub tag: Option<String>,
}
