use axum::{
    extract::{State, Path, Json, Extension, Multipart},
    response::Json as ResponseJson,
    http::StatusCode,
};
use diesel::{SelectableHelper, RunQueryDsl};
use serde::{Deserialize, Serialize};
use crate::{
    AppServices,
    models::{Plugin, NewPlugin, UpdatePlugin, PluginInfo},
    middleware::{
        auth::AuthenticatedUser,
        errors::AppError,
    },
    services::{plugin_seeder, plugin_zip_handler::PluginZipHandler},
};

#[derive(Deserialize)]
pub struct CreatePluginRequest {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub version: String,
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub homepage_url: Option<String>,
    pub repository_url: Option<String>,
    pub license: Option<String>,
    pub capabilities: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
    pub config_schema: Option<serde_json::Value>,
    pub config_data: Option<serde_json::Value>,
    pub manifest_data: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct UpdatePluginRequest {
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub homepage_url: Option<String>,
    pub repository_url: Option<String>,
    pub license: Option<String>,
    pub status: Option<String>,
    pub config_data: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct PluginActionRequest {
    pub action: String, // "activate", "deactivate", "install", "uninstall"
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct PluginSettingRequest {
    pub setting_key: String,
    pub setting_value: serde_json::Value,
    pub setting_type: Option<String>,
    pub is_encrypted: Option<bool>,
}

#[derive(Serialize)]
pub struct PluginActionResponse {
    pub success: bool,
    pub message: String,
    pub plugin: Option<PluginInfo>,
}

/// Get all plugins (admin only)
/// 
/// Returns a list of all plugins in the system with their current status.
/// Requires admin authentication.
pub async fn get_plugins(
    Extension(_auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<PluginInfo>>, AppError> {
    let plugins = services.db_service.execute(move |conn| {
        Plugin::list(conn)
    }).await?;
    
    let plugin_infos: Vec<PluginInfo> = plugins.into_iter().map(PluginInfo::from).collect();
    Ok(ResponseJson(plugin_infos))
}

/// Seed sample plugins (admin only)
/// 
/// Populates the database with sample plugins for demonstration purposes.
/// Requires admin authentication.
pub async fn seed_sample_plugins(
    Extension(_auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    match plugin_seeder::seed_sample_plugins(services.db_pool.clone()).await {
        Ok(_) => {
            Ok(ResponseJson(serde_json::json!({
                "success": true,
                "message": "Sample plugins seeded successfully"
            })))
        }
        Err(e) => {
            Err(AppError::DatabaseError(format!("Failed to seed sample plugins: {}", e)))
        }
    }
}

/// Upload and install plugin from ZIP file (admin only)
pub async fn upload_plugin_zip(
    Extension(_auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>,
    mut multipart: Multipart,
) -> Result<ResponseJson<PluginInfo>, AppError> {
    let mut zip_data: Option<Vec<u8>> = None;

    // Process multipart form data
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        AppError::BadRequest(format!("Failed to read multipart data: {}", e))
    })? {
        if let Some(name) = field.name() {
            if name == "plugin_zip" {
                let data = field.bytes().await.map_err(|e| {
                    AppError::BadRequest(format!("Failed to read file data: {}", e))
                })?;
                zip_data = Some(data.to_vec());
                break;
            }
        }
    }

    let zip_data = zip_data.ok_or_else(|| {
        AppError::BadRequest("No plugin ZIP file provided".to_string())
    })?;

    // Create plugins directory if it doesn't exist
    let plugins_dir = std::path::PathBuf::from("plugins");
    if !plugins_dir.exists() {
        std::fs::create_dir_all(&plugins_dir).map_err(|e| {
            AppError::InternalServerError(format!("Failed to create plugins directory: {}", e))
        })?;
    }

    // Extract and validate plugin
    let zip_handler = PluginZipHandler::new(plugins_dir);
    let (manifest, _plugin_dir) = zip_handler.extract_plugin_zip(&zip_data).await.map_err(|e| {
        AppError::BadRequest(format!("Plugin ZIP error: {}", e))
    })?;

    // Create plugin from manifest
    let plugin = zip_handler.create_plugin_from_manifest(&manifest, &_plugin_dir);
    
    // Convert to NewPlugin for database insertion
    let new_plugin = NewPlugin {
        name: plugin.name,
        display_name: plugin.display_name,
        description: plugin.description,
        version: plugin.version,
        author: plugin.author,
        author_email: plugin.author_email,
        homepage_url: plugin.homepage_url,
        repository_url: plugin.repository_url,
        license: plugin.license,
        status: Some(plugin.status),
        is_system: Some(plugin.is_system),
        capabilities: plugin.capabilities,
        dependencies: plugin.dependencies,
        config_schema: plugin.config_schema,
        config_data: plugin.config_data,
        manifest_data: plugin.manifest_data,
        install_path: plugin.install_path,
    };

    // Insert plugin into database
    let mut conn = services.db_pool.get().map_err(|e| {
        AppError::DatabaseError(format!("Failed to get database connection: {}", e))
    })?;

    let created_plugin = diesel::insert_into(crate::schema::plugins::table)
        .values(&new_plugin)
        .returning(Plugin::as_returning())
        .get_result(&mut conn)
        .map_err(|e| AppError::DatabaseError(format!("Failed to create plugin: {}", e)))?;

    // Convert to PluginInfo for response
    let plugin_info = PluginInfo::from(created_plugin);

    Ok(ResponseJson(plugin_info))
}

/// Download base plugin template ZIP
pub async fn download_base_plugin() -> Result<axum::response::Response, AppError> {
    let plugins_dir = std::path::PathBuf::from("plugins");
    let zip_handler = PluginZipHandler::new(plugins_dir);
    
    let zip_data = zip_handler.generate_base_plugin_zip().map_err(|e| {
        AppError::InternalServerError(format!("Failed to generate base plugin: {}", e))
    })?;

    let response = axum::response::Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/zip")
        .header("Content-Disposition", "attachment; filename=\"base-plugin-template.zip\"")
        .header("Content-Length", zip_data.len().to_string())
        .body(axum::body::Body::from(zip_data))
        .map_err(|e| AppError::InternalServerError(format!("Failed to build response: {}", e)))?;

    Ok(response)
}

/// Get a specific plugin by ID (admin only)
/// 
/// Returns detailed information about a specific plugin.
/// Requires admin authentication.
pub async fn get_plugin(
    Extension(_auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>,
    Path(id): Path<i32>
) -> Result<ResponseJson<PluginInfo>, AppError> {
    let plugin = services.db_service.execute_optional(move |conn| {
        Plugin::find_by_id(conn, id)
    }).await?
        .ok_or_else(|| AppError::NotFound("Plugin not found".to_string()))?;
    
    Ok(ResponseJson(PluginInfo::from(plugin)))
}

/// Create a new plugin (admin only)
/// 
/// Creates a new plugin entry in the system.
/// This is typically used for registering plugins that have been manually installed.
/// Requires admin authentication.
pub async fn create_plugin(
    Extension(_auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>,
    Json(request): Json<CreatePluginRequest>
) -> Result<(StatusCode, ResponseJson<PluginInfo>), AppError> {
    // Validate input
    if request.name.trim().is_empty() {
        return Err(AppError::ValidationError("Plugin name cannot be empty".to_string()));
    }
    
    if request.display_name.trim().is_empty() {
        return Err(AppError::ValidationError("Plugin display name cannot be empty".to_string()));
    }
    
    if request.version.trim().is_empty() {
        return Err(AppError::ValidationError("Plugin version cannot be empty".to_string()));
    }

    // Check if plugin with same name already exists
    let plugin_name = request.name.clone();
    let existing_plugin = services.db_service.execute_optional(move |conn| {
        Plugin::find_by_name(conn, &plugin_name)
    }).await?;
    
    if existing_plugin.is_some() {
        return Err(AppError::ValidationError("Plugin with this name already exists".to_string()));
    }

    // Convert capabilities and dependencies to JSON
    let capabilities_json = request.capabilities
        .map(|caps| serde_json::Value::Array(
            caps.into_iter().map(serde_json::Value::String).collect()
        ));
        
    let dependencies_json = request.dependencies
        .map(|deps| serde_json::Value::Array(
            deps.into_iter().map(serde_json::Value::String).collect()
        ));

    let new_plugin = NewPlugin {
        name: request.name.trim().to_string(),
        display_name: request.display_name.trim().to_string(),
        description: request.description,
        version: request.version.trim().to_string(),
        author: request.author,
        author_email: request.author_email,
        homepage_url: request.homepage_url,
        repository_url: request.repository_url,
        license: request.license,
        status: Some("inactive".to_string()),
        is_system: Some(false),
        install_path: None,
        config_schema: request.config_schema,
        config_data: request.config_data,
        capabilities: capabilities_json,
        dependencies: dependencies_json,
        manifest_data: request.manifest_data,
    };
    
    let created_plugin = services.db_service.execute(move |conn| {
        Plugin::create(conn, new_plugin)
    }).await?;
    
    Ok((StatusCode::CREATED, ResponseJson(PluginInfo::from(created_plugin))))
}

/// Update an existing plugin (admin only)
/// 
/// Updates plugin metadata and configuration.
/// Requires admin authentication.
pub async fn update_plugin(
    Extension(_auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>,
    Path(id): Path<i32>,
    Json(request): Json<UpdatePluginRequest>
) -> Result<ResponseJson<PluginInfo>, AppError> {
    // Validate that plugin exists
    let _existing_plugin = services.db_service.execute_optional(move |conn| {
        Plugin::find_by_id(conn, id)
    }).await?
        .ok_or_else(|| AppError::NotFound("Plugin not found".to_string()))?;

    let update_plugin = UpdatePlugin {
        display_name: request.display_name,
        description: request.description,
        version: request.version,
        author: request.author,
        author_email: request.author_email,
        homepage_url: request.homepage_url,
        repository_url: request.repository_url,
        license: request.license,
        status: request.status,
        config_data: request.config_data,
        updated_at: Some(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    
    let updated_plugin = services.db_service.execute(move |conn| {
        Plugin::update(conn, id, update_plugin)
    }).await?;
    
    Ok(ResponseJson(PluginInfo::from(updated_plugin)))
}

/// Perform an action on a plugin (admin only)
/// 
/// Activates, deactivates, installs, or uninstalls a plugin.
/// Requires admin authentication.
pub async fn plugin_action(
    Extension(_auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>,
    Path(id): Path<i32>,
    Json(request): Json<PluginActionRequest>
) -> Result<ResponseJson<PluginActionResponse>, AppError> {
    // Validate that plugin exists
    let plugin = services.db_service.execute_optional(move |conn| {
        Plugin::find_by_id(conn, id)
    }).await?
        .ok_or_else(|| AppError::NotFound("Plugin not found".to_string()))?;

    let result = match request.action.as_str() {
        "activate" => {
            if plugin.status == "active" {
                return Ok(ResponseJson(PluginActionResponse {
                    success: false,
                    message: "Plugin is already active".to_string(),
                    plugin: Some(PluginInfo::from(plugin)),
                }));
            }
            
            let activated_plugin = services.db_service.execute(move |conn| {
                Plugin::activate(conn, id)
            }).await?;
            
            PluginActionResponse {
                success: true,
                message: "Plugin activated successfully".to_string(),
                plugin: Some(PluginInfo::from(activated_plugin)),
            }
        },
        "deactivate" => {
            if plugin.status == "inactive" {
                return Ok(ResponseJson(PluginActionResponse {
                    success: false,
                    message: "Plugin is already inactive".to_string(),
                    plugin: Some(PluginInfo::from(plugin)),
                }));
            }
            
            if plugin.is_system {
                return Ok(ResponseJson(PluginActionResponse {
                    success: false,
                    message: "System plugins cannot be deactivated".to_string(),
                    plugin: Some(PluginInfo::from(plugin)),
                }));
            }
            
            let deactivated_plugin = services.db_service.execute(move |conn| {
                Plugin::deactivate(conn, id)
            }).await?;
            
            PluginActionResponse {
                success: true,
                message: "Plugin deactivated successfully".to_string(),
                plugin: Some(PluginInfo::from(deactivated_plugin)),
            }
        },
        "uninstall" => {
            if plugin.is_system {
                return Ok(ResponseJson(PluginActionResponse {
                    success: false,
                    message: "System plugins cannot be uninstalled".to_string(),
                    plugin: Some(PluginInfo::from(plugin)),
                }));
            }
            
            if plugin.status == "active" {
                return Ok(ResponseJson(PluginActionResponse {
                    success: false,
                    message: "Plugin must be deactivated before uninstalling".to_string(),
                    plugin: Some(PluginInfo::from(plugin)),
                }));
            }
            
            let deleted_count = services.db_service.execute(move |conn| {
                Plugin::delete(conn, id)
            }).await?;
            
            if deleted_count > 0 {
                PluginActionResponse {
                    success: true,
                    message: "Plugin uninstalled successfully".to_string(),
                    plugin: None,
                }
            } else {
                PluginActionResponse {
                    success: false,
                    message: "Failed to uninstall plugin".to_string(),
                    plugin: Some(PluginInfo::from(plugin)),
                }
            }
        },
        _ => {
            return Err(AppError::ValidationError(format!("Unknown action: {}", request.action)));
        }
    };
    
    Ok(ResponseJson(result))
}

/// Delete a plugin (admin only)
/// 
/// Permanently removes a plugin from the system.
/// System plugins cannot be deleted.
/// Requires admin authentication.
pub async fn delete_plugin(
    Extension(_auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>,
    Path(id): Path<i32>
) -> Result<StatusCode, AppError> {
    // Validate that plugin exists and is not a system plugin
    let plugin = services.db_service.execute_optional(move |conn| {
        Plugin::find_by_id(conn, id)
    }).await?
        .ok_or_else(|| AppError::NotFound("Plugin not found".to_string()))?;

    if plugin.is_system {
        return Err(AppError::ValidationError("System plugins cannot be deleted".to_string()));
    }

    if plugin.status == "active" {
        return Err(AppError::ValidationError("Plugin must be deactivated before deletion".to_string()));
    }

    let deleted_count = services.db_service.execute(move |conn| {
        Plugin::delete(conn, id)
    }).await?;
    
    if deleted_count > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::DatabaseError("Failed to delete plugin".to_string()))
    }
}

/// Get active plugins (public endpoint for frontend)
/// 
/// Returns a list of currently active plugins.
/// This endpoint can be used by the frontend to determine available functionality.
pub async fn get_active_plugins(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<PluginInfo>>, AppError> {
    let plugins = services.db_service.execute(move |conn| {
        Plugin::get_active(conn)
    }).await?;
    
    let plugin_infos: Vec<PluginInfo> = plugins.into_iter().map(PluginInfo::from).collect();
    Ok(ResponseJson(plugin_infos))
}
