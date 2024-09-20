use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::Queryable;
use chrono::NaiveDateTime;

use crate::backend::schema::users;

#[derive(Serialize, Queryable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}
