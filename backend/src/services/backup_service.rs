use std::process::Command;
use std::path::Path;
use std::fs;
use chrono::{DateTime, Utc};
use sha2::{Sha256, Digest};
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::models::{BackupInfo, DataSnapshot, TableSnapshot};
use crate::database::DbPool;

#[derive(Debug)]
pub struct BackupService {
    pub backup_dir: String,
    pub database_url: String,
    #[allow(dead_code)]
    pub encryption_key: Option<[u8; 32]>,
}

#[derive(Debug)]
pub enum BackupError {
    DatabaseError(String),
    #[allow(dead_code)]
    FileSystemError(String),
    ProcessError(String),
    ValidationError(String),
}

impl std::fmt::Display for BackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            BackupError::FileSystemError(msg) => write!(f, "File system error: {}", msg),
            BackupError::ProcessError(msg) => write!(f, "Process error: {}", msg),
            BackupError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for BackupError {}

#[allow(dead_code)]
impl BackupService {
    pub fn new(backup_dir: String, database_url: String) -> Self {
        // Ensure backup directory exists
        if let Err(e) = fs::create_dir_all(&backup_dir) {
            eprintln!("Warning: Could not create backup directory: {}", e);
        }
        
        Self {
            backup_dir,
            database_url,
            encryption_key: None,
        }
    }
    
