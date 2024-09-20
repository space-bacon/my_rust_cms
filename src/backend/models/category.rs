use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::backend::schema::categories;

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "categories"]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub slug: Option<String>,
}
