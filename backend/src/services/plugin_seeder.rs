use crate::{
    models::{Plugin, NewPlugin},
    services::sample_plugins::get_sample_plugins,
    database::DbPool,
};
use diesel::prelude::*;
use std::sync::Arc;

/// Seed the database with sample plugins
pub async fn seed_sample_plugins(db_pool: Arc<DbPool>) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = db_pool.get()?;
    
    let sample_plugins = get_sample_plugins();
    
    for plugin in sample_plugins {
        let manifest = plugin.manifest();
        
        // Check if plugin already exists
        let existing = Plugin::find_by_name(&mut conn, &manifest.name);
        
        if existing.is_ok() {
            println!("Plugin '{}' already exists, skipping...", manifest.name);
            continue;
        }
        
        // Convert capabilities to JSON array
        let capabilities_json = serde_json::Value::Array(
            manifest.capabilities.iter()
                .map(|cap| match cap {
                    crate::services::plugin_interface::PluginCapability::HttpRoutes => serde_json::Value::String("http_routes".to_string()),
                    crate::services::plugin_interface::PluginCapability::ContentFilters => serde_json::Value::String("content_filters".to_string()),
                    crate::services::plugin_interface::PluginCapability::AdminPages => serde_json::Value::String("admin_pages".to_string()),
                    crate::services::plugin_interface::PluginCapability::FrontendComponents => serde_json::Value::String("frontend_components".to_string()),
                    crate::services::plugin_interface::PluginCapability::DatabaseAccess => serde_json::Value::String("database_access".to_string()),
                    crate::services::plugin_interface::PluginCapability::EmailSending => serde_json::Value::String("email_sending".to_string()),
                    crate::services::plugin_interface::PluginCapability::FileSystemAccess => serde_json::Value::String("filesystem_access".to_string()),
                    crate::services::plugin_interface::PluginCapability::ExternalRequests => serde_json::Value::String("external_requests".to_string()),
                    crate::services::plugin_interface::PluginCapability::BackgroundTasks => serde_json::Value::String("background_tasks".to_string()),
                    crate::services::plugin_interface::PluginCapability::AuthenticationHooks => serde_json::Value::String("auth_hooks".to_string()),
                    crate::services::plugin_interface::PluginCapability::ApiEndpoints => serde_json::Value::String("api_endpoints".to_string()),
                    crate::services::plugin_interface::PluginCapability::AdminInterfaceModification => serde_json::Value::String("admin_interface_modification".to_string()),
                    crate::services::plugin_interface::PluginCapability::Custom(name) => serde_json::Value::String(format!("custom_{}", name)),
                })
                .collect()
        );
        
        // Convert dependencies to JSON array
        let dependencies_json = serde_json::Value::Array(
            manifest.dependencies.iter()
                .map(|dep| serde_json::json!({
                    "name": dep.name,
                    "version_constraint": dep.version_constraint,
                    "optional": dep.optional
                }))
                .collect()
        );
        
        let new_plugin = NewPlugin {
            name: manifest.name.clone(),
            display_name: manifest.display_name.clone(),
            description: manifest.description.clone(),
            version: manifest.version.clone(),
            author: manifest.author.clone(),
            author_email: manifest.author_email.clone(),
            homepage_url: manifest.homepage_url.clone(),
            repository_url: manifest.repository_url.clone(),
            license: manifest.license.clone(),
            status: Some("inactive".to_string()),
            is_system: Some(false),
            install_path: None,
            config_schema: manifest.config_schema.clone(),
            config_data: manifest.default_config.clone(),
            capabilities: Some(capabilities_json),
            dependencies: Some(dependencies_json),
            manifest_data: Some(serde_json::to_value(&manifest)?),
        };
        
        match Plugin::create(&mut conn, new_plugin) {
            Ok(_) => {
                println!("✅ Created sample plugin: {}", manifest.display_name);
            }
            Err(e) => {
                eprintln!("❌ Failed to create plugin '{}': {}", manifest.display_name, e);
            }
        }
    }
    
    println!("🎉 Sample plugin seeding completed!");
    Ok(())
}

/// Remove all sample plugins from the database
pub async fn remove_sample_plugins(db_pool: Arc<DbPool>) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = db_pool.get()?;
    
    let sample_plugins = get_sample_plugins();
    
    for plugin in sample_plugins {
        let manifest = plugin.manifest();
        
        if let Ok(existing_plugin) = Plugin::find_by_name(&mut conn, &manifest.name) {
            if let Some(plugin) = existing_plugin {
                match Plugin::delete(&mut conn, plugin.id) {
                    Ok(_) => {
                        println!("🗑️  Removed sample plugin: {}", manifest.display_name);
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to remove plugin '{}': {}", manifest.display_name, e);
                    }
                }
            }
        }
    }
    
    println!("🧹 Sample plugin cleanup completed!");
    Ok(())
}

