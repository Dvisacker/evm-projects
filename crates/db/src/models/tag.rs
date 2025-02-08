use crate::schema::tags;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = tags)]
pub struct DbTag {
    pub name: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = tags)]
pub struct NewDbTag {
    pub name: String,
}
