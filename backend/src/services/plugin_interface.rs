use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use async_trait::async_trait;

/// Plugin manifest structure that every plugin must provide
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Unique plugin identifier (kebab-case recommended)
    pub name: String,
    /// Human-readable display name
    pub display_name: String,
    /// Plugin description
    pub description: Option<String>,
    /// Semantic version (e.g., "1.0.0")
    pub version: String,
    /// Plugin author information
    pub author: Option<String>,
    pub author_email: Option<String>,
    /// URLs for documentation and source
    pub homepage_url: Option<String>,
    pub repository_url: Option<String>,
    /// License identifier (e.g., "MIT", "GPL-3.0")
    pub license: Option<String>,
    /// Minimum CMS version required
    pub min_cms_version: Option<String>,
    /// Maximum CMS version supported
    pub max_cms_version: Option<String>,
    /// Plugin capabilities/features
    pub capabilities: Vec<PluginCapability>,
    /// Dependencies on other plugins
    pub dependencies: Vec<PluginDependency>,
    /// Configuration schema for the plugin
    pub config_schema: Option<serde_json::Value>,
    /// Default configuration values
    pub default_config: Option<serde_json::Value>,
}

/// Plugin capabilities that can be declared
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCapability {
    /// Can handle HTTP requests at specific routes
    HttpRoutes,
    /// Can modify content before/after rendering
    ContentFilters,
    /// Can add custom admin pages
    AdminPages,
    /// Can add custom frontend components
    FrontendComponents,
    /// Can handle database operations
    DatabaseAccess,
    /// Can send emails
    EmailSending,
    /// Can access file system
    FileSystemAccess,
    /// Can make external HTTP requests
    ExternalRequests,
    /// Can schedule background tasks
    BackgroundTasks,
    /// Can modify user authentication flow
    AuthenticationHooks,
    /// Can add custom API endpoints
    ApiEndpoints,
    /// Can modify the admin interface
    AdminInterfaceModification,
    /// Custom capability (specify name)
    Custom(String),
}

/// Plugin dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    /// Name of the required plugin
    pub name: String,
    /// Version constraint (e.g., ">=1.0.0", "^2.1.0")
    pub version_constraint: Option<String>,
    /// Whether this dependency is optional
    pub optional: bool,
}

/// Plugin execution context provided to plugins
#[derive(Debug, Clone)]
pub struct PluginContext {
    /// Plugin configuration data
    pub config: serde_json::Value,
    /// CMS version
    pub cms_version: String,
    /// Plugin data directory path
    pub data_dir: std::path::PathBuf,
    /// Temporary directory for plugin use
    pub temp_dir: std::path::PathBuf,
    /// Plugin-specific logger
    pub logger: PluginLogger,
}

/// Plugin-specific logging interface
#[derive(Debug, Clone)]
pub struct PluginLogger {
    pub plugin_name: String,
}

impl PluginLogger {
    pub fn new(plugin_name: String) -> Self {
        Self { plugin_name }
    }
    
    pub fn info(&self, message: &str) {
        tracing::info!(plugin = %self.plugin_name, "{}", message);
    }
    
    pub fn warn(&self, message: &str) {
        tracing::warn!(plugin = %self.plugin_name, "{}", message);
    }
    
    pub fn error(&self, message: &str) {
        tracing::error!(plugin = %self.plugin_name, "{}", message);
    }
    
    pub fn debug(&self, message: &str) {
        tracing::debug!(plugin = %self.plugin_name, "{}", message);
    }
}

/// Hook execution result
#[derive(Debug)]
pub enum HookResult<T> {
    /// Continue with the provided value
    Continue(T),
    /// Stop processing and return the value
    Stop(T),
    /// Skip this hook and continue with original value
    Skip,
    /// Error occurred during hook execution
    Error(String),
}

/// Plugin lifecycle events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEvent {
    /// Plugin is being installed
    Install,
    /// Plugin is being activated
    Activate,
    /// Plugin is being deactivated
    Deactivate,
    /// Plugin is being uninstalled
    Uninstall,
    /// Plugin configuration is being updated
    ConfigUpdate,
    /// CMS is starting up
    CmsStartup,
    /// CMS is shutting down
    CmsShutdown,
}

