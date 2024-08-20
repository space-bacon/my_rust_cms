use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "tags"]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
}
