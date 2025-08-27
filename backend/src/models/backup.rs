use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::schema::{backups, backup_schedules, backup_logs};

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = backups)]
pub struct Backup {
    pub id: Uuid,
    pub filename: String,
    pub backup_type: String,
    pub status: String,
    pub file_size: Option<i64>,
    pub checksum: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub metadata: serde_json::Value,
    pub error_message: Option<String>,
    pub retention_policy: Option<String>,
    pub is_encrypted: Option<bool>,
    pub compression_type: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = backups)]
pub struct NewBackup {
    pub filename: String,
    pub backup_type: String,
    pub status: Option<String>,
    pub file_size: Option<i64>,
    pub checksum: Option<String>,
    pub description: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
    pub retention_policy: Option<String>,
    pub is_encrypted: Option<bool>,
    pub compression_type: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = backups)]
pub struct UpdateBackup {
    pub status: Option<String>,
    pub file_size: Option<i64>,
    pub checksum: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = backup_schedules)]
pub struct BackupSchedule {
    pub id: Uuid,
    pub name: String,
    pub backup_type: String,
    pub cron_expression: String,
    pub is_active: Option<bool>,
    pub retention_days: Option<i32>,
    pub max_backups: Option<i32>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub next_run_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub settings: serde_json::Value,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = backup_schedules)]
pub struct NewBackupSchedule {
    pub name: String,
    pub backup_type: String,
    pub cron_expression: String,
    pub is_active: Option<bool>,
    pub retention_days: Option<i32>,
    pub max_backups: Option<i32>,
    pub description: Option<String>,
    pub next_run_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub settings: Option<serde_json::Value>,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = backup_logs)]
pub struct BackupLog {
    pub id: Uuid,
    pub backup_id: Option<Uuid>,
    pub schedule_id: Option<Uuid>,
    pub level: String,
    pub message: String,
    pub details: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = backup_logs)]
pub struct NewBackupLog {
    pub backup_id: Option<Uuid>,
    pub schedule_id: Option<Uuid>,
    pub level: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

// Legacy BackupInfo for compatibility
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BackupInfo {
    pub id: String,
    pub filename: String,
    pub size: u64,
    pub created_at: DateTime<Utc>,
    pub backup_type: String,
    pub checksum: String,
    pub description: Option<String>,
}

impl From<Backup> for BackupInfo {
    fn from(backup: Backup) -> Self {
        Self {
            id: backup.id.to_string(),
            filename: backup.filename,
            size: backup.file_size.unwrap_or(0) as u64,
            created_at: backup.created_at,
            backup_type: backup.backup_type,
            checksum: backup.checksum.unwrap_or_default(),
            description: backup.description,
        }
    }
}

// Data snapshot structures
#[derive(Serialize, Deserialize, Debug)]
pub struct DataSnapshot {
    pub timestamp: DateTime<Utc>,
    pub tables: Vec<TableSnapshot>,
    pub total_rows: i64,
    pub data_hash: String,
    pub integrity_verified: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableSnapshot {
    pub table_name: String,
    pub row_count: i64,
    pub table_hash: String,
    pub last_modified: Option<DateTime<Utc>>,
}

impl Backup {
    /// Create a new backup record
    pub fn create(conn: &mut PgConnection, new_backup: NewBackup) -> QueryResult<Backup> {
        diesel::insert_into(backups::table)
            .values(&new_backup)
            .get_result(conn)
    }

    /// Find backup by ID
    pub fn find_by_id(conn: &mut PgConnection, backup_id: Uuid) -> QueryResult<Backup> {
        backups::table.find(backup_id).first(conn)
    }

    /// List all backups with optional filtering
    pub fn list(
        conn: &mut PgConnection,
        backup_type_filter: Option<String>,
        status_filter: Option<String>,
        limit: Option<i64>,
    ) -> QueryResult<Vec<Backup>> {
        let mut query = backups::table.into_boxed();

        if let Some(backup_type) = backup_type_filter {
            query = query.filter(backups::backup_type.eq(backup_type));
        }

        if let Some(status) = status_filter {
            query = query.filter(backups::status.eq(status));
        }

        query = query.order(backups::created_at.desc());

        if let Some(limit_val) = limit {
            query = query.limit(limit_val);
        }

        query.load(conn)
    }

