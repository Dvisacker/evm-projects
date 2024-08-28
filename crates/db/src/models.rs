use diesel::prelude::*;
use crate::schema::pools;

#[derive(Queryable, Selectable)]
#[diesel(table_name = pools)]
pub struct Pool {
    pub id: i32,
    pub address: String,
    pub chain: String,
    pub factory_address: String,
    pub exchange_name: String,
    pub token_a: String,
    pub token_a_symbol: String,
    pub token_a_decimals: i32,
    pub token_b: String,
    pub token_b_symbol: String,
    pub token_b_decimals: i32,
    pub reserve_0: String,
    pub reserve_1: String,
    pub fee: i32,
}

#[derive(Insertable)]
#[diesel(table_name = pools)]
pub struct NewPool<'a> {
    pub address: &'a str,
    pub chain: &'a str,
    pub factory_address: &'a str,
    pub exchange_name: &'a str,
    pub token_a: &'a str,
    pub token_a_symbol: &'a str,
    pub token_a_decimals: i32,
    pub token_b: &'a str,
    pub token_b_symbol: &'a str,
    pub token_b_decimals: i32,
    pub reserve_0: &'a str,
    pub reserve_1: &'a str,
    pub fee: i32,
}
