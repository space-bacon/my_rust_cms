use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "comments"]
pub struct Comment {
    pub id: i32,
    pub post_id: i32,
    pub author_name: String,
    pub author_email: String,
    pub content: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
}
