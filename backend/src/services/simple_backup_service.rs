use std::process::Command;
use std::path::Path;
use std::fs;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::models::setting::{BackupInfo};

#[derive(Debug)]
pub struct SimpleBackupService {
    pub backup_dir: String,
    pub database_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SimpleBackupError {
    FileSystemError(String),
    ProcessError(String),
    ValidationError(String),
    ConfigurationError(String),
}

impl std::fmt::Display for SimpleBackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleBackupError::FileSystemError(msg) => write!(f, "File system error: {}", msg),
            SimpleBackupError::ProcessError(msg) => write!(f, "Process error: {}", msg),
            SimpleBackupError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            SimpleBackupError::ConfigurationError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for SimpleBackupError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleBackupRequest {
    pub backup_type: String,
    pub description: Option<String>,
}

impl SimpleBackupService {
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
        }
    }

    /// Create a backup without database dependency
    pub async fn create_backup_simple(
        &self,
        request: SimpleBackupRequest,
    ) -> Result<BackupInfo, SimpleBackupError> {
        let timestamp = Utc::now();
        let _backup_id = Uuid::new_v4().to_string();
        let filename = self.generate_filename(&request.backup_type, &timestamp);
        
        let result = match request.backup_type.as_str() {
            "database" => self.create_database_backup_simple(&filename, request.description).await,
            "media" => self.create_media_backup_simple(&filename, request.description).await,
            "full" => self.create_full_backup_simple(&filename, request.description).await,
            _ => Err(SimpleBackupError::ValidationError("Invalid backup type".to_string())),
        };

        match result {
            Ok(backup_info) => {
                println!("✅ Backup completed successfully: {}", backup_info.filename);
                Ok(backup_info)
            }
            Err(e) => {
                eprintln!("❌ Backup failed: {}", e);
                Err(e)
            }
        }
    }

    /// Simple database backup
    async fn create_database_backup_simple(
        &self,
        filename: &str,
        description: Option<String>,
    ) -> Result<BackupInfo, SimpleBackupError> {
        let backup_path = Path::new(&self.backup_dir).join(filename);
        
        // Parse database URL
        let db_params = self.parse_database_url()?;

        println!("🔄 Starting database backup...");

        // Check if pg_dump is available
        if Command::new("pg_dump").arg("--version").output().is_err() {
            return Err(SimpleBackupError::ConfigurationError(
                "pg_dump not found. Please install PostgreSQL client tools.".to_string()
            ));
        }

        // Execute pg_dump with fallback options
        let output = Command::new("pg_dump")
            .arg("--host").arg(&db_params.host)
            .arg("--port").arg(&db_params.port.to_string())
            .arg("--username").arg(&db_params.username)
            .arg("--dbname").arg(&db_params.database)
            .arg("--no-password")
            .arg("--format=custom")
            .arg("--compress=9")
            .arg("--file").arg(&backup_path)
            .env("PGPASSWORD", &db_params.password)
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    
                    // If custom format fails, try plain SQL format
                    println!("⚠️  Custom format failed, trying plain SQL format...");
                    let fallback_output = Command::new("pg_dump")
                        .arg("--host").arg(&db_params.host)
                        .arg("--port").arg(&db_params.port.to_string())
                        .arg("--username").arg(&db_params.username)
                        .arg("--dbname").arg(&db_params.database)
                        .arg("--no-password")
                        .arg("--clean")
                        .arg("--create")
                        .arg("--file").arg(&backup_path)
                        .env("PGPASSWORD", &db_params.password)
                        .output()
                        .map_err(|e| SimpleBackupError::ProcessError(format!("Failed to execute pg_dump fallback: {}", e)))?;

                    if !fallback_output.status.success() {
                        let fallback_stderr = String::from_utf8_lossy(&fallback_output.stderr);
                        return Err(SimpleBackupError::ProcessError(format!("pg_dump failed: {}\nFallback also failed: {}", stderr, fallback_stderr)));
                    }
                }
            }
            Err(e) => {
                return Err(SimpleBackupError::ProcessError(format!("Failed to execute pg_dump: {}", e)));
            }
        }

        self.finalize_backup_simple(filename, "database", description).await
    }

    /// Simple media backup
    async fn create_media_backup_simple(
        &self,
        filename: &str,
        description: Option<String>,
    ) -> Result<BackupInfo, SimpleBackupError> {
        let backup_path = Path::new(&self.backup_dir).join(filename);
        
        println!("🔄 Starting media backup...");

        // Check if media directories exist
        let media_paths = vec!["uploads", "backend/uploads"];
        let existing_paths: Vec<&str> = media_paths.iter()
            .filter(|path| Path::new(path).exists())
            .copied()
            .collect();

        if existing_paths.is_empty() {
            println!("⚠️  No media directories found, creating empty archive");
            // Create empty tar file
            let output = Command::new("tar")
                .arg("-czf")
                .arg(&backup_path)
                .arg("-T")
                .arg("/dev/null")
                .output()
                .map_err(|e| SimpleBackupError::ProcessError(format!("Failed to create empty archive: {}", e)))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(SimpleBackupError::ProcessError(format!("Failed to create empty archive: {}", stderr)));
            }
        } else {
            // Create tar.gz archive
            let mut tar_cmd = Command::new("tar");
            tar_cmd.arg("-czf")
                .arg(&backup_path)
                .arg("-C")
                .arg(".");

            for path in existing_paths {
                tar_cmd.arg(path);
            }

            let output = tar_cmd.output()
                .map_err(|e| SimpleBackupError::ProcessError(format!("Failed to execute tar: {}", e)))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(SimpleBackupError::ProcessError(format!("tar failed: {}", stderr)));
            }
        }

        self.finalize_backup_simple(filename, "media", description).await
    }

    /// Simple full backup
    async fn create_full_backup_simple(
        &self,
        filename: &str,
        description: Option<String>,
    ) -> Result<BackupInfo, SimpleBackupError> {
        println!("🔄 Starting full system backup...");

        // Create temporary database backup
        let db_backup_name = format!("temp_db_{}.pgdump", Uuid::new_v4());
        let _db_result = self.create_database_backup_simple(&db_backup_name, Some("Temporary database backup for full backup".to_string())).await?;
        
        // Create temporary media backup
        let media_backup_name = format!("temp_media_{}.tar.gz", Uuid::new_v4());
        let _media_result = self.create_media_backup_simple(&media_backup_name, Some("Temporary media backup for full backup".to_string())).await?;

        // Combine into final archive
        let backup_path = Path::new(&self.backup_dir).join(filename);
        let output = Command::new("tar")
            .arg("-czf")
            .arg(&backup_path)
            .arg("-C")
            .arg(&self.backup_dir)
            .arg(&db_backup_name)
            .arg(&media_backup_name)
            .output()
            .map_err(|e| SimpleBackupError::ProcessError(format!("Failed to create full backup archive: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(SimpleBackupError::ProcessError(format!("Full backup archive creation failed: {}", stderr)));
        }

        // Clean up temporary files
        let _ = fs::remove_file(Path::new(&self.backup_dir).join(&db_backup_name));
        let _ = fs::remove_file(Path::new(&self.backup_dir).join(&media_backup_name));

        self.finalize_backup_simple(filename, "full", description).await
    }

    /// List backups from filesystem
    pub async fn list_backups_simple(&self) -> Result<Vec<BackupInfo>, SimpleBackupError> {
        let backup_dir = Path::new(&self.backup_dir);
        
        if !backup_dir.exists() {
            return Ok(Vec::new());
        }

        let mut backups = Vec::new();
        
        let entries = fs::read_dir(backup_dir)
            .map_err(|e| SimpleBackupError::FileSystemError(format!("Failed to read backup directory: {}", e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| SimpleBackupError::FileSystemError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if filename.ends_with(".sql") || filename.ends_with(".tar.gz") || filename.ends_with(".pgdump") {
                        if let Ok(metadata) = fs::metadata(&path) {
                            if let Ok(file_contents) = fs::read(&path) {
                                let checksum = format!("sha256:{}", hex::encode(Sha256::digest(&file_contents)));
                                
                                // Extract backup type from filename
                                let backup_type = if filename.contains("db_backup") {
                                    "database"
                                } else if filename.contains("media_backup") {
                                    "media"
                                } else if filename.contains("full_backup") {
                                    "full"
                                } else {
                                    "unknown"
                                };

                                // Extract timestamp from filename or use file creation time
                                let created_at = metadata.created()
                                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs();
                                
                                let datetime = DateTime::from_timestamp(created_at as i64, 0)
                                    .unwrap_or_else(|| Utc::now());

                                backups.push(BackupInfo {
                                    id: Uuid::new_v4().to_string(),
                                    filename: filename.to_string(),
                                    size: metadata.len(),
                                    created_at: datetime,
                                    backup_type: backup_type.to_string(),
                                    checksum,
                                    description: None,
                                });
                            }
                        }
                    }
                }
            }
        }

        // Sort by creation time (newest first)
        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        Ok(backups)
    }

    /// Finalize backup by calculating size and checksum
    async fn finalize_backup_simple(&self, filename: &str, backup_type: &str, description: Option<String>) -> Result<BackupInfo, SimpleBackupError> {
        let backup_path = Path::new(&self.backup_dir).join(filename);
        
        // Calculate file size and checksum
        let metadata = fs::metadata(&backup_path)
            .map_err(|e| SimpleBackupError::FileSystemError(format!("Failed to read backup file metadata: {}", e)))?;
        
        let file_contents = fs::read(&backup_path)
            .map_err(|e| SimpleBackupError::FileSystemError(format!("Failed to read backup file: {}", e)))?;
        
        let checksum = format!("sha256:{}", hex::encode(Sha256::digest(&file_contents)));

        // Set secure permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(&backup_path) {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o640); // Owner read/write, group read
                let _ = fs::set_permissions(&backup_path, permissions);
            }
        }

        Ok(BackupInfo {
            id: Uuid::new_v4().to_string(),
            filename: filename.to_string(),
            size: metadata.len(),
            created_at: Utc::now(),
            backup_type: backup_type.to_string(),
            checksum,
            description,
        })
    }

    /// Generate filename for backup
    fn generate_filename(&self, backup_type: &str, timestamp: &DateTime<Utc>) -> String {
        let backup_id = Uuid::new_v4();
        let timestamp_str = timestamp.format("%Y%m%d_%H%M%S");
        
        match backup_type {
            "database" => format!("db_backup_{}_{}.pgdump", &backup_id.to_string()[..8], timestamp_str),
            "media" => format!("media_backup_{}_{}.tar.gz", &backup_id.to_string()[..8], timestamp_str),
            "full" => format!("full_backup_{}_{}.tar.gz", &backup_id.to_string()[..8], timestamp_str),
            _ => format!("backup_{}_{}.tar.gz", &backup_id.to_string()[..8], timestamp_str),
        }
    }

    /// Parse database URL
    fn parse_database_url(&self) -> Result<DatabaseParams, SimpleBackupError> {
        let url = &self.database_url;
        

        
        if !url.starts_with("postgres://") && !url.starts_with("postgresql://") {
            return Err(SimpleBackupError::ValidationError("Invalid database URL format".to_string()));
        }

        let without_protocol = url.strip_prefix("postgres://")
            .or_else(|| url.strip_prefix("postgresql://"))
            .unwrap();
            
        let parts: Vec<&str> = without_protocol.split('@').collect();
        
        if parts.len() != 2 {
            return Err(SimpleBackupError::ValidationError("Invalid database URL format".to_string()));
        }

        let auth_parts: Vec<&str> = parts[0].split(':').collect();
        let host_parts: Vec<&str> = parts[1].split('/').collect();
        
        if auth_parts.len() != 2 || host_parts.len() != 2 {
            return Err(SimpleBackupError::ValidationError("Invalid database URL format".to_string()));
        }

        let host_port: Vec<&str> = host_parts[0].split(':').collect();
        let host = host_port[0].to_string();
        let port = host_port.get(1).unwrap_or(&"5432").parse::<u16>()
            .map_err(|_| SimpleBackupError::ValidationError("Invalid port number".to_string()))?;

        let params = DatabaseParams {
            username: auth_parts[0].to_string(),
            password: auth_parts[1].to_string(),
            host,
            port,
            database: host_parts[1].to_string(),
        };
        

        
        Ok(params)
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
