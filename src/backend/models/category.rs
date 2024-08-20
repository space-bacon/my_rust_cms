use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
}