/// Main plugin trait that all plugins must implement
#[async_trait]
pub trait Plugin: Send + Sync {
    /// Return the plugin manifest
    fn manifest(&self) -> PluginManifest;
    
    /// Initialize the plugin with the given context
    async fn initialize(&mut self, context: PluginContext) -> Result<(), PluginError>;
    
    /// Handle plugin lifecycle events
    async fn handle_event(&mut self, event: PluginEvent) -> Result<(), PluginError> {
        // Default implementation does nothing
        let _ = event;
        Ok(())
    }
    
    /// Validate plugin configuration
    fn validate_config(&self, config: &serde_json::Value) -> Result<(), PluginError> {
        // Default implementation accepts any config
        let _ = config;
        Ok(())
    }
    
    /// Get plugin health status
    async fn health_check(&self) -> Result<PluginHealthStatus, PluginError> {
        Ok(PluginHealthStatus::Healthy)
    }
    
    /// Clean up resources when plugin is being shut down
    async fn shutdown(&mut self) -> Result<(), PluginError> {
        Ok(())
    }
}

/// Plugin health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginHealthStatus {
    Healthy,
    Warning(String),
    Unhealthy(String),
}

/// Plugin-specific errors
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Initialization error: {0}")]
    InitializationError(String),
    
    #[error("Runtime error: {0}")]
    RuntimeError(String),
    
    #[error("Dependency error: {0}")]
    DependencyError(String),
    
    #[error("Permission error: {0}")]
    PermissionError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Hook registry for managing plugin hooks
/// Note: This is a simplified version for demonstration.
/// In a real implementation, you might use an enum dispatch or other pattern
/// to handle different hook types without generics in trait objects.
pub struct HookRegistry {
    // For now, we'll use a simpler approach without trait objects
    // In practice, you might use specific hook types or an enum dispatch pattern
}

impl HookRegistry {
    pub fn new() -> Self {
        Self {}
    }
    
    /// Register a hook for a specific event
    /// Note: This is a placeholder - actual implementation would depend on your hook system design
    pub fn register_hook(&mut self, _event_name: String) {
        // Placeholder implementation
    }
    
    /// Execute all hooks for a given event
    /// Note: This is a placeholder - actual implementation would depend on your hook system design
    pub async fn execute_hooks<T>(&self, _event_name: &str, value: T) -> Result<T, PluginError>
    where
        T: Clone + Send + Sync,
    {
        // Placeholder implementation - just return the value unchanged
        Ok(value)
    }
}

/// Hook trait for implementing plugin hooks
/// Note: Due to Rust's trait object limitations with generic methods,
/// you might need to use a different approach for hooks in practice,
/// such as specific hook traits for different data types or an enum dispatch pattern.
#[async_trait]
pub trait Hook: Send + Sync {
    /// Execute hook on string content (most common case)
    async fn execute_string(&self, value: String) -> Result<HookResult<String>, PluginError> {
        Ok(HookResult::Continue(value))
    }
    
    /// Execute hook on JSON value
    async fn execute_json(&self, value: serde_json::Value) -> Result<HookResult<serde_json::Value>, PluginError> {
        Ok(HookResult::Continue(value))
    }
}

/// Plugin manager for loading and managing plugins
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
    hook_registry: HookRegistry,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            hook_registry: HookRegistry::new(),
        }
    }
    
    /// Load a plugin from a dynamic library
    pub async fn load_plugin(&mut self, plugin_path: &std::path::Path) -> Result<(), PluginError> {
        // This would implement dynamic loading of plugins
        // For now, this is a placeholder for the interface
        let _ = plugin_path;
        Err(PluginError::RuntimeError("Dynamic loading not yet implemented".to_string()))
    }
    
    /// Register a plugin instance
    pub async fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), PluginError> {
        let manifest = plugin.manifest();
        self.plugins.insert(manifest.name.clone(), plugin);
        Ok(())
    }
    
    /// Activate a plugin by name
    pub async fn activate_plugin(&mut self, plugin_name: &str) -> Result<(), PluginError> {
        if let Some(plugin) = self.plugins.get_mut(plugin_name) {
            plugin.handle_event(PluginEvent::Activate).await?;
            Ok(())
        } else {
            Err(PluginError::RuntimeError(format!("Plugin '{}' not found", plugin_name)))
        }
    }
    
    /// Deactivate a plugin by name
    pub async fn deactivate_plugin(&mut self, plugin_name: &str) -> Result<(), PluginError> {
        if let Some(plugin) = self.plugins.get_mut(plugin_name) {
            plugin.handle_event(PluginEvent::Deactivate).await?;
            Ok(())
        } else {
            Err(PluginError::RuntimeError(format!("Plugin '{}' not found", plugin_name)))
        }
    }
    
    /// Get plugin health status
    pub async fn get_plugin_health(&self, plugin_name: &str) -> Result<PluginHealthStatus, PluginError> {
        if let Some(plugin) = self.plugins.get(plugin_name) {
            plugin.health_check().await
        } else {
            Err(PluginError::RuntimeError(format!("Plugin '{}' not found", plugin_name)))
        }
    }
    
    /// List all registered plugins
    pub fn list_plugins(&self) -> Vec<PluginManifest> {
        self.plugins.values().map(|p| p.manifest()).collect()
    }
}

