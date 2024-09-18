use crate::models::{DbCurvePool, NewDbCurvePool};
use crate::schema::curve_pools;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

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
