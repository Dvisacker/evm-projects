use crate::schema::uni_v2_pools;
use diesel::prelude::*;

use super::{db_pool::DbPool, NewDbPool};

#[derive(Queryable, Selectable, Debug, Clone, Default)]
#[diesel(table_name = uni_v2_pools)]
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
    pub factory: Option<String>,
}

impl From<DbUniV2Pool> for DbPool {
    fn from(pool: DbUniV2Pool) -> Self {
        DbPool::UniV2(pool)
    }
}

#[derive(Insertable, Debug, Default)]
#[diesel(table_name = uni_v2_pools)]
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
    pub factory: Option<String>,
}

impl From<NewDbUniV2Pool> for NewDbPool {
    fn from(pool: NewDbUniV2Pool) -> Self {
        NewDbPool::UniV2(pool)
    }
}
