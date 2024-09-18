use crate::schema::pools;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = pools)]
pub struct DbUniV2Pool {
    pub id: i32,
    pub address: String,
    pub chain: String,
    pub factory_address: String,
    pub exchange_name: String,
    pub exchange_type: String,
    pub token_a: String,
    pub token_a_symbol: String,
    pub token_a_decimals: i32,
    pub token_b: String,
    pub token_b_symbol: String,
    pub token_b_decimals: i32,
    pub reserve_0: String,
    pub reserve_1: String,
    pub fee: i32,
    pub filtered: Option<bool>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = pools)]
pub struct NewDbUniV2Pool {
    pub address: String,
    pub chain: String,
    pub factory_address: String,
    pub exchange_name: String,
    pub exchange_type: String,
    pub token_a: String,
    pub token_a_symbol: String,
    pub token_a_decimals: i32,
    pub token_b: String,
    pub token_b_symbol: String,
    pub token_b_decimals: i32,
    pub reserve_0: String,
    pub reserve_1: String,
    pub fee: i32,
    pub filtered: Option<bool>,
}