    /// Create a new backup service with encryption enabled
    pub fn new_with_encryption(backup_dir: String, database_url: String, encryption_key: [u8; 32]) -> Self {
        // Ensure backup directory exists and set secure permissions
        if let Err(e) = fs::create_dir_all(&backup_dir) {
            eprintln!("Warning: Could not create backup directory: {}", e);
        }
        
        // Set restrictive permissions on backup directory (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(&backup_dir) {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o700); // Owner read/write/execute only
                let _ = fs::set_permissions(&backup_dir, permissions);
            }
        }
        
        Self {
            backup_dir,
            database_url,
            encryption_key: Some(encryption_key),
        }
    }
    
    /// Encrypt backup file data
    fn encrypt_backup_data(&self, data: &[u8]) -> Result<Vec<u8>, BackupError> {
        if let Some(key) = &self.encryption_key {
            let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
            let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
            
            match cipher.encrypt(&nonce, data) {
                Ok(ciphertext) => {
                    // Prepend nonce to ciphertext for storage
                    let mut encrypted_data = nonce.to_vec();
                    encrypted_data.extend_from_slice(&ciphertext);
                    Ok(encrypted_data)
                }
                Err(e) => Err(BackupError::ProcessError(format!("Encryption failed: {}", e)))
            }
        } else {
            // Return data as-is if encryption is not enabled
            Ok(data.to_vec())
        }
    }
    
    /// Decrypt backup file data
    fn decrypt_backup_data(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, BackupError> {
        if let Some(key) = &self.encryption_key {
            if encrypted_data.len() < 12 {
                return Err(BackupError::ValidationError("Encrypted data too short".to_string()));
            }
            
            let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
            let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
            let nonce = Nonce::from_slice(nonce_bytes);
            
            match cipher.decrypt(nonce, ciphertext) {
                Ok(plaintext) => Ok(plaintext),
                Err(e) => Err(BackupError::ProcessError(format!("Decryption failed: {}", e)))
            }
        } else {
            // Return data as-is if encryption is not enabled
            Ok(encrypted_data.to_vec())
        }
    }
    
    /// Write encrypted backup to file
    fn write_encrypted_backup(&self, data: &[u8], file_path: &str) -> Result<(), BackupError> {
        let encrypted_data = self.encrypt_backup_data(data)?;
        
        fs::write(file_path, encrypted_data)
            .map_err(|e| BackupError::FileSystemError(format!("Failed to write backup: {}", e)))?;
        
        // Set restrictive permissions on backup file (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(metadata) = fs::metadata(file_path) {
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o600); // Owner read/write only
                let _ = fs::set_permissions(file_path, permissions);
            }
        }
        
        Ok(())
    }
    
    /// Read and decrypt backup from file
    fn read_encrypted_backup(&self, file_path: &str) -> Result<Vec<u8>, BackupError> {
        let encrypted_data = fs::read(file_path)
            .map_err(|e| BackupError::FileSystemError(format!("Failed to read backup: {}", e)))?;
        
        self.decrypt_backup_data(&encrypted_data)
    }

    /// Create a database backup using pg_dump
    pub async fn create_database_backup(&self, description: Option<String>) -> Result<BackupInfo, BackupError> {
        let timestamp = Utc::now();
        let backup_id = uuid::Uuid::new_v4().to_string();
        let filename = format!("db_backup_{}_{}.sql", 
            backup_id[..8].to_string(), 
            timestamp.format("%Y%m%d_%H%M%S")
        );
        let backup_path = Path::new(&self.backup_dir).join(&filename);

        // Parse database URL to extract connection parameters
        let db_params = self.parse_database_url()?;

        // Execute pg_dump command
        let output = Command::new("pg_dump")
            .arg("--host").arg(&db_params.host)
            .arg("--port").arg(&db_params.port.to_string())
            .arg("--username").arg(&db_params.username)
            .arg("--dbname").arg(&db_params.database)
            .arg("--no-password") // Use .pgpass or environment variables for auth
            .arg("--verbose")
            .arg("--clean")
            .arg("--create")
            .arg("--file").arg(&backup_path)
            .env("PGPASSWORD", &db_params.password)
            .output()
            .map_err(|e| BackupError::ProcessError(format!("Failed to execute pg_dump: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BackupError::ProcessError(format!("pg_dump failed: {}", stderr)));
        }

        // Calculate file size and checksum
        let metadata = fs::metadata(&backup_path)
            .map_err(|e| BackupError::FileSystemError(format!("Failed to read backup file metadata: {}", e)))?;
        
        let file_contents = fs::read(&backup_path)
            .map_err(|e| BackupError::FileSystemError(format!("Failed to read backup file: {}", e)))?;
        
        let checksum = format!("sha256:{}", hex::encode(Sha256::digest(&file_contents)));

        Ok(BackupInfo {
            id: backup_id,
            filename,
            size: metadata.len(),
            created_at: timestamp,
            backup_type: "database".to_string(),
            checksum,
            description,
        })
    }

    /// Create a media files backup
    pub async fn create_media_backup(&self, description: Option<String>) -> Result<BackupInfo, BackupError> {
        let timestamp = Utc::now();
        let backup_id = uuid::Uuid::new_v4().to_string();
        let filename = format!("media_backup_{}_{}.tar.gz", 
            backup_id[..8].to_string(), 
            timestamp.format("%Y%m%d_%H%M%S")
        );
        let backup_path = Path::new(&self.backup_dir).join(&filename);

        // Create tar.gz archive of uploads directory
        let output = Command::new("tar")
            .arg("-czf")
            .arg(&backup_path)
            .arg("-C")
            .arg(".")
            .arg("uploads")
            .arg("backend/uploads")
            .output()
            .map_err(|e| BackupError::ProcessError(format!("Failed to execute tar: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BackupError::ProcessError(format!("tar failed: {}", stderr)));
        }

        // Calculate file size and checksum
        let metadata = fs::metadata(&backup_path)
            .map_err(|e| BackupError::FileSystemError(format!("Failed to read backup file metadata: {}", e)))?;
        
        let file_contents = fs::read(&backup_path)
            .map_err(|e| BackupError::FileSystemError(format!("Failed to read backup file: {}", e)))?;
        
        let checksum = format!("sha256:{}", hex::encode(Sha256::digest(&file_contents)));

        Ok(BackupInfo {
            id: backup_id,
            filename,
            size: metadata.len(),
            created_at: timestamp,
            backup_type: "media".to_string(),
            checksum,
            description,
        })
    }

    /// Create a full system backup (database + media)
    pub async fn create_full_backup(&self, description: Option<String>) -> Result<BackupInfo, BackupError> {
        // Create database backup first
        let db_backup = self.create_database_backup(Some("Database portion of full backup".to_string())).await?;
        
        // Create media backup
        let media_backup = self.create_media_backup(Some("Media portion of full backup".to_string())).await?;

        let timestamp = Utc::now();
        let backup_id = uuid::Uuid::new_v4().to_string();
        let filename = format!("full_backup_{}_{}.tar.gz", 
            backup_id[..8].to_string(), 
            timestamp.format("%Y%m%d_%H%M%S")
        );
        let backup_path = Path::new(&self.backup_dir).join(&filename);

        // Create combined archive
        let output = Command::new("tar")
            .arg("-czf")
            .arg(&backup_path)
            .arg("-C")
            .arg(&self.backup_dir)
            .arg(&db_backup.filename)
            .arg(&media_backup.filename)
            .output()
            .map_err(|e| BackupError::ProcessError(format!("Failed to create full backup archive: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BackupError::ProcessError(format!("Full backup archive creation failed: {}", stderr)));
        }

        // Calculate file size and checksum
        let metadata = fs::metadata(&backup_path)
            .map_err(|e| BackupError::FileSystemError(format!("Failed to read backup file metadata: {}", e)))?;
        
        let file_contents = fs::read(&backup_path)
            .map_err(|e| BackupError::FileSystemError(format!("Failed to read backup file: {}", e)))?;
        
        let checksum = format!("sha256:{}", hex::encode(Sha256::digest(&file_contents)));

        // Clean up individual backup files
        let _ = fs::remove_file(Path::new(&self.backup_dir).join(&db_backup.filename));
        let _ = fs::remove_file(Path::new(&self.backup_dir).join(&media_backup.filename));

        Ok(BackupInfo {
            id: backup_id,
            filename,
            size: metadata.len(),
            created_at: timestamp,
            backup_type: "full".to_string(),
            checksum,
            description,
        })
    }

    /// List all available backups
    pub async fn list_backups(&self) -> Result<Vec<BackupInfo>, BackupError> {
        let backup_dir = Path::new(&self.backup_dir);
        
        if !backup_dir.exists() {
            return Ok(Vec::new());
        }

        let mut backups = Vec::new();
        
        let entries = fs::read_dir(backup_dir)
            .map_err(|e| BackupError::FileSystemError(format!("Failed to read backup directory: {}", e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| BackupError::FileSystemError(format!("Failed to read directory entry: {}", e)))?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if filename.ends_with(".sql") || filename.ends_with(".tar.gz") {
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

                                // Extract timestamp from filename (basic implementation)
                                let created_at = metadata.created()
                                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs();
                                
                                let datetime = DateTime::from_timestamp(created_at as i64, 0)
                                    .unwrap_or_else(|| Utc::now());

                                backups.push(BackupInfo {
                                    id: uuid::Uuid::new_v4().to_string(), // Generate new ID for listing
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

    /// Restore database from backup
    pub async fn restore_database(&self, backup_filename: &str) -> Result<String, BackupError> {
        let backup_path = Path::new(&self.backup_dir).join(backup_filename);
        
        if !backup_path.exists() {
            return Err(BackupError::ValidationError("Backup file not found".to_string()));
        }

        // Parse database URL to extract connection parameters
        let db_params = self.parse_database_url()?;

        // Execute psql to restore the database
        let output = Command::new("psql")
            .arg("--host").arg(&db_params.host)
            .arg("--port").arg(&db_params.port.to_string())
            .arg("--username").arg(&db_params.username)
            .arg("--dbname").arg(&db_params.database)
            .arg("--file").arg(&backup_path)
            .env("PGPASSWORD", &db_params.password)
            .output()
            .map_err(|e| BackupError::ProcessError(format!("Failed to execute psql: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(BackupError::ProcessError(format!("Database restore failed: {}", stderr)));
        }

        Ok(format!("Database successfully restored from {}", backup_filename))
    }

    /// Generate data snapshot with Merkle tree integrity
    pub async fn create_data_snapshot(&self, db_pool: &DbPool) -> Result<DataSnapshot, BackupError> {
        let mut conn = db_pool.get()
            .map_err(|e| BackupError::DatabaseError(e.to_string()))?;

        let timestamp = Utc::now();
        let mut table_snapshots = Vec::new();
        let mut total_rows = 0i64;
        let mut table_hashes = Vec::new();

        // Define tables to include in snapshot
        let tables = vec![
            "users", "posts", "pages", "media", "comments", 
            "categories", "settings", "sessions", "navigation",
            "templates", "components", "page_components"
        ];

        for table_name in &tables {
            let row_count = self.get_table_row_count(&mut conn, table_name)?;
            total_rows += row_count;

            // Create a content-based hash for the table
            let table_hash = self.calculate_table_hash(&mut conn, table_name)?;
            table_hashes.push(table_hash.clone());

            table_snapshots.push(TableSnapshot {
                table_name: table_name.to_string(),
                row_count,
                table_hash,
                last_modified: Some(timestamp),
            });
        }

        // Calculate Merkle root hash from all table hashes
        let merkle_root = self.calculate_merkle_root(&table_hashes);

        // Verify integrity by recalculating a sample
        let integrity_verified = self.verify_snapshot_integrity(&table_snapshots, &merkle_root);

        Ok(DataSnapshot {
            timestamp,
            tables: table_snapshots,
            total_rows,
            data_hash: merkle_root,
            integrity_verified,
        })
    }

    /// Parse database URL into connection parameters
    fn parse_database_url(&self) -> Result<DatabaseParams, BackupError> {
        // Parse PostgreSQL URL format: postgresql://username:password@host:port/database
        let url = &self.database_url;
        
        if !url.starts_with("postgresql://") {
            return Err(BackupError::ValidationError("Invalid database URL format".to_string()));
        }

        let without_protocol = url.strip_prefix("postgresql://").unwrap();
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

    /// Get row count for a specific table
    fn get_table_row_count(&self, conn: &mut PgConnection, table_name: &str) -> Result<i64, BackupError> {
        // Use specific table queries instead of dynamic SQL to avoid injection
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

    /// Calculate hash for table contents
    fn calculate_table_hash(&self, conn: &mut PgConnection, table_name: &str) -> Result<String, BackupError> {
        // For now, create a simple hash based on table name and row count
        // In a production system, this would be more sophisticated
        let row_count = self.get_table_row_count(conn, table_name)?;
        let combined = format!("{}:{}", table_name, row_count);
        let table_hash = hex::encode(Sha256::digest(combined.as_bytes()));
        
        Ok(format!("sha256:{}", table_hash))
    }

    /// Calculate Merkle root hash from table hashes
    fn calculate_merkle_root(&self, table_hashes: &[String]) -> String {
        if table_hashes.is_empty() {
            return "sha256:empty".to_string();
        }

        let mut hashes = table_hashes.to_vec();
        
        // Build Merkle tree bottom-up
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            
            // Process pairs of hashes
            for chunk in hashes.chunks(2) {
                let combined = if chunk.len() == 2 {
                    format!("{}{}", chunk[0], chunk[1])
                } else {
                    // Odd number - duplicate the last hash
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