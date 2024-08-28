use crate::models::{NewPool, Pool};
use crate::schema::pools;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sqlite::SqliteConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub mod models;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("migrations/2024-08-28-045356_create_pools");

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn run_migrations(
    conn: &mut SqliteConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

pub fn create_pool(conn: &mut SqliteConnection, new_pool: &NewPool) -> Result<Pool, Error> {
    diesel::insert_into(pools::table)
        .values(new_pool)
        .execute(conn)?;

    pools::table.order(pools::id.desc()).first(conn)
}

pub fn batch_insert_pools(
    conn: &mut SqliteConnection,
    new_pools: &Vec<NewPool>,
) -> Result<Vec<Pool>, Error> {
    conn.transaction(|conn| {
        let inserted_pools: Vec<Pool> = new_pools
            .iter()
            .map(|new_pool| {
                diesel::insert_into(pools::table)
                    .values(new_pool)
                    .execute(conn)?;

                pools::table.order(pools::id.desc()).first(conn)
            })
            .collect::<Result<Vec<Pool>, Error>>()?;

        Ok(inserted_pools)
    })
}

pub fn get_all_pools(conn: &mut SqliteConnection) -> Result<Vec<Pool>, Error> {
    pools::table.load::<Pool>(conn)
}

pub fn get_pool_by_address(conn: &mut SqliteConnection, pool_address: &str) -> Result<Pool, Error> {
    pools::table
        .filter(pools::address.eq(pool_address))
        .first(conn)
}

pub fn update_pool(
    conn: &mut SqliteConnection,
    pool_address: &str,
    updated_pool: &NewPool,
) -> Result<Pool, Error> {
    diesel::update(pools::table.filter(pools::address.eq(pool_address)))
        .set((
            pools::chain.eq(updated_pool.chain.clone()),
            pools::factory_address.eq(updated_pool.factory_address.clone()),
            pools::exchange_name.eq(updated_pool.exchange_name.clone()),
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

pub fn delete_pool(conn: &mut SqliteConnection, pool_address: &str) -> Result<usize, Error> {
    diesel::delete(pools::table.filter(pools::address.eq(pool_address))).execute(conn)
}

// Add a new function to get pools by chain
pub fn get_pools_by_chain(
    conn: &mut SqliteConnection,
    chain_name: &str,
) -> Result<Vec<Pool>, Error> {
    pools::table
        .filter(pools::chain.eq(chain_name))
        .load::<Pool>(conn)
}

// Add a new function to get pools by exchange
pub fn get_pools_by_exchange(
    conn: &mut SqliteConnection,
    exchange_name: &str,
) -> Result<Vec<Pool>, Error> {
    pools::table
        .filter(pools::exchange_name.eq(exchange_name))
        .load::<Pool>(conn)
}
