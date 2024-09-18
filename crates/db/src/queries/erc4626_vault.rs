use crate::models::{DbErc4626Vault, NewDbErc4626Vault};
use crate::schema::erc4626_vaults;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

pub fn insert_erc4626_vault(
    conn: &mut PgConnection,
    new_vault: &NewDbErc4626Vault,
) -> Result<DbErc4626Vault, Error> {
    diesel::insert_into(erc4626_vaults::table)
        .values(new_vault)
        .get_result(conn)
}

pub fn batch_insert_erc4626_vaults(
    conn: &mut PgConnection,
    new_vaults: &Vec<NewDbErc4626Vault>,
) -> Result<Vec<DbErc4626Vault>, Error> {
    diesel::insert_into(erc4626_vaults::table)
        .values(new_vaults)
        .get_results(conn)
}

pub fn get_erc4626_vault_by_address(
    conn: &mut PgConnection,
    vault_address: &str,
) -> Result<DbErc4626Vault, Error> {
    erc4626_vaults::table
        .filter(erc4626_vaults::address.eq(vault_address))
        .first(conn)
}

pub fn get_erc4626_vaults(
    conn: &mut PgConnection,
    chain_name: Option<&str>,
    exchange_name: Option<&str>,
    exchange_type: Option<&str>,
    limit: Option<i64>,
) -> Result<Vec<DbErc4626Vault>, Error> {
    let mut query = erc4626_vaults::table.into_boxed();

    if let Some(chain_name) = chain_name {
        query = query.filter(erc4626_vaults::chain.eq(chain_name));
    }

    if let Some(exchange_name) = exchange_name {
        query = query.filter(erc4626_vaults::exchange_name.eq(exchange_name));
    }

    if let Some(exchange_type) = exchange_type {
        query = query.filter(erc4626_vaults::exchange_type.eq(exchange_type));
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    query.load::<DbErc4626Vault>(conn)
}

pub fn delete_erc4626_vault(conn: &mut PgConnection, vault_address: &str) -> Result<usize, Error> {
    diesel::delete(erc4626_vaults::table.filter(erc4626_vaults::address.eq(vault_address)))
        .execute(conn)
}
