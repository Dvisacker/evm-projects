use crate::models::{DbUniV3Pool, NewDbUniV3Pool};
use crate::schema::uni_v3_pools;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::upsert::excluded;

pub fn insert_uni_v3_pool(
    conn: &mut PgConnection,
    new_pool: &NewDbUniV3Pool,
) -> Result<DbUniV3Pool, Error> {
    diesel::insert_into(uni_v3_pools::table)
        .values(new_pool)
        .get_result(conn)
}

pub fn batch_insert_uni_v3_pools(
    conn: &mut PgConnection,
    new_pools: &Vec<NewDbUniV3Pool>,
) -> Result<Vec<DbUniV3Pool>, Error> {
    diesel::insert_into(uni_v3_pools::table)
        .values(new_pools)
        .get_results(conn)
}

pub fn batch_upsert_uni_v3_pools(
    conn: &mut PgConnection,
    new_pools: &[NewDbUniV3Pool],
) -> Result<Vec<DbUniV3Pool>, Error> {
    diesel::insert_into(uni_v3_pools::table)
        .values(new_pools)
        .on_conflict((uni_v3_pools::chain, uni_v3_pools::address))
        .do_update()
        .set((
            uni_v3_pools::chain.eq(excluded(uni_v3_pools::chain)),
            uni_v3_pools::token_a.eq(excluded(uni_v3_pools::token_a)),
            uni_v3_pools::token_a_decimals.eq(excluded(uni_v3_pools::token_a_decimals)),
            uni_v3_pools::token_a_symbol.eq(excluded(uni_v3_pools::token_a_symbol)),
            uni_v3_pools::token_b.eq(excluded(uni_v3_pools::token_b)),
            uni_v3_pools::token_b_decimals.eq(excluded(uni_v3_pools::token_b_decimals)),
            uni_v3_pools::token_b_symbol.eq(excluded(uni_v3_pools::token_b_symbol)),
            uni_v3_pools::liquidity.eq(excluded(uni_v3_pools::liquidity)),
            uni_v3_pools::sqrt_price.eq(excluded(uni_v3_pools::sqrt_price)),
            uni_v3_pools::fee.eq(excluded(uni_v3_pools::fee)),
            uni_v3_pools::tick.eq(excluded(uni_v3_pools::tick)),
            uni_v3_pools::tick_spacing.eq(excluded(uni_v3_pools::tick_spacing)),
            uni_v3_pools::tick_bitmap.eq(excluded(uni_v3_pools::tick_bitmap)),
            uni_v3_pools::ticks.eq(excluded(uni_v3_pools::ticks)),
            uni_v3_pools::exchange_name.eq(excluded(uni_v3_pools::exchange_name)),
            uni_v3_pools::exchange_type.eq(excluded(uni_v3_pools::exchange_type)),
        ))
        .get_results(conn)
}

pub fn get_uni_v3_pool_by_address(
    conn: &mut PgConnection,
    pool_address: &str,
) -> Result<DbUniV3Pool, Error> {
    uni_v3_pools::table
        .filter(uni_v3_pools::address.eq(pool_address))
        .first(conn)
}

pub fn get_uni_v3_pools(
    conn: &mut PgConnection,
    chain_name: Option<&str>,
    exchange_name: Option<&str>,
    exchange_type: Option<&str>,
    limit: Option<i64>,
    filtered: Option<bool>,
) -> Result<Vec<DbUniV3Pool>, Error> {
    let mut query = uni_v3_pools::table.into_boxed();

    if let Some(chain_name) = chain_name {
        query = query.filter(uni_v3_pools::chain.eq(chain_name));
    }

    if let Some(exchange_name) = exchange_name {
        query = query.filter(uni_v3_pools::exchange_name.eq(exchange_name));
    }

    if let Some(exchange_type) = exchange_type {
        query = query.filter(uni_v3_pools::exchange_type.eq(exchange_type));
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    if let Some(filtered) = filtered {
        if filtered {
            query = query.filter(uni_v3_pools::filtered.eq(true));
        } else {
            query = query.filter(uni_v3_pools::filtered.eq(false));
        }
    } else {
        query = query.filter(uni_v3_pools::filtered.is_null());
    }

    query.load::<DbUniV3Pool>(conn)
}

pub fn update_uni_v3_pool(
    conn: &mut PgConnection,
    pool_address: &str,
    updated_pool: &NewDbUniV3Pool,
) -> Result<DbUniV3Pool, Error> {
    diesel::update(uni_v3_pools::table.filter(uni_v3_pools::address.eq(pool_address)))
        .set((
            uni_v3_pools::chain.eq(updated_pool.chain.clone()),
            uni_v3_pools::token_a.eq(updated_pool.token_a.clone()),
            uni_v3_pools::token_a_decimals.eq(updated_pool.token_a_decimals),
            uni_v3_pools::token_a_symbol.eq(updated_pool.token_a_symbol.clone()),
            uni_v3_pools::token_b.eq(updated_pool.token_b.clone()),
            uni_v3_pools::token_b_decimals.eq(updated_pool.token_b_decimals),
            uni_v3_pools::token_b_symbol.eq(updated_pool.token_b_symbol.clone()),
            uni_v3_pools::liquidity.eq(updated_pool.liquidity.clone()),
            uni_v3_pools::sqrt_price.eq(updated_pool.sqrt_price.clone()),
            uni_v3_pools::fee.eq(updated_pool.fee),
            uni_v3_pools::tick.eq(updated_pool.tick),
            uni_v3_pools::tick_spacing.eq(updated_pool.tick_spacing),
            uni_v3_pools::tick_bitmap.eq(updated_pool.tick_bitmap.clone()),
            uni_v3_pools::ticks.eq(updated_pool.ticks.clone()),
            uni_v3_pools::exchange_name.eq(updated_pool.exchange_name.clone()),
            uni_v3_pools::exchange_type.eq(updated_pool.exchange_type.clone()),
        ))
        .get_result(conn)
}

pub fn delete_uni_v3_pool(conn: &mut PgConnection, pool_address: &str) -> Result<usize, Error> {
    diesel::delete(uni_v3_pools::table.filter(uni_v3_pools::address.eq(pool_address))).execute(conn)
}
