use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::schema::settings;

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct Setting {
    pub id: i32,
    pub setting_key: String,
    pub setting_value: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub setting_type: String, // "site", "system", "backup"
    pub description: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = settings)]
pub struct NewSetting {
    pub setting_key: String,
    pub setting_value: Option<String>,
    pub setting_type: String,
    pub description: Option<String>,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[diesel(table_name = settings)]
pub struct UpdateSetting {
    pub setting_value: Option<String>,
    pub description: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Setting {
    pub fn list(conn: &mut PgConnection) -> QueryResult<Vec<Setting>> {
        settings::table
            .order(settings::setting_key.asc())
            .load::<Setting>(conn)
    }

    pub fn list_by_type(conn: &mut PgConnection, setting_type: &str) -> QueryResult<Vec<Setting>> {
        settings::table
            .filter(settings::setting_type.eq(setting_type))
            .order(settings::setting_key.asc())
            .load::<Setting>(conn)
    }

    pub fn find_by_key(conn: &mut PgConnection, key: &str) -> QueryResult<Option<Setting>> {
        settings::table
            .filter(settings::setting_key.eq(key))
            .first::<Setting>(conn)
            .optional()
    }

    pub fn create(conn: &mut PgConnection, new_setting: NewSetting) -> QueryResult<Setting> {
        diesel::insert_into(settings::table)
            .values(&new_setting)
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, key: &str, update_setting: UpdateSetting) -> QueryResult<Setting> {
        diesel::update(settings::table.filter(settings::setting_key.eq(key)))
            .set(&update_setting)
            .get_result(conn)
    }

    #[allow(dead_code)]
    pub fn delete(conn: &mut PgConnection, key: &str) -> QueryResult<usize> {
        diesel::delete(settings::table.filter(settings::setting_key.eq(key)))
            .execute(conn)
    }

    pub fn upsert(conn: &mut PgConnection, key: &str, value: &str, setting_type: &str, description: Option<String>) -> QueryResult<Setting> {
        match Self::find_by_key(conn, key)? {
            Some(_) => {
                let update_setting = UpdateSetting {
                    setting_value: Some(value.to_string()),
                    description,
                    updated_at: Some(chrono::Utc::now().naive_utc()),
                };
                Self::update(conn, key, update_setting)
            }
            None => {
                let new_setting = NewSetting {
                    setting_key: key.to_string(),
                    setting_value: Some(value.to_string()),
                    setting_type: setting_type.to_string(),
                    description,
                };
                Self::create(conn, new_setting)
            }
        }
    }
}

// Helper structs for API responses
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemInfo {
    pub rust_version: String,
    pub database_version: String,
    pub uptime: String,
    pub memory_usage: String,
    pub cpu_usage: String,
    pub disk_usage: String,
    pub active_sessions: i32,
    pub total_posts: i64,
    pub total_users: i64,
    pub total_media: i64,
    pub last_backup: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupInfo {
    pub id: String,
    pub filename: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub backup_type: String, // "database", "media", "full"
    pub checksum: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataSnapshot {
    pub timestamp: DateTime<Utc>,
    pub tables: Vec<TableSnapshot>,
    pub total_rows: i64,
    pub data_hash: String, // Merkle root hash
    pub integrity_verified: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableSnapshot {
    pub table_name: String,
    pub row_count: i64,
    pub table_hash: String,
    pub last_modified: Option<DateTime<Utc>>,
}