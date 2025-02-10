use crate::models::tag::{DbTag, NewDbTag};
use crate::schema::tags;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;

pub fn insert_tag(conn: &mut PgConnection, new_tag: &NewDbTag) -> Result<DbTag, Error> {
    diesel::insert_into(tags::table)
        .values(new_tag)
        .get_result(conn)
}

pub fn upsert_tag(conn: &mut PgConnection, new_tag: &NewDbTag) -> Result<DbTag, Error> {
    diesel::insert_into(tags::table)
        .values(new_tag)
        .on_conflict(tags::name)
        .do_nothing()
        .get_result(conn)
}

pub fn batch_insert_tags(
    conn: &mut PgConnection,
    new_tags: &[NewDbTag],
) -> Result<Vec<DbTag>, Error> {
    diesel::insert_into(tags::table)
        .values(new_tags)
        .get_results(conn)
}

pub fn get_all_tags(conn: &mut PgConnection) -> Result<Vec<DbTag>, Error> {
    tags::table.load::<DbTag>(conn)
}

pub fn get_tag_by_name(conn: &mut PgConnection, tag_name: &str) -> Result<DbTag, Error> {
    tags::table.filter(tags::name.eq(tag_name)).first(conn)
}

pub fn delete_tag(conn: &mut PgConnection, tag_name: &str) -> Result<usize, Error> {
    diesel::delete(tags::table.filter(tags::name.eq(tag_name))).execute(conn)
}
