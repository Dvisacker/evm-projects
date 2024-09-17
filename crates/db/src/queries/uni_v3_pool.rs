use crate::models::{NewUniV3Pool, UniV3Pool};
use crate::schema::uni_v3_pools;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

pub fn insert_uni_v3_pool(
    conn: &mut PgConnection,
    new_pool: &NewUniV3Pool,
) -> Result<UniV3Pool, Error> {
    diesel::insert_into(uni_v3_pools::table)
        .values(new_pool)
        .get_result(conn)
}

pub fn batch_insert_uni_v3_pools(
    conn: &mut PgConnection,
    new_pools: &Vec<NewUniV3Pool>,
) -> Result<Vec<UniV3Pool>, Error> {
    diesel::insert_into(uni_v3_pools::table)
        .values(new_pools)
        .get_results(conn)
}

pub fn get_uni_v3_pool_by_address(
    conn: &mut PgConnection,
    pool_address: &str,
) -> Result<UniV3Pool, Error> {
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
) -> Result<Vec<UniV3Pool>, Error> {
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

    query.load::<UniV3Pool>(conn)
}

pub fn update_uni_v3_pool(
    conn: &mut PgConnection,
    pool_address: &str,
    updated_pool: &NewUniV3Pool,
) -> Result<UniV3Pool, Error> {
    diesel::update(uni_v3_pools::table.filter(uni_v3_pools::address.eq(pool_address)))
        .set(updated_pool)
        .get_result(conn)
}

pub fn delete_uni_v3_pool(conn: &mut PgConnection, pool_address: &str) -> Result<usize, Error> {
    diesel::delete(uni_v3_pools::table.filter(uni_v3_pools::address.eq(pool_address))).execute(conn)
}
