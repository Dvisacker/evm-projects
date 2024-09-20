use crate::models::{DbUniV2Pool, NewDbUniV2Pool};
use crate::schema::uni_v2_pools;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::upsert::excluded;

pub fn insert_uni_v2_pool(
    conn: &mut PgConnection,
    new_pool: &NewDbUniV2Pool,
) -> Result<DbUniV2Pool, Error> {
    diesel::insert_into(uni_v2_pools::table)
        .values(new_pool)
        .get_result(conn)
}

pub fn batch_insert_uni_v2_pools(
    conn: &mut PgConnection,
    new_pools: &Vec<NewDbUniV2Pool>,
) -> Result<Vec<DbUniV2Pool>, Error> {
    diesel::insert_into(uni_v2_pools::table)
        .values(new_pools)
        .get_results(conn)
}

pub fn batch_upsert_uni_v2_pools(
    conn: &mut PgConnection,
    new_pools: &Vec<NewDbUniV2Pool>,
) -> Result<Vec<DbUniV2Pool>, Error> {
    diesel::insert_into(uni_v2_pools::table)
        .values(new_pools)
        .on_conflict((uni_v2_pools::chain, uni_v2_pools::address))
        .do_update()
        .set((
            uni_v2_pools::factory_address.eq(excluded(uni_v2_pools::factory_address)),
            uni_v2_pools::exchange_name.eq(excluded(uni_v2_pools::exchange_name)),
            uni_v2_pools::exchange_type.eq(excluded(uni_v2_pools::exchange_type)),
            uni_v2_pools::token_a.eq(excluded(uni_v2_pools::token_a)),
            uni_v2_pools::token_a_symbol.eq(excluded(uni_v2_pools::token_a_symbol)),
            uni_v2_pools::token_a_decimals.eq(excluded(uni_v2_pools::token_a_decimals)),
            uni_v2_pools::token_b.eq(excluded(uni_v2_pools::token_b)),
            uni_v2_pools::token_b_symbol.eq(excluded(uni_v2_pools::token_b_symbol)),
            uni_v2_pools::token_b_decimals.eq(excluded(uni_v2_pools::token_b_decimals)),
            uni_v2_pools::reserve_0.eq(excluded(uni_v2_pools::reserve_0)),
            uni_v2_pools::reserve_1.eq(excluded(uni_v2_pools::reserve_1)),
            uni_v2_pools::fee.eq(excluded(uni_v2_pools::fee)),
        ))
        .get_results(conn)
}

pub fn get_all_uni_v2_pools(conn: &mut PgConnection) -> Result<Vec<DbUniV2Pool>, Error> {
    uni_v2_pools::table.load::<DbUniV2Pool>(conn)
}

pub fn get_uni_v2_pool_by_address(
    conn: &mut PgConnection,
    pool_address: &str,
) -> Result<DbUniV2Pool, Error> {
    uni_v2_pools::table
        .filter(uni_v2_pools::address.eq(pool_address))
        .first(conn)
}

pub fn update_uni_v2_pool(
    conn: &mut PgConnection,
    pool_address: &str,
    updated_pool: &NewDbUniV2Pool,
) -> Result<DbUniV2Pool, Error> {
    diesel::update(uni_v2_pools::table.filter(uni_v2_pools::address.eq(pool_address)))
        .set((
            uni_v2_pools::chain.eq(updated_pool.chain.clone()),
            uni_v2_pools::factory_address.eq(updated_pool.factory_address.clone()),
            uni_v2_pools::exchange_name.eq(updated_pool.exchange_name.clone()),
            uni_v2_pools::exchange_type.eq(updated_pool.exchange_type.clone()), // Add this line
            uni_v2_pools::token_a.eq(updated_pool.token_a.clone()),
            uni_v2_pools::token_a_symbol.eq(updated_pool.token_a_symbol.clone()),
            uni_v2_pools::token_a_decimals.eq(updated_pool.token_a_decimals),
            uni_v2_pools::token_b.eq(updated_pool.token_b.clone()),
            uni_v2_pools::token_b_symbol.eq(updated_pool.token_b_symbol.clone()),
            uni_v2_pools::token_b_decimals.eq(updated_pool.token_b_decimals),
            uni_v2_pools::reserve_0.eq(updated_pool.reserve_0.clone()),
            uni_v2_pools::reserve_1.eq(updated_pool.reserve_1.clone()),
            uni_v2_pools::fee.eq(updated_pool.fee),
        ))
        .execute(conn)?;

    get_uni_v2_pool_by_address(conn, pool_address)
}

pub fn delete_uni_v2_pool(conn: &mut PgConnection, pool_address: &str) -> Result<usize, Error> {
    diesel::delete(uni_v2_pools::table.filter(uni_v2_pools::address.eq(pool_address))).execute(conn)
}

pub fn get_uni_v2_pools(
    conn: &mut PgConnection,
    chain_name: Option<&str>,
    exchange_name: Option<&str>,
    exchange_type: Option<&str>,
    limit: Option<i64>,
    active: Option<bool>,
) -> Result<Vec<DbUniV2Pool>, Error> {
    let mut query = uni_v2_pools::table.into_boxed();

    if let Some(chain_name) = chain_name {
        query = query.filter(uni_v2_pools::chain.eq(chain_name));
    }

    if let Some(exchange_name) = exchange_name {
        query = query.filter(uni_v2_pools::exchange_name.eq(exchange_name));
    }

    if let Some(exchange_type) = exchange_type {
        query = query.filter(uni_v2_pools::exchange_type.eq(exchange_type));
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    if let Some(active) = active {
        if active {
            query = query.filter(uni_v2_pools::active.eq(true));
        } else {
            query = query.filter(uni_v2_pools::active.eq(false));
        }
    } else {
        query = query.filter(uni_v2_pools::active.is_null());
    }

    query.load::<DbUniV2Pool>(conn)
}

// Add a new function to get pools by chain
pub fn get_uni_v2_pools_by_chain(
    conn: &mut PgConnection,
    chain_name: &str,
) -> Result<Vec<DbUniV2Pool>, Error> {
    uni_v2_pools::table
        .filter(uni_v2_pools::chain.eq(chain_name))
        .load::<DbUniV2Pool>(conn)
}

// Add a new function to get pools by exchange
pub fn get_uni_v2_pools_by_exchange(
    conn: &mut PgConnection,
    exchange_name: &str,
) -> Result<Vec<DbUniV2Pool>, Error> {
    uni_v2_pools::table
        .filter(uni_v2_pools::exchange_name.eq(exchange_name))
        .load::<DbUniV2Pool>(conn)
}

pub fn batch_update_uni_v2_pool_active(
    conn: &mut PgConnection,
    pool_addresses: &[String],
    active: bool,
) -> QueryResult<usize> {
    diesel::update(uni_v2_pools::table)
        .filter(uni_v2_pools::address.eq_any(pool_addresses))
        .set(uni_v2_pools::active.eq(active))
        .execute(conn)
}

pub fn get_uni_v2_active_pools(
    conn: &mut PgConnection,
    chain_name: &str,
) -> Result<Vec<DbUniV2Pool>, Error> {
    uni_v2_pools::table
        .filter(uni_v2_pools::chain.eq(chain_name))
        .filter(uni_v2_pools::active.eq(true))
        .load::<DbUniV2Pool>(conn)
}
