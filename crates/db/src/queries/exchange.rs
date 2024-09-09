use crate::models::{Exchange, NewExchange};
use crate::schema::exchanges;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn insert_exchange(
    conn: &mut PgConnection,
    new_exchange: &NewExchange,
) -> Result<Exchange, diesel::result::Error> {
    diesel::insert_into(exchanges::table)
        .values(new_exchange)
        .get_result(conn)
}

pub fn get_exchanges(conn: &mut PgConnection) -> QueryResult<Vec<Exchange>> {
    exchanges::table.load::<Exchange>(conn)
}

pub fn get_exchange_by_name(
    conn: &mut PgConnection,
    chain: &str,
    name: &str,
) -> QueryResult<Exchange> {
    exchanges::table
        .filter(exchanges::chain.eq(chain))
        .filter(exchanges::exchange_name.eq(name))
        .first(conn)
}

pub fn get_exchanges_by_chain(conn: &mut PgConnection, chain: &str) -> QueryResult<Vec<Exchange>> {
    exchanges::table
        .filter(exchanges::chain.eq(chain))
        .load::<Exchange>(conn)
}

pub fn get_all_exchanges(conn: &mut PgConnection) -> QueryResult<Vec<Exchange>> {
    exchanges::table.load::<Exchange>(conn)
}
