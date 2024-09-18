use crate::models::{DbUniV2Pool, NewDbUniV2Pool};
use crate::schema::pools;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::upsert::excluded;

pub fn insert_pool(
    conn: &mut PgConnection,
    new_pool: &NewDbUniV2Pool,
) -> Result<DbUniV2Pool, Error> {
    diesel::insert_into(pools::table)
        .values(new_pool)
        .get_result(conn)
}

pub fn batch_insert_pools(
    conn: &mut PgConnection,
    new_pools: &Vec<NewDbUniV2Pool>,
) -> Result<Vec<DbUniV2Pool>, Error> {
    diesel::insert_into(pools::table)
        .values(new_pools)
        .get_results(conn)
}

pub fn batch_upsert_pools(
    conn: &mut PgConnection,
    new_pools: &Vec<NewDbUniV2Pool>,
) -> Result<Vec<DbUniV2Pool>, Error> {
    diesel::insert_into(pools::table)
        .values(new_pools)
        .on_conflict((pools::chain, pools::address))
        .do_update()
        .set((
            pools::factory_address.eq(excluded(pools::factory_address)),
            pools::exchange_name.eq(excluded(pools::exchange_name)),
            pools::exchange_type.eq(excluded(pools::exchange_type)),
            pools::token_a.eq(excluded(pools::token_a)),
            pools::token_a_symbol.eq(excluded(pools::token_a_symbol)),
            pools::token_a_decimals.eq(excluded(pools::token_a_decimals)),
            pools::token_b.eq(excluded(pools::token_b)),
            pools::token_b_symbol.eq(excluded(pools::token_b_symbol)),
            pools::token_b_decimals.eq(excluded(pools::token_b_decimals)),
            pools::reserve_0.eq(excluded(pools::reserve_0)),
            pools::reserve_1.eq(excluded(pools::reserve_1)),
            pools::fee.eq(excluded(pools::fee)),
        ))
        .get_results(conn)
}

pub fn get_all_pools(conn: &mut PgConnection) -> Result<Vec<DbUniV2Pool>, Error> {
    pools::table.load::<DbUniV2Pool>(conn)
}

pub fn get_pool_by_address(
    conn: &mut PgConnection,
    pool_address: &str,
) -> Result<DbUniV2Pool, Error> {
    pools::table
        .filter(pools::address.eq(pool_address))
        .first(conn)
}

pub fn update_pool(
    conn: &mut PgConnection,
    pool_address: &str,
    updated_pool: &NewDbUniV2Pool,
) -> Result<DbUniV2Pool, Error> {
    diesel::update(pools::table.filter(pools::address.eq(pool_address)))
        .set((
            pools::chain.eq(updated_pool.chain.clone()),
            pools::factory_address.eq(updated_pool.factory_address.clone()),
            pools::exchange_name.eq(updated_pool.exchange_name.clone()),
            pools::exchange_type.eq(updated_pool.exchange_type.clone()), // Add this line
            pools::token_a.eq(updated_pool.token_a.clone()),
            pools::token_a_symbol.eq(updated_pool.token_a_symbol.clone()),
            pools::token_a_decimals.eq(updated_pool.token_a_decimals),
            pools::token_b.eq(updated_pool.token_b.clone()),
            pools::token_b_symbol.eq(updated_pool.token_b_symbol.clone()),
            pools::token_b_decimals.eq(updated_pool.token_b_decimals),
            pools::reserve_0.eq(updated_pool.reserve_0.clone()),
            pools::reserve_1.eq(updated_pool.reserve_1.clone()),
            pools::fee.eq(updated_pool.fee),
        ))
        .execute(conn)?;

    get_pool_by_address(conn, pool_address)
}

pub fn delete_pool(conn: &mut PgConnection, pool_address: &str) -> Result<usize, Error> {
    diesel::delete(pools::table.filter(pools::address.eq(pool_address))).execute(conn)
}

pub fn get_pools(
    conn: &mut PgConnection,
    chain_name: Option<&str>,
    exchange_name: Option<&str>,
    exchange_type: Option<&str>,
    limit: Option<i64>,
    filtered: Option<bool>,
) -> Result<Vec<DbUniV2Pool>, Error> {
    let mut query = pools::table.into_boxed();

    if let Some(chain_name) = chain_name {
        query = query.filter(pools::chain.eq(chain_name));
    }

    if let Some(exchange_name) = exchange_name {
        query = query.filter(pools::exchange_name.eq(exchange_name));
    }

    if let Some(exchange_type) = exchange_type {
        query = query.filter(pools::exchange_type.eq(exchange_type));
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    if let Some(filtered) = filtered {
        if filtered {
            query = query.filter(pools::filtered.eq(true));
        } else {
            query = query.filter(pools::filtered.eq(false));
        }
    } else {
        query = query.filter(pools::filtered.is_null());
    }

    // query = query.filter(pools::filtered.eq_any(filtered));

    query.load::<DbUniV2Pool>(conn)
}

// Add a new function to get pools by chain
pub fn get_pools_by_chain(
    conn: &mut PgConnection,
    chain_name: &str,
) -> Result<Vec<DbUniV2Pool>, Error> {
    pools::table
        .filter(pools::chain.eq(chain_name))
        .load::<DbUniV2Pool>(conn)
}

// Add a new function to get pools by exchange
pub fn get_pools_by_exchange(
    conn: &mut PgConnection,
    exchange_name: &str,
) -> Result<Vec<DbUniV2Pool>, Error> {
    pools::table
        .filter(pools::exchange_name.eq(exchange_name))
        .load::<DbUniV2Pool>(conn)
}

pub fn batch_update_filtered(
    conn: &mut PgConnection,
    pool_addresses: &[String],
    filtered: bool,
) -> QueryResult<usize> {
    diesel::update(pools::table)
        .filter(pools::address.eq_any(pool_addresses))
        .set(pools::filtered.eq(filtered))
        .execute(conn)
}

pub fn get_filtered_pools(
    conn: &mut PgConnection,
    chain_name: &str,
) -> Result<Vec<DbUniV2Pool>, Error> {
    pools::table
        .filter(pools::chain.eq(chain_name))
        .filter(pools::filtered.eq(true))
        .load::<DbUniV2Pool>(conn)
}
