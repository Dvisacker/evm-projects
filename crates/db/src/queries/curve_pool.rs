use crate::models::{DbCurvePool, NewDbCurvePool};
use crate::schema::curve_pools;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::upsert::excluded;

pub fn insert_curve_pool(
    conn: &mut PgConnection,
    new_pool: &NewDbCurvePool,
) -> Result<DbCurvePool, Error> {
    diesel::insert_into(curve_pools::table)
        .values(new_pool)
        .get_result(conn)
}

pub fn batch_insert_curve_pools(
    conn: &mut PgConnection,
    new_pools: &Vec<NewDbCurvePool>,
) -> Result<Vec<DbCurvePool>, Error> {
    diesel::insert_into(curve_pools::table)
        .values(new_pools)
        .get_results(conn)
}

pub fn get_curve_pool_by_address(
    conn: &mut PgConnection,
    pool_address: &str,
) -> Result<DbCurvePool, Error> {
    curve_pools::table
        .filter(curve_pools::address.eq(pool_address))
        .first(conn)
}

pub fn get_curve_pools(
    conn: &mut PgConnection,
    chain_name: Option<&str>,
    exchange_name: Option<&str>,
    exchange_type: Option<&str>,
    limit: Option<i64>,
) -> Result<Vec<DbCurvePool>, Error> {
    let mut query = curve_pools::table.into_boxed();

    if let Some(chain_name) = chain_name {
        query = query.filter(curve_pools::chain.eq(chain_name));
    }

    if let Some(exchange_name) = exchange_name {
        query = query.filter(curve_pools::exchange_name.eq(exchange_name));
    }

    if let Some(exchange_type) = exchange_type {
        query = query.filter(curve_pools::exchange_type.eq(exchange_type));
    }

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    query.load::<DbCurvePool>(conn)
}

pub fn delete_curve_pool(conn: &mut PgConnection, pool_address: &str) -> Result<usize, Error> {
    diesel::delete(curve_pools::table.filter(curve_pools::address.eq(pool_address))).execute(conn)
}

pub fn batch_upsert_curve_pools(
    conn: &mut PgConnection,
    new_pools: &[NewDbCurvePool],
) -> Result<Vec<DbCurvePool>, Error> {
    diesel::insert_into(curve_pools::table)
        .values(new_pools)
        .on_conflict((curve_pools::chain, curve_pools::address))
        .do_update()
        .set((
            curve_pools::token_a.eq(excluded(curve_pools::token_a)),
            curve_pools::token_a_decimals.eq(excluded(curve_pools::token_a_decimals)),
            curve_pools::token_a_symbol.eq(excluded(curve_pools::token_a_symbol)),
            curve_pools::token_a_balance.eq(excluded(curve_pools::token_a_balance)),
            curve_pools::token_b.eq(excluded(curve_pools::token_b)),
            curve_pools::token_b_decimals.eq(excluded(curve_pools::token_b_decimals)),
            curve_pools::token_b_symbol.eq(excluded(curve_pools::token_b_symbol)),
            curve_pools::token_b_balance.eq(excluded(curve_pools::token_b_balance)),
            curve_pools::token_c.eq(excluded(curve_pools::token_c)),
            curve_pools::token_c_decimals.eq(excluded(curve_pools::token_c_decimals)),
            curve_pools::token_c_symbol.eq(excluded(curve_pools::token_c_symbol)),
            curve_pools::token_c_balance.eq(excluded(curve_pools::token_c_balance)),
            curve_pools::token_d.eq(excluded(curve_pools::token_d)),
            curve_pools::token_d_decimals.eq(excluded(curve_pools::token_d_decimals)),
            curve_pools::token_d_symbol.eq(excluded(curve_pools::token_d_symbol)),
            curve_pools::token_d_balance.eq(excluded(curve_pools::token_d_balance)),
            curve_pools::exchange_name.eq(excluded(curve_pools::exchange_name)),
            curve_pools::exchange_type.eq(excluded(curve_pools::exchange_type)),
            curve_pools::active.eq(excluded(curve_pools::active)),
            curve_pools::tag.eq(excluded(curve_pools::tag)),
        ))
        .get_results(conn)
}

pub fn update_curve_pool(
    conn: &mut PgConnection,
    pool_address: &str,
    updated_pool: &NewDbCurvePool,
) -> Result<DbCurvePool, Error> {
    diesel::update(curve_pools::table.filter(curve_pools::address.eq(pool_address)))
        .set((
            curve_pools::chain.eq(updated_pool.chain.clone()),
            curve_pools::token_a.eq(updated_pool.token_a.clone()),
            curve_pools::token_a_decimals.eq(updated_pool.token_a_decimals),
            curve_pools::token_a_symbol.eq(updated_pool.token_a_symbol.clone()),
            curve_pools::token_a_balance.eq(updated_pool.token_a_balance.clone()),
            curve_pools::token_b.eq(updated_pool.token_b.clone()),
            curve_pools::token_b_decimals.eq(updated_pool.token_b_decimals),
            curve_pools::token_b_symbol.eq(updated_pool.token_b_symbol.clone()),
            curve_pools::token_b_balance.eq(updated_pool.token_b_balance.clone()),
            curve_pools::token_c.eq(updated_pool.token_c.clone()),
            curve_pools::token_c_decimals.eq(updated_pool.token_c_decimals),
            curve_pools::token_c_symbol.eq(updated_pool.token_c_symbol.clone()),
            curve_pools::token_c_balance.eq(updated_pool.token_c_balance.clone()),
            curve_pools::token_d.eq(updated_pool.token_d.clone()),
            curve_pools::token_d_decimals.eq(updated_pool.token_d_decimals),
            curve_pools::token_d_symbol.eq(updated_pool.token_d_symbol.clone()),
            curve_pools::token_d_balance.eq(updated_pool.token_d_balance.clone()),
            curve_pools::exchange_name.eq(updated_pool.exchange_name.clone()),
            curve_pools::exchange_type.eq(updated_pool.exchange_type.clone()),
            curve_pools::active.eq(updated_pool.active),
            curve_pools::tag.eq(updated_pool.tag.clone()),
        ))
        .get_result(conn)
}
