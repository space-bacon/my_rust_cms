use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "media"]
pub struct Media {
    pub id: i32,
    pub url: String,
    pub alt_text: String,
    pub uploaded_at: chrono::NaiveDateTime,
}
