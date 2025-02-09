use crate::schema::erc4626_vaults;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = erc4626_vaults)]
pub struct DbErc4626Vault {
    pub id: i32,
    pub address: String,
    pub chain: String,
    pub vault_token: String,
    pub vault_token_decimals: i32,
    pub vault_token_symbol: String,
    pub asset_token: String,
    pub asset_token_decimals: i32,
    pub asset_token_symbol: String,
    pub vault_reserve: String,
    pub asset_reserve: String,
    pub deposit_fee: i32,
    pub withdraw_fee: i32,
    pub exchange_name: Option<String>,
    pub exchange_type: Option<String>,
    pub active: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub tag: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = erc4626_vaults)]
pub struct NewDbErc4626Vault {
    pub address: String,
    pub chain: String,
    pub vault_token: String,
    pub vault_token_decimals: i32,
    pub vault_token_symbol: String,
    pub asset_token: String,
    pub asset_token_decimals: i32,
    pub asset_token_symbol: String,
    pub vault_reserve: String,
    pub asset_reserve: String,
    pub deposit_fee: i32,
    pub withdraw_fee: i32,
    pub active: Option<bool>,
    pub exchange_name: Option<String>,
    pub exchange_type: Option<String>,
    pub tag: Option<String>,
}
