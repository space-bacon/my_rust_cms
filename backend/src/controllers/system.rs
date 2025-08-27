use axum::{
    extract::{State, Query, Path, Json},
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::{
    models::{Setting, SystemInfo},
    models::setting::{BackupInfo, DataSnapshot},
    middleware::errors::AppError,
    services::{BackupService, SimpleBackupService, SimpleBackupRequest},
    AppServices,
};

#[derive(Deserialize)]
pub struct SettingsQuery {
    pub setting_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SettingsRequest {
    pub settings: Vec<SettingData>,
}

#[derive(Serialize, Deserialize)]
pub struct SettingData {
    pub key: String,
    pub value: String,
    pub setting_type: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BackupRequest {
    pub backup_type: String, // "database", "media", "full"
    pub description: Option<String>,
}

// Get all settings or settings by type
pub async fn get_settings(
    State(services): State<AppServices>,
    Query(params): Query<SettingsQuery>
) -> Result<ResponseJson<Vec<Setting>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseConnection(e.to_string()))?;

    let settings = match params.setting_type {
        Some(setting_type) => Setting::list_by_type(&mut conn, &setting_type),
        None => Setting::list(&mut conn),
    }
    .map_err(|e| AppError::DatabaseQuery(e.to_string()))?;

    Ok(ResponseJson(settings))
}

// Public: get a safe subset of settings (e.g., site/container/theme)
pub async fn get_public_settings(
    State(services): State<AppServices>,
    Query(params): Query<SettingsQuery>
) -> Result<ResponseJson<Vec<Setting>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseConnection(e.to_string()))?;

    // Only allow specific types to be exposed publicly
    let allowed_types = ["site", "container", "theme"];

    let settings = match params.setting_type {
        Some(ref setting_type) if allowed_types.contains(&setting_type.as_str()) => {
            Setting::list_by_type(&mut conn, setting_type)
        }
        // If no type provided, return only explicitly whitelisted keys that are safe
        _ => {
            use diesel::prelude::*;
            use crate::schema::settings::dsl as s;
            s::settings
                .filter(s::setting_type.eq_any(allowed_types))
                .load::<Setting>(&mut conn)
        }
    }
    .map_err(|e| AppError::DatabaseQuery(e.to_string()))?;

    Ok(ResponseJson(settings))
}

// Get specific setting by key
pub async fn get_setting(
    State(services): State<AppServices>,
    Path(key): Path<String>
) -> Result<ResponseJson<Setting>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseConnection(e.to_string()))?;

    let setting = Setting::find_by_key(&mut conn, &key)
        .map_err(|e| AppError::DatabaseQuery(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Setting not found".to_string()))?;

    Ok(ResponseJson(setting))
}

// Update multiple settings
pub async fn update_settings(
    State(services): State<AppServices>,
    Json(request): Json<SettingsRequest>
) -> Result<ResponseJson<Vec<Setting>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseConnection(e.to_string()))?;

    let mut updated_settings = Vec::new();

    for setting_data in request.settings {
        let setting = Setting::upsert(
            &mut conn,
            &setting_data.key,
            &setting_data.value,
            &setting_data.setting_type,
            setting_data.description
        )
        .map_err(|e| AppError::DatabaseQuery(e.to_string()))?;
        
        updated_settings.push(setting);
    }

    Ok(ResponseJson(updated_settings))
}

// Get system information
pub async fn get_system_info(
    State(services): State<AppServices>
) -> Result<ResponseJson<SystemInfo>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseConnection(e.to_string()))?;

    // Get system statistics
    let total_posts: i64 = {
        use crate::schema::posts::dsl::*;
        posts.count().get_result(&mut conn).unwrap_or(0)
    };

    let total_users: i64 = {
        use crate::schema::users::dsl::*;
        users.count().get_result(&mut conn).unwrap_or(0)
    };

    let total_media: i64 = {
        use crate::schema::media::dsl::*;
        media.count().get_result(&mut conn).unwrap_or(0)
    };

    let active_sessions: i32 = {
        use crate::schema::sessions::dsl::*;
        sessions.count().get_result(&mut conn).unwrap_or(0) as i32
    };

    // Get last backup time from settings
    let last_backup = Setting::find_by_key(&mut conn, "last_backup_time")
        .ok()
        .flatten()
        .and_then(|s| s.setting_value)
        .and_then(|v| DateTime::parse_from_rfc3339(&v).ok())
        .map(|dt| dt.with_timezone(&Utc));

    // System info (simplified for now)
    let system_info = SystemInfo {
        rust_version: env!("CARGO_PKG_VERSION").to_string(),
        database_version: "PostgreSQL 14+".to_string(),
        uptime: "System uptime info".to_string(),
        memory_usage: "Memory usage info".to_string(),
        cpu_usage: "CPU usage info".to_string(),
        disk_usage: "Disk usage info".to_string(),
        active_sessions,
        total_posts,
        total_users,
        total_media,
        last_backup,
    };

    Ok(ResponseJson(system_info))
}

