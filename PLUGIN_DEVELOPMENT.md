# Plugin Development Guide

Welcome to the Rust CMS Plugin Development Guide! This document will help you create powerful plugins to extend your CMS functionality.

## Overview

The Rust CMS plugin system allows you to extend the core functionality by creating custom plugins. Plugins can:

- Add new admin pages and interfaces
- Modify content rendering and processing
- Handle custom HTTP routes and API endpoints
- Integrate with external services
- Add background processing capabilities
- Extend authentication and authorization
- Customize the frontend experience

## Plugin Architecture

### Plugin Manifest

Every plugin must provide a manifest that describes its capabilities, dependencies, and configuration:

```rust
use rust_cms::plugin_interface::{PluginManifest, PluginCapability};

fn create_manifest() -> PluginManifest {
    PluginManifest {
        name: "my-awesome-plugin".to_string(),
        display_name: "My Awesome Plugin".to_string(),
        description: Some("This plugin does amazing things".to_string()),
        version: "1.0.0".to_string(),
        author: Some("Your Name".to_string()),
        author_email: Some("you@example.com".to_string()),
        homepage_url: Some("https://your-plugin-site.com".to_string()),
        repository_url: Some("https://github.com/you/my-awesome-plugin".to_string()),
        license: Some("MIT".to_string()),
        min_cms_version: Some("1.0.0".to_string()),
        max_cms_version: None,
        capabilities: vec![
            PluginCapability::AdminPages,
            PluginCapability::ContentFilters,
            PluginCapability::ApiEndpoints,
        ],
        dependencies: vec![],
        config_schema: Some(serde_json::json!({
            "type": "object",
            "properties": {
                "api_key": {
                    "type": "string",
                    "description": "API key for external service"
                },
                "enabled": {
                    "type": "boolean",
                    "default": true
                }
            },
            "required": ["api_key"]
        })),
        default_config: Some(serde_json::json!({
            "enabled": true
        })),
    }
}
```

### Plugin Implementation

Implement the `Plugin` trait for your plugin:

```rust
use async_trait::async_trait;
use rust_cms::plugin_interface::{Plugin, PluginContext, PluginEvent, PluginError, PluginHealthStatus};

pub struct MyAwesomePlugin {
    context: Option<PluginContext>,
    api_client: Option<ApiClient>,
}

impl MyAwesomePlugin {
    pub fn new() -> Self {
        Self {
            context: None,
            api_client: None,
        }
    }
}

#[async_trait]
impl Plugin for MyAwesomePlugin {
    fn manifest(&self) -> PluginManifest {
        create_manifest()
    }
    
    async fn initialize(&mut self, context: PluginContext) -> Result<(), PluginError> {
        context.logger.info("Initializing My Awesome Plugin");
        
        // Extract API key from config
        let api_key = context.config
            .get("api_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| PluginError::ConfigError("Missing api_key".to_string()))?;
        
        // Initialize API client
        self.api_client = Some(ApiClient::new(api_key));
        self.context = Some(context);
        
        Ok(())
    }
    
    async fn handle_event(&mut self, event: PluginEvent) -> Result<(), PluginError> {
        if let Some(ref context) = self.context {
            match event {
                PluginEvent::Activate => {
                    context.logger.info("Plugin activated");
                    // Perform activation tasks
                }
                PluginEvent::Deactivate => {
                    context.logger.info("Plugin deactivated");
                    // Clean up active resources
                }
                PluginEvent::ConfigUpdate => {
                    context.logger.info("Configuration updated");
                    // Reload configuration
                }
                _ => {}
            }
        }
        Ok(())
    }
    
    fn validate_config(&self, config: &serde_json::Value) -> Result<(), PluginError> {
        // Validate required fields
        if !config.get("api_key").and_then(|v| v.as_str()).map_or(false, |s| !s.is_empty()) {
            return Err(PluginError::ConfigError("api_key is required and cannot be empty".to_string()));
        }
        
        Ok(())
    }
    
    async fn health_check(&self) -> Result<PluginHealthStatus, PluginError> {
        if let Some(ref api_client) = self.api_client {
            match api_client.ping().await {
                Ok(_) => Ok(PluginHealthStatus::Healthy),
                Err(e) => Ok(PluginHealthStatus::Warning(format!("API connection issue: {}", e))),
            }
        } else {
            Ok(PluginHealthStatus::Unhealthy("Plugin not properly initialized".to_string()))
        }
    }
}
```

## Plugin Capabilities

### Admin Pages

Add custom admin pages to the CMS interface:

```rust
use rust_cms::plugin_interface::PluginCapability;

// In your manifest
capabilities: vec![PluginCapability::AdminPages],

// Implementation would register admin routes
// This is a conceptual example - actual implementation may vary
impl AdminPageProvider for MyAwesomePlugin {
    fn admin_routes(&self) -> Vec<AdminRoute> {
        vec![
            AdminRoute {
                path: "/admin/my-plugin".to_string(),
                title: "My Plugin Settings".to_string(),
                component: MyPluginAdminComponent,
            }
        ]
    }
}
```

### Content Filters

Modify content before or after rendering:

```rust
use rust_cms::plugin_interface::{Hook, HookResult};

pub struct ContentFilterHook;

#[async_trait]
impl Hook for ContentFilterHook {
    async fn execute<T>(&self, content: T) -> Result<HookResult<T>, PluginError>
    where
        T: Clone + Send + Sync
    {
        // Modify content here
        // For example, add custom shortcodes, modify HTML, etc.
        Ok(HookResult::Continue(content))
    }
}
```

### API Endpoints

Add custom API endpoints:

