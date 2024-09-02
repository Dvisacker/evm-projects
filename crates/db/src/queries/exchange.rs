use crate::models::{Exchange, NewExchange};
use crate::schema::exchanges;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn insert_exchange(
    conn: &mut SqliteConnection,
    new_exchange: &NewExchange,
) -> Result<Exchange, diesel::result::Error> {
    let result = diesel::insert_into(exchanges::table)
        .values(new_exchange)
        .execute(conn);

    if result.is_err() {
        return Err(result.err().unwrap());
    }

    exchanges::table
        .order(exchanges::id.desc())
        .first(conn)
        .map_err(|_| diesel::result::Error::NotFound)
}

pub fn get_exchanges(conn: &mut SqliteConnection) -> QueryResult<Vec<Exchange>> {
    exchanges::table.load::<Exchange>(conn)
}

pub fn get_exchange_by_name(
    conn: &mut SqliteConnection,
    chain: &str,
    name: &str,
) -> QueryResult<Exchange> {
    exchanges::table
        .filter(exchanges::chain.eq(chain))
        .filter(exchanges::exchange_name.eq(name))
        .first(conn)
}

pub fn get_exchanges_by_chain(
    conn: &mut SqliteConnection,
    chain: &str,
) -> QueryResult<Vec<Exchange>> {
    exchanges::table
        .filter(exchanges::chain.eq(chain))
        .load::<Exchange>(conn)
}

pub fn get_all_exchanges(conn: &mut SqliteConnection) -> QueryResult<Vec<Exchange>> {
    exchanges::table.load::<Exchange>(conn)
}
