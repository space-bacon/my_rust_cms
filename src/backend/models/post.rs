use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{Queryable, Insertable, Identifiable, Associations, AsChangeset};
use chrono::NaiveDateTime;

use crate::backend::schema::posts;
use crate::backend::models::user::User;

#[derive(Serialize, Queryable, Identifiable, Associations, Debug)]
#[table_name = "posts"]
#[belongs_to(User, foreign_key = "author_id")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: String,
    pub author_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: String,
    pub author_id: i32,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "posts"]
pub struct UpdatePost {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub content: Option<String>,
    pub status: Option<String>,
}