// Create backup with simple service (minimal database dependency)
pub async fn create_backup(
    State(services): State<AppServices>,
    Json(request): Json<BackupRequest>
) -> Result<ResponseJson<BackupInfo>, AppError> {
    // Validate backup type
    if !["database", "media", "full"].contains(&request.backup_type.as_str()) {
        return Err(AppError::BadRequest("Invalid backup type. Must be one of: database, media, full".to_string()));
    }

    // Initialize simple backup service (no database dependency for backup creation)
    let backup_dir = std::env::var("BACKUP_DIR").unwrap_or_else(|_| "./backups".to_string());
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        // Fallback database URL if not set
        "postgres://rustcms:password@localhost:5432/my_rust_cms".to_string()
    });
    
    let backup_service = SimpleBackupService::new(backup_dir, database_url);

    // Convert request to service request
    let service_request = SimpleBackupRequest {
        backup_type: request.backup_type.clone(),
        description: request.description.clone(),
    };

    // Create backup with simple service
    let backup_info = backup_service
        .create_backup_simple(service_request)
        .await
        .map_err(|e| {
            eprintln!("Backup creation error: {}", e);
            match e {
                crate::services::SimpleBackupError::ConfigurationError(msg) => {
                    AppError::Configuration(msg)
                },
                crate::services::SimpleBackupError::ValidationError(msg) => {
                    AppError::BadRequest(msg)
                },
                _ => AppError::InternalServerError(format!("Backup creation failed: {}", e))
            }
        })?;
    
    // Try to update last backup time in settings (gracefully handle connection failures)
    if let Ok(mut conn) = services.db_pool.get() {
        let timestamp = Utc::now();
        let _ = Setting::upsert(
            &mut conn,
            "last_backup_time",
            &timestamp.to_rfc3339(),
            "system",
            Some("Last backup timestamp".to_string())
        );
        println!("✅ Updated last backup time in database");
    } else {
        println!("⚠️  Could not update last backup time in database (connection failed)");
    }

    Ok(ResponseJson(backup_info))
}

// Get data snapshot with integrity check
pub async fn get_data_snapshot(
    State(services): State<AppServices>
) -> Result<ResponseJson<DataSnapshot>, AppError> {
    // Initialize backup service
    let backup_dir = std::env::var("BACKUP_DIR").unwrap_or_else(|_| "./backups".to_string());
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| AppError::Configuration("DATABASE_URL not set".to_string()))?;
    
    let backup_service = BackupService::new(backup_dir, database_url);

    // Create comprehensive data snapshot with Merkle tree integrity
    let snapshot = backup_service.create_data_snapshot(&services.db_pool).await
        .map_err(|e| AppError::InternalServerError(format!("Failed to create data snapshot: {}", e)))?;

    Ok(ResponseJson(snapshot))
}

// List available backups with simple service
pub async fn list_backups(
    State(_services): State<AppServices>
) -> Result<ResponseJson<Vec<BackupInfo>>, AppError> {
    // Initialize simple backup service
    let backup_dir = std::env::var("BACKUP_DIR").unwrap_or_else(|_| "./backups".to_string());
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| AppError::Configuration("DATABASE_URL not set".to_string()))?;
    
    let backup_service = SimpleBackupService::new(backup_dir, database_url);
    
    // List all available backups using simple service (filesystem-based)
    let backups = backup_service
        .list_backups_simple()
        .await
        .map_err(|e| {
            eprintln!("Failed to list backups: {}", e);
            AppError::InternalServerError(format!("Failed to list backups: {}", e))
        })?;
    
    Ok(ResponseJson(backups))
}

// Restore from backup
pub async fn restore_backup(
    State(services): State<AppServices>,
    Path(backup_filename): Path<String>
) -> Result<ResponseJson<String>, AppError> {
    let _conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseConnection(e.to_string()))?;

    // Initialize backup service
    let backup_dir = std::env::var("BACKUP_DIR").unwrap_or_else(|_| "./backups".to_string());
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| AppError::Configuration("DATABASE_URL not set".to_string()))?;
    
    let backup_service = BackupService::new(backup_dir, database_url);
    
    // Perform database restore
    let result = backup_service.restore_database(&backup_filename).await
        .map_err(|e| AppError::InternalServerError(format!("Database restore failed: {}", e)))?;
    
    Ok(ResponseJson(result))
}