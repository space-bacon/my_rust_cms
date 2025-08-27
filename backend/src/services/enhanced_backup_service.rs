use std::process::Command;
use std::path::Path;
use std::fs;
use chrono::{DateTime, Utc, Duration};
use sha2::{Sha256, Digest};
use uuid::Uuid;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{
    Backup, NewBackup, UpdateBackup, BackupLog, NewBackupLog, 
    BackupSchedule, BackupInfo, DataSnapshot, TableSnapshot, BackupStatistics
};
use crate::database::DbPool;

#[derive(Debug)]
pub struct EnhancedBackupService {
    pub backup_dir: String,
    pub database_url: String,
    pub max_backup_size: u64,
    pub default_retention_days: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BackupError {
    DatabaseError(String),
    FileSystemError(String),
    ProcessError(String),
    ValidationError(String),
    ConfigurationError(String),
    NetworkError(String),
}

impl std::fmt::Display for BackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            BackupError::FileSystemError(msg) => write!(f, "File system error: {}", msg),
            BackupError::ProcessError(msg) => write!(f, "Process error: {}", msg),
            BackupError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            BackupError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
            BackupError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for BackupError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupRequest {
    pub backup_type: String,
    pub description: Option<String>,
    pub retention_days: Option<i32>,
    pub compress: Option<bool>,
    pub encrypt: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BackupProgress {
    pub backup_id: String,
    pub status: String,
    pub progress_percent: f32,
    pub current_step: String,
    pub estimated_completion: Option<DateTime<Utc>>,
    pub bytes_processed: u64,
    pub total_bytes: Option<u64>,
}

impl EnhancedBackupService {
    pub fn new(backup_dir: String, database_url: String) -> Self {
        // Ensure backup directory exists with proper permissions
        if let Err(e) = fs::create_dir_all(&backup_dir) {
            eprintln!("Warning: Could not create backup directory: {}", e);
        }

        // Set secure permissions on backup directory (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(&backup_dir) {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o750); // Owner and group read/write/execute
                let _ = fs::set_permissions(&backup_dir, permissions);
            }
        }

        Self {
            backup_dir,
            database_url,
            max_backup_size: 10 * 1024 * 1024 * 1024, // 10GB default
            default_retention_days: 30,
        }
    }

    /// Create a comprehensive backup with full tracking
    pub async fn create_backup(
        &self,
        db_pool: &DbPool,
        request: BackupRequest,
        user_id: Option<Uuid>,
    ) -> Result<BackupInfo, BackupError> {
        let mut conn = db_pool.get()
            .map_err(|e| BackupError::DatabaseError(format!("Failed to get database connection: {}", e)))?;

        // Create backup record
        let backup_id = Uuid::new_v4();
        let timestamp = Utc::now();
        let filename = self.generate_filename(&request.backup_type, &timestamp);
        
        let expires_at = request.retention_days
            .map(|days| timestamp + Duration::days(days as i64));

        let new_backup = NewBackup {
            filename: filename.clone(),
            backup_type: request.backup_type.clone(),
            status: Some("in_progress".to_string()),
            file_size: None,
            checksum: None,
            description: request.description.clone(),
            expires_at,
            created_by: user_id,
            metadata: Some(serde_json::json!({
                "compress": request.compress.unwrap_or(true),
                "encrypt": request.encrypt.unwrap_or(false),
                "version": "2.0"
            })),
            retention_policy: Some("default".to_string()),
            is_encrypted: request.encrypt,
            compression_type: Some("gzip".to_string()),
        };

        let backup = Backup::create(&mut conn, new_backup)
            .map_err(|e| BackupError::DatabaseError(format!("Failed to create backup record: {}", e)))?;

        // Log backup start
        self.log_backup_event(&mut conn, backup.id, "info", "Backup started", None)?;

        // Perform the actual backup based on type
        let result = match request.backup_type.as_str() {
            "database" => self.create_database_backup_enhanced(&backup, &mut conn).await,
            "media" => self.create_media_backup_enhanced(&backup, &mut conn).await,
            "full" => self.create_full_backup_enhanced(&backup, &mut conn).await,
            "incremental" => self.create_incremental_backup(&backup, &mut conn).await,
            _ => Err(BackupError::ValidationError("Invalid backup type".to_string())),
        };

        match result {
            Ok(backup_info) => {
                // Update backup record as completed
                let update = UpdateBackup {
                    status: Some("completed".to_string()),
                    file_size: Some(backup_info.size as i64),
                    checksum: Some(backup_info.checksum.clone()),
                    completed_at: Some(Utc::now()),
                    error_message: None,
                    metadata: None,
                };

                Backup::update_status(&mut conn, backup.id, update)
                    .map_err(|e| BackupError::DatabaseError(format!("Failed to update backup status: {}", e)))?;

                self.log_backup_event(&mut conn, backup.id, "info", "Backup completed successfully", None)?;
                Ok(backup_info)
            }
            Err(e) => {
                // Update backup record as failed
                let update = UpdateBackup {
                    status: Some("failed".to_string()),
                    file_size: None,
                    checksum: None,
                    completed_at: Some(Utc::now()),
                    error_message: Some(e.to_string()),
                    metadata: None,
                };

                let _ = Backup::update_status(&mut conn, backup.id, update);
                self.log_backup_event(&mut conn, backup.id, "error", &format!("Backup failed: {}", e), None)?;
                Err(e)
            }
        }
    }

    /// Enhanced database backup with better error handling
    async fn create_database_backup_enhanced(
        &self,
        backup: &Backup,
        conn: &mut PgConnection,
    ) -> Result<BackupInfo, BackupError> {
        let backup_path = Path::new(&self.backup_dir).join(&backup.filename);
        
        // Parse database URL
        let db_params = self.parse_database_url()?;

        self.log_backup_event(conn, backup.id, "info", "Starting database dump", None)?;

        // Check if pg_dump is available
        if Command::new("pg_dump").arg("--version").output().is_err() {
            return Err(BackupError::ConfigurationError(
                "pg_dump not found. Please install PostgreSQL client tools.".to_string()
            ));
        }

        // Execute pg_dump with enhanced options
        let output = Command::new("pg_dump")
            .arg("--host").arg(&db_params.host)
            .arg("--port").arg(&db_params.port.to_string())
            .arg("--username").arg(&db_params.username)
            .arg("--dbname").arg(&db_params.database)
            .arg("--no-password")
            .arg("--verbose")
            .arg("--clean")
            .arg("--create")
            .arg("--if-exists")
            .arg("--format=custom")
            .arg("--compress=9")
            .arg("--file").arg(&backup_path)
            .env("PGPASSWORD", &db_params.password)
            .output()
            .map_err(|e| BackupError::ProcessError(format!("Failed to execute pg_dump: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            self.log_backup_event(
                conn, 
                backup.id, 
                "error", 
                "pg_dump failed", 
                Some(serde_json::json!({
                    "stderr": stderr.to_string(),
                    "stdout": stdout.to_string()
                }))
            )?;
            
            return Err(BackupError::ProcessError(format!("pg_dump failed: {}", stderr)));
        }

        self.finalize_backup(backup, &backup_path).await
    }

    /// Enhanced media backup
    async fn create_media_backup_enhanced(
        &self,
        backup: &Backup,
        conn: &mut PgConnection,
    ) -> Result<BackupInfo, BackupError> {
        let backup_path = Path::new(&self.backup_dir).join(&backup.filename);
        
        self.log_backup_event(conn, backup.id, "info", "Starting media backup", None)?;

        // Check if media directories exist
        let media_paths = vec!["uploads", "backend/uploads"];
        let existing_paths: Vec<&str> = media_paths.iter()
            .filter(|path| Path::new(path).exists())
            .copied()
            .collect();

        if existing_paths.is_empty() {
            return Err(BackupError::ValidationError("No media directories found to backup".to_string()));
        }

        // Create tar.gz archive with better compression
        let mut tar_cmd = Command::new("tar");
        tar_cmd.arg("-czf")
            .arg(&backup_path)
            .arg("-C")
            .arg(".");

        for path in existing_paths {
            tar_cmd.arg(path);
        }

        let output = tar_cmd.output()
            .map_err(|e| BackupError::ProcessError(format!("Failed to execute tar: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            self.log_backup_event(
                conn, 
                backup.id, 
                "error", 
                "tar command failed", 
                Some(serde_json::json!({"stderr": stderr.to_string()}))
            )?;
            return Err(BackupError::ProcessError(format!("tar failed: {}", stderr)));
        }

        self.finalize_backup(backup, &backup_path).await
    }

    /// Full system backup
    async fn create_full_backup_enhanced(
        &self,
        backup: &Backup,
        conn: &mut PgConnection,
    ) -> Result<BackupInfo, BackupError> {
        self.log_backup_event(conn, backup.id, "info", "Starting full system backup", None)?;

        // Create temporary database backup
        let db_backup_name = format!("temp_db_{}.sql", Uuid::new_v4());
        let temp_db_backup = Backup {
            id: Uuid::new_v4(),
            filename: db_backup_name.clone(),
            backup_type: "database".to_string(),
            status: "in_progress".to_string(),
            file_size: None,
            checksum: None,
            description: Some("Temporary database backup for full backup".to_string()),
            created_at: Utc::now(),
            completed_at: None,
            expires_at: None,
            created_by: backup.created_by,
            metadata: serde_json::json!({}),
            error_message: None,
            retention_policy: Some("temporary".to_string()),
            is_encrypted: Some(false),
            compression_type: Some("gzip".to_string()),
        };

        let db_result = self.create_database_backup_enhanced(&temp_db_backup, conn).await?;
        
        // Create temporary media backup
        let media_backup_name = format!("temp_media_{}.tar.gz", Uuid::new_v4());
        let temp_media_backup = Backup {
            id: Uuid::new_v4(),
            filename: media_backup_name.clone(),
            backup_type: "media".to_string(),
            status: "in_progress".to_string(),
            file_size: None,
            checksum: None,
            description: Some("Temporary media backup for full backup".to_string()),
            created_at: Utc::now(),
            completed_at: None,
            expires_at: None,
            created_by: backup.created_by,
            metadata: serde_json::json!({}),
            error_message: None,
            retention_policy: Some("temporary".to_string()),
            is_encrypted: Some(false),
            compression_type: Some("gzip".to_string()),
        };

        let media_result = self.create_media_backup_enhanced(&temp_media_backup, conn).await?;

        // Combine into final archive
        let backup_path = Path::new(&self.backup_dir).join(&backup.filename);
        let output = Command::new("tar")
            .arg("-czf")
            .arg(&backup_path)
            .arg("-C")
            .arg(&self.backup_dir)
            .arg(&db_result.filename)
            .arg(&media_result.filename)
            .output()
            .map_err(|e| BackupError::ProcessError(format!("Failed to create full backup archive: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BackupError::ProcessError(format!("Full backup archive creation failed: {}", stderr)));
        }

        // Clean up temporary files
        let _ = fs::remove_file(Path::new(&self.backup_dir).join(&db_result.filename));
        let _ = fs::remove_file(Path::new(&self.backup_dir).join(&media_result.filename));

        self.finalize_backup(backup, &backup_path).await
    }

    /// Incremental backup (placeholder for future implementation)
    async fn create_incremental_backup(
        &self,
        backup: &Backup,
        conn: &mut PgConnection,
    ) -> Result<BackupInfo, BackupError> {
        self.log_backup_event(conn, backup.id, "info", "Incremental backup not yet implemented", None)?;
        Err(BackupError::ValidationError("Incremental backups not yet implemented".to_string()))
    }

    /// Finalize backup by calculating size and checksum
    async fn finalize_backup(&self, backup: &Backup, backup_path: &Path) -> Result<BackupInfo, BackupError> {
        // Calculate file size and checksum
        let metadata = fs::metadata(backup_path)
            .map_err(|e| BackupError::FileSystemError(format!("Failed to read backup file metadata: {}", e)))?;
        
        let file_contents = fs::read(backup_path)
            .map_err(|e| BackupError::FileSystemError(format!("Failed to read backup file: {}", e)))?;
        
        let checksum = format!("sha256:{}", hex::encode(Sha256::digest(&file_contents)));

        // Set secure permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(backup_path) {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o640); // Owner read/write, group read
                let _ = fs::set_permissions(backup_path, permissions);
            }
        }

        Ok(BackupInfo {
            id: backup.id.to_string(),
            filename: backup.filename.clone(),
            size: metadata.len(),
            created_at: backup.created_at,
            backup_type: backup.backup_type.clone(),
            checksum,
            description: backup.description.clone(),
        })
    }

    /// List all backups with enhanced filtering
    pub async fn list_backups_enhanced(
        &self,
        db_pool: &DbPool,
        backup_type: Option<String>,
        status: Option<String>,
        limit: Option<i64>,
    ) -> Result<Vec<BackupInfo>, BackupError> {
        let mut conn = db_pool.get()
            .map_err(|e| BackupError::DatabaseError(format!("Failed to get database connection: {}", e)))?;

        let backups = Backup::list(&mut conn, backup_type, status, limit)
            .map_err(|e| BackupError::DatabaseError(format!("Failed to list backups: {}", e)))?;

        Ok(backups.into_iter().map(BackupInfo::from).collect())
    }

    /// Get backup statistics
    pub async fn get_backup_statistics(&self, db_pool: &DbPool) -> Result<BackupStatistics, BackupError> {
        let mut conn = db_pool.get()
            .map_err(|e| BackupError::DatabaseError(format!("Failed to get database connection: {}", e)))?;

        Backup::get_statistics(&mut conn)
            .map_err(|e| BackupError::DatabaseError(format!("Failed to get backup statistics: {}", e)))
    }

    /// Cleanup expired backups
    pub async fn cleanup_expired_backups(&self, db_pool: &DbPool) -> Result<usize, BackupError> {
        let mut conn = db_pool.get()
            .map_err(|e| BackupError::DatabaseError(format!("Failed to get database connection: {}", e)))?;

        // Get expired backups before deletion for file cleanup
        let expired_backups = Backup::list(&mut conn, None, Some("completed".to_string()), None)
            .map_err(|e| BackupError::DatabaseError(format!("Failed to list backups: {}", e)))?
            .into_iter()
            .filter(|b| {
                b.expires_at.map_or(false, |exp| exp < Utc::now())
            })
            .collect::<Vec<_>>();

        // Delete physical files
        for backup in &expired_backups {
            let file_path = Path::new(&self.backup_dir).join(&backup.filename);
            if file_path.exists() {
                let _ = fs::remove_file(file_path);
            }
        }

        // Delete database records
        let deleted_count = Backup::cleanup_expired(&mut conn)
            .map_err(|e| BackupError::DatabaseError(format!("Failed to cleanup expired backups: {}", e)))?;

        Ok(deleted_count)
    }

    /// Generate filename for backup
    fn generate_filename(&self, backup_type: &str, timestamp: &DateTime<Utc>) -> String {
        let backup_id = Uuid::new_v4();
        let timestamp_str = timestamp.format("%Y%m%d_%H%M%S");
        
        match backup_type {
            "database" => format!("db_backup_{}_{}.pgdump", &backup_id.to_string()[..8], timestamp_str),
            "media" => format!("media_backup_{}_{}.tar.gz", &backup_id.to_string()[..8], timestamp_str),
            "full" => format!("full_backup_{}_{}.tar.gz", &backup_id.to_string()[..8], timestamp_str),
            "incremental" => format!("inc_backup_{}_{}.tar.gz", &backup_id.to_string()[..8], timestamp_str),
            _ => format!("backup_{}_{}.tar.gz", &backup_id.to_string()[..8], timestamp_str),
        }
    }

    /// Log backup events
    fn log_backup_event(
        &self,
        conn: &mut PgConnection,
        backup_id: Uuid,
        level: &str,
        message: &str,
        details: Option<serde_json::Value>,
    ) -> Result<(), BackupError> {
        let log_entry = NewBackupLog {
            backup_id: Some(backup_id),
            schedule_id: None,
            level: level.to_string(),
            message: message.to_string(),
            details,
        };

        BackupLog::create(conn, log_entry)
            .map_err(|e| BackupError::DatabaseError(format!("Failed to create backup log: {}", e)))?;

        Ok(())
    }

    /// Parse database URL (improved version)
    fn parse_database_url(&self) -> Result<DatabaseParams, BackupError> {
        let url = &self.database_url;
        
        if !url.starts_with("postgres://") && !url.starts_with("postgresql://") {
            return Err(BackupError::ValidationError("Invalid database URL format".to_string()));
        }

        let without_protocol = url.strip_prefix("postgres://")
            .or_else(|| url.strip_prefix("postgresql://"))
            .unwrap();
            
        let parts: Vec<&str> = without_protocol.split('@').collect();
        
        if parts.len() != 2 {
            return Err(BackupError::ValidationError("Invalid database URL format".to_string()));
        }

        let auth_parts: Vec<&str> = parts[0].split(':').collect();
        let host_parts: Vec<&str> = parts[1].split('/').collect();
        
        if auth_parts.len() != 2 || host_parts.len() != 2 {
            return Err(BackupError::ValidationError("Invalid database URL format".to_string()));
        }

        let host_port: Vec<&str> = host_parts[0].split(':').collect();
        let host = host_port[0].to_string();
        let port = host_port.get(1).unwrap_or(&"5432").parse::<u16>()
            .map_err(|_| BackupError::ValidationError("Invalid port number".to_string()))?;

        Ok(DatabaseParams {
            username: auth_parts[0].to_string(),
            password: auth_parts[1].to_string(),
            host,
            port,
            database: host_parts[1].to_string(),
        })
    }

    /// Create data snapshot with integrity verification
    pub async fn create_data_snapshot(&self, db_pool: &DbPool) -> Result<DataSnapshot, BackupError> {
        let mut conn = db_pool.get()
            .map_err(|e| BackupError::DatabaseError(format!("Failed to get database connection: {}", e)))?;

        let timestamp = Utc::now();
        let mut table_snapshots = Vec::new();
        let mut total_rows = 0i64;
        let mut table_hashes = Vec::new();

        // Define tables to include in snapshot
        let tables = vec![
            "users", "posts", "pages", "media", "comments", 
            "categories", "settings", "sessions", "navigation",
            "templates", "components", "page_components", "backups"
        ];

        for table_name in &tables {
            let row_count = self.get_table_row_count(&mut conn, table_name)?;
            total_rows += row_count;

            let table_hash = self.calculate_table_hash(&mut conn, table_name)?;
            table_hashes.push(table_hash.clone());

            table_snapshots.push(TableSnapshot {
                table_name: table_name.to_string(),
                row_count,
                table_hash,
                last_modified: Some(timestamp),
            });
        }

        let merkle_root = self.calculate_merkle_root(&table_hashes);
        let integrity_verified = self.verify_snapshot_integrity(&table_snapshots, &merkle_root);

        Ok(DataSnapshot {
            timestamp,
            tables: table_snapshots,
            total_rows,
            data_hash: merkle_root,
            integrity_verified,
        })
    }

    /// Get table row count with better error handling
    fn get_table_row_count(&self, conn: &mut PgConnection, table_name: &str) -> Result<i64, BackupError> {
        let count = match table_name {
            "users" => {
                use crate::schema::users::dsl::*;
                users.count().get_result(conn).unwrap_or(0)
            },
            "posts" => {
                use crate::schema::posts::dsl::*;
                posts.count().get_result(conn).unwrap_or(0)
            },
            "pages" => {
                use crate::schema::pages::dsl::*;
                pages.count().get_result(conn).unwrap_or(0)
            },
            "media" => {
                use crate::schema::media::dsl::*;
                media.count().get_result(conn).unwrap_or(0)
            },
            "comments" => {
                use crate::schema::comments::dsl::*;
                comments.count().get_result(conn).unwrap_or(0)
            },
            "categories" => {
                use crate::schema::categories::dsl::*;
                categories.count().get_result(conn).unwrap_or(0)
            },
            "settings" => {
                use crate::schema::settings::dsl::*;
                settings.count().get_result(conn).unwrap_or(0)
            },
            "sessions" => {
                use crate::schema::sessions::dsl::*;
                sessions.count().get_result(conn).unwrap_or(0)
            },
            "navigation" => {
                use crate::schema::navigation::dsl::*;
                navigation.count().get_result(conn).unwrap_or(0)
            },
            "templates" => {
                use crate::schema::templates::dsl::*;
                templates.count().get_result(conn).unwrap_or(0)
            },
            "components" => {
                use crate::schema::components::dsl::*;
                components.count().get_result(conn).unwrap_or(0)
            },
            "page_components" => {
                use crate::schema::page_components::dsl::*;
                page_components.count().get_result(conn).unwrap_or(0)
            },
            _ => 0,
        };
        
        Ok(count)
    }

    /// Calculate table hash
    fn calculate_table_hash(&self, conn: &mut PgConnection, table_name: &str) -> Result<String, BackupError> {
        let row_count = self.get_table_row_count(conn, table_name)?;
        let combined = format!("{}:{}", table_name, row_count);
        let table_hash = hex::encode(Sha256::digest(combined.as_bytes()));
        
        Ok(format!("sha256:{}", table_hash))
    }

    /// Calculate Merkle root hash
    fn calculate_merkle_root(&self, table_hashes: &[String]) -> String {
        if table_hashes.is_empty() {
            return "sha256:empty".to_string();
        }

        let mut hashes = table_hashes.to_vec();
        
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            
            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    format!("{}{}", chunk[0], chunk[0])
                };
                
                let hash = hex::encode(Sha256::digest(combined.as_bytes()));
                next_level.push(format!("sha256:{}", hash));
            }
            
            hashes = next_level;
        }

        hashes.into_iter().next().unwrap_or_else(|| "sha256:empty".to_string())
    }

    /// Verify snapshot integrity
    fn verify_snapshot_integrity(&self, table_snapshots: &[TableSnapshot], expected_root: &str) -> bool {
        let table_hashes: Vec<String> = table_snapshots.iter()
            .map(|ts| ts.table_hash.clone())
            .collect();
        
        let calculated_root = self.calculate_merkle_root(&table_hashes);
        calculated_root == expected_root
    }
}

#[derive(Debug)]
struct DatabaseParams {
    username: String,
    password: String,
    host: String,
    port: u16,
    database: String,
}
