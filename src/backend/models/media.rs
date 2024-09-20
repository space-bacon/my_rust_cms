use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use chrono::NaiveDateTime;

use crate::backend::schema::media;

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[table_name = "media"]
pub struct Media {
    pub id: i32,
    pub url: String,
    pub alt_text: String,
    pub uploaded_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "media"]
pub struct NewMedia {
    pub url: String,
    pub alt_text: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "media"]
pub struct UpdateMedia {
    pub url: Option<String>,
    pub alt_text: Option<String>,
}