/// Example plugin implementation for demonstration
pub struct ExamplePlugin {
    context: Option<PluginContext>,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        Self { context: None }
    }
}

#[async_trait]
impl Plugin for ExamplePlugin {
    fn manifest(&self) -> PluginManifest {
        PluginManifest {
            name: "example-plugin".to_string(),
            display_name: "Example Plugin".to_string(),
            description: Some("A simple example plugin for demonstration".to_string()),
            version: "1.0.0".to_string(),
            author: Some("CMS Developer".to_string()),
            author_email: Some("dev@example.com".to_string()),
            homepage_url: Some("https://example.com/plugin".to_string()),
            repository_url: Some("https://github.com/example/plugin".to_string()),
            license: Some("MIT".to_string()),
            min_cms_version: Some("1.0.0".to_string()),
            max_cms_version: None,
            capabilities: vec![
                PluginCapability::ContentFilters,
                PluginCapability::AdminPages,
            ],
            dependencies: vec![],
            config_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "enabled": {
                        "type": "boolean",
                        "default": true
                    },
                    "message": {
                        "type": "string",
                        "default": "Hello from Example Plugin!"
                    }
                }
            })),
            default_config: Some(serde_json::json!({
                "enabled": true,
                "message": "Hello from Example Plugin!"
            })),
        }
    }
    
    async fn initialize(&mut self, context: PluginContext) -> Result<(), PluginError> {
        context.logger.info("Example plugin initialized successfully");
        self.context = Some(context);
        Ok(())
    }
    
    async fn handle_event(&mut self, event: PluginEvent) -> Result<(), PluginError> {
        if let Some(ref context) = self.context {
            match event {
                PluginEvent::Activate => {
                    context.logger.info("Example plugin activated");
                }
                PluginEvent::Deactivate => {
                    context.logger.info("Example plugin deactivated");
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    fn validate_config(&self, config: &serde_json::Value) -> Result<(), PluginError> {
        // Validate that config has required fields
        if !config.is_object() {
            return Err(PluginError::ConfigError("Config must be an object".to_string()));
        }
        
        if let Some(enabled) = config.get("enabled") {
            if !enabled.is_boolean() {
                return Err(PluginError::ConfigError("'enabled' must be a boolean".to_string()));
            }
        }
        
        if let Some(message) = config.get("message") {
            if !message.is_string() {
                return Err(PluginError::ConfigError("'message' must be a string".to_string()));
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[tokio::test]
    async fn test_example_plugin() {
        let mut plugin = ExamplePlugin::new();
        let manifest = plugin.manifest();
        
        assert_eq!(manifest.name, "example-plugin");
        assert_eq!(manifest.version, "1.0.0");
        
        let context = PluginContext {
            config: serde_json::json!({"enabled": true}),
            cms_version: "1.0.0".to_string(),
            data_dir: PathBuf::from("/tmp/plugin-data"),
            temp_dir: PathBuf::from("/tmp/plugin-temp"),
            logger: PluginLogger::new("example-plugin".to_string()),
        };
        
        assert!(plugin.initialize(context).await.is_ok());
        assert!(plugin.handle_event(PluginEvent::Activate).await.is_ok());
        assert!(plugin.validate_config(&serde_json::json!({"enabled": true})).is_ok());
        assert!(plugin.validate_config(&serde_json::json!({"enabled": "invalid"})).is_err());
    }
}