    /// Update backup status and metadata
    pub fn update_status(
        conn: &mut PgConnection,
        backup_id: Uuid,
        update: UpdateBackup,
    ) -> QueryResult<Backup> {
        diesel::update(backups::table.find(backup_id))
            .set(&update)
            .get_result(conn)
    }

    /// Delete old backups based on retention policy
    pub fn cleanup_expired(conn: &mut PgConnection) -> QueryResult<usize> {
        use crate::schema::backups::dsl::*;
        
        diesel::delete(
            backups.filter(
                expires_at.is_not_null()
                    .and(expires_at.lt(Utc::now()))
            )
        ).execute(conn)
    }

    /// Get backup statistics
    pub fn get_statistics(conn: &mut PgConnection) -> QueryResult<BackupStatistics> {
        use crate::schema::backups::dsl::*;
        
        let total_backups: i64 = backups.count().get_result(conn)?;
        let completed_backups: i64 = backups
            .filter(status.eq("completed"))
            .count()
            .get_result(conn)?;
        let failed_backups: i64 = backups
            .filter(status.eq("failed"))
            .count()
            .get_result(conn)?;
        let total_size: Option<i64> = backups
            .filter(status.eq("completed"))
            .select(diesel::dsl::sum(file_size))
            .first(conn)?;

        Ok(BackupStatistics {
            total_backups,
            completed_backups,
            failed_backups,
            total_size: total_size.unwrap_or(0),
        })
    }
}

impl BackupSchedule {
    /// Create a new backup schedule
    pub fn create(conn: &mut PgConnection, new_schedule: NewBackupSchedule) -> QueryResult<BackupSchedule> {
        diesel::insert_into(backup_schedules::table)
            .values(&new_schedule)
            .get_result(conn)
    }

    /// List all backup schedules
    pub fn list(conn: &mut PgConnection, active_only: bool) -> QueryResult<Vec<BackupSchedule>> {
        let mut query = backup_schedules::table.into_boxed();
        
        if active_only {
            query = query.filter(backup_schedules::is_active.eq(true));
        }
        
        query.order(backup_schedules::created_at.desc()).load(conn)
    }

    /// Update schedule's last run time
    pub fn update_last_run(
        conn: &mut PgConnection,
        schedule_id: Uuid,
        last_run: DateTime<Utc>,
        next_run: Option<DateTime<Utc>>,
    ) -> QueryResult<BackupSchedule> {
        diesel::update(backup_schedules::table.find(schedule_id))
            .set((
                backup_schedules::last_run_at.eq(last_run),
                backup_schedules::next_run_at.eq(next_run),
                backup_schedules::updated_at.eq(Utc::now()),
            ))
            .get_result(conn)
    }
}

impl BackupLog {
    /// Create a new backup log entry
    pub fn create(conn: &mut PgConnection, new_log: NewBackupLog) -> QueryResult<BackupLog> {
        diesel::insert_into(backup_logs::table)
            .values(&new_log)
            .get_result(conn)
    }

    /// Get logs for a specific backup
    pub fn for_backup(conn: &mut PgConnection, backup_id: Uuid) -> QueryResult<Vec<BackupLog>> {
        backup_logs::table
            .filter(backup_logs::backup_id.eq(backup_id))
            .order(backup_logs::created_at.asc())
            .load(conn)
    }

    /// Get recent logs with optional level filtering
    pub fn recent(
        conn: &mut PgConnection,
        level_filter: Option<String>,
        limit: i64,
    ) -> QueryResult<Vec<BackupLog>> {
        let mut query = backup_logs::table.into_boxed();
        
        if let Some(level) = level_filter {
            query = query.filter(backup_logs::level.eq(level));
        }
        
        query
            .order(backup_logs::created_at.desc())
            .limit(limit)
            .load(conn)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupStatistics {
    pub total_backups: i64,
    pub completed_backups: i64,
    pub failed_backups: i64,
    pub total_size: i64,
}
