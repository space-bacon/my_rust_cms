// src/backend/models/settings.rs

use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::backend::schema::settings;
use chrono::NaiveDateTime;

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[table_name = "settings"]
pub struct Settings {
    pub id: i32,
    pub site_name: String,
    pub site_description: String,
    pub admin_email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "settings"]
pub struct NewSettings {
    pub site_name: String,
    pub site_description: String,
    pub admin_email: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "settings"]
pub struct UpdateSettings {
    pub site_name: Option<String>,
    pub site_description: Option<String>,
    pub admin_email: Option<String>,
}
