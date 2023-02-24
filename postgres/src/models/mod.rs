use crate::schema::providers;
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, PartialEq, Eq)]
pub struct Provider {
    id: i32,
    first_name: String,
    last_name: String,
    npi: String,
}

#[derive(Insertable)]
#[diesel(table_name = providers)]
pub struct NewProvider {
    pub first_name: String,
    pub last_name: String,
    pub npi: String,
}