```rust
use axum::{Router, routing::get};

impl ApiEndpointProvider for MyAwesomePlugin {
    fn api_routes(&self) -> Router {
        Router::new()
            .route("/api/my-plugin/status", get(get_plugin_status))
            .route("/api/my-plugin/data", get(get_plugin_data))
    }
}

async fn get_plugin_status() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "version": "1.0.0"
    }))
}
```

## Configuration Schema

Define a JSON schema for your plugin configuration:

```json
{
  "type": "object",
  "properties": {
    "enabled": {
      "type": "boolean",
      "default": true,
      "description": "Enable or disable the plugin"
    },
    "api_endpoint": {
      "type": "string",
      "format": "uri",
      "description": "External API endpoint URL"
    },
    "cache_duration": {
      "type": "integer",
      "minimum": 0,
      "maximum": 3600,
      "default": 300,
      "description": "Cache duration in seconds"
    },
    "features": {
      "type": "array",
      "items": {
        "type": "string",
        "enum": ["feature1", "feature2", "feature3"]
      },
      "description": "Enabled features"
    }
  },
  "required": ["api_endpoint"]
}
```

## Plugin Directory Structure

Organize your plugin files in a clear structure:

```
my-awesome-plugin/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── plugin.rs
│   ├── api/
│   │   ├── mod.rs
│   │   └── handlers.rs
│   ├── admin/
│   │   ├── mod.rs
│   │   └── components.rs
│   └── utils/
│       ├── mod.rs
│       └── helpers.rs
├── assets/
│   ├── styles.css
│   └── scripts.js
├── templates/
│   └── admin.html
├── README.md
└── plugin.json
```

## Plugin Lifecycle

Understanding the plugin lifecycle helps you implement proper initialization and cleanup:

1. **Installation**: Plugin files are copied to the plugins directory
2. **Registration**: Plugin manifest is read and validated
3. **Initialization**: Plugin is instantiated and `initialize()` is called
4. **Activation**: Plugin receives `PluginEvent::Activate`
5. **Runtime**: Plugin handles hooks and events
6. **Deactivation**: Plugin receives `PluginEvent::Deactivate`
7. **Shutdown**: Plugin `shutdown()` method is called
8. **Uninstallation**: Plugin files are removed

## Best Practices

### Security

- Always validate and sanitize user inputs
- Use the provided configuration schema for validation
- Follow the principle of least privilege
- Never store sensitive data in plain text

### Performance

- Use async/await for I/O operations
- Implement proper caching where appropriate
- Avoid blocking operations in hooks
- Monitor resource usage

### Error Handling

- Use the provided `PluginError` types
- Provide meaningful error messages
- Implement proper logging
- Handle edge cases gracefully

### Configuration

- Provide sensible defaults
- Use JSON schema for validation
- Document all configuration options
- Support configuration hot-reloading

## Testing Your Plugin

Create comprehensive tests for your plugin:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rust_cms::plugin_interface::*;
    
    #[tokio::test]
    async fn test_plugin_initialization() {
        let mut plugin = MyAwesomePlugin::new();
        let context = create_test_context();
        
        assert!(plugin.initialize(context).await.is_ok());
    }
    
    #[tokio::test]
    async fn test_config_validation() {
        let plugin = MyAwesomePlugin::new();
        
        // Valid config
        let valid_config = serde_json::json!({
            "api_key": "test-key",
            "enabled": true
        });
        assert!(plugin.validate_config(&valid_config).is_ok());
        
        // Invalid config
        let invalid_config = serde_json::json!({
            "enabled": true
        });
        assert!(plugin.validate_config(&invalid_config).is_err());
    }
}
```

## Deployment

### Development

1. Create your plugin in the `plugins/` directory
2. Add it to the plugin registry during development
3. Test thoroughly in development environment

### Production

1. Build your plugin as a dynamic library or package
2. Upload through the admin interface
3. Configure and activate through the plugin manager
4. Monitor logs and health status

## Example Plugins

### Simple Content Filter

```rust
pub struct MarkdownPlugin {
    context: Option<PluginContext>,
}

#[async_trait]
impl Plugin for MarkdownPlugin {
    fn manifest(&self) -> PluginManifest {
        PluginManifest {
            name: "markdown-processor".to_string(),
            display_name: "Markdown Processor".to_string(),
            description: Some("Converts markdown to HTML".to_string()),
            version: "1.0.0".to_string(),
            capabilities: vec![PluginCapability::ContentFilters],
            // ... other fields
        }
    }
    
    async fn initialize(&mut self, context: PluginContext) -> Result<(), PluginError> {
        self.context = Some(context);
        Ok(())
    }
}
```

### Admin Dashboard Widget

```rust
pub struct AnalyticsPlugin {
    context: Option<PluginContext>,
    analytics_client: Option<AnalyticsClient>,
}

#[async_trait]
impl Plugin for AnalyticsPlugin {
    fn manifest(&self) -> PluginManifest {
        PluginManifest {
            name: "analytics-dashboard".to_string(),
            display_name: "Analytics Dashboard".to_string(),
            description: Some("Displays website analytics in admin dashboard".to_string()),
            version: "1.0.0".to_string(),
            capabilities: vec![
                PluginCapability::AdminPages,
                PluginCapability::ApiEndpoints,
                PluginCapability::ExternalRequests,
            ],
            // ... other fields
        }
    }
    
    // ... implementation
}
```

## Getting Help

- Check the [API documentation](./docs/api.md)
- Browse [example plugins](./examples/plugins/)
- Join our [community forum](https://community.rustcms.dev)
- Report issues on [GitHub](https://github.com/rustcms/rustcms/issues)

## Contributing

We welcome plugin contributions! Please:

1. Follow the coding standards
2. Include comprehensive tests
3. Document your plugin thoroughly
4. Submit a pull request with your plugin

Happy plugin development! 🦀
