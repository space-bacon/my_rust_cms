use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "pages"]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub content: String,  // JSON or a serialized format for the page structure
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
