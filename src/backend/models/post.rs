use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: String,
    pub author_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
