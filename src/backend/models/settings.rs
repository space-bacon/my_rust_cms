use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "settings"]
pub struct Settings {
    pub id: i32,
    pub name: String,
    pub value: String,
}
