# Rust CMS Plugin Architecture

## Overview

The Rust CMS plugin system is designed to be secure, performant, and easy to use. It supports multiple installation methods and provides a comprehensive API for extending CMS functionality.

## Plugin Architecture

### Core Components

1. **Plugin Manifest** (`plugin.json`) - Metadata and configuration
2. **Entry Point** (`src/lib.rs`) - Main plugin code
3. **Configuration Schema** - JSON Schema for settings
4. **Hook System** - Event-driven plugin execution
5. **Security Layer** - Sandboxing and validation

### Plugin Lifecycle

```
Installation → Validation → Activation → Execution → Deactivation → Removal
```

## Plugin Manifest Format

Every plugin must include a `plugin.json` file with the following structure:

```json
{
  "name": "my-plugin",
  "display_name": "My Awesome Plugin",
  "version": "1.0.0",
  "description": "A sample plugin for demonstration",
  "author": "Your Name",
  "author_email": "your.email@example.com",
  "homepage_url": "https://example.com/plugin",
  "repository_url": "https://github.com/user/plugin",
  "license": "MIT",
  "capabilities": ["content_filter", "admin_menu"],
  "dependencies": [],
  "entry_point": "src/lib.rs",
  "config_schema": {
    "type": "object",
    "properties": {
      "enabled": {
        "type": "boolean",
        "default": true
      }
    }
  },
  "min_cms_version": "1.0.0",
  "hooks": ["content_render", "admin_init"],
  "permissions": ["read_content", "manage_settings"]
}
```

### Required Fields

- `name`: Unique plugin identifier (alphanumeric, hyphens, underscores)
- `display_name`: Human-readable plugin name
- `version`: Semantic version (e.g., "1.0.0")
- `entry_point`: Path to main plugin file

### Optional Fields

- `description`: Plugin description
- `author`: Plugin author name
- `author_email`: Author email address
- `homepage_url`: Plugin homepage URL
- `repository_url`: Source code repository URL
- `license`: License identifier (e.g., "MIT", "GPL-3.0")
- `capabilities`: Array of plugin capabilities
- `dependencies`: Array of required plugins
- `config_schema`: JSON Schema for configuration
- `min_cms_version`: Minimum required CMS version
- `max_cms_version`: Maximum supported CMS version
- `hooks`: Array of hook names the plugin uses
- `permissions`: Array of required permissions

## Plugin Capabilities

Capabilities define what your plugin can do:

### Content Management
- `content_filter`: Modify content before display
- `content_save`: Process content before saving
- `content_delete`: Handle content deletion
- `media_handler`: Process media files

### Administration
- `admin_menu`: Add admin menu items
- `admin_dashboard`: Add dashboard widgets
- `user_management`: Manage users
- `settings_page`: Add settings pages

### API & Integration
- `api_endpoint`: Provide custom API endpoints
- `webhook_handler`: Handle webhook events
- `external_service`: Integrate with external services

### Theming & UI
- `theme_modifier`: Modify themes and templates
- `css_injection`: Inject custom CSS
- `js_injection`: Inject custom JavaScript

### Database & Storage
- `database_access`: Direct database access
- `cache_management`: Manage caching
- `file_storage`: Handle file operations

## Hook System

Plugins can register hooks to respond to CMS events:

### Content Hooks
- `content_render`: Before content is rendered
- `content_save`: Before content is saved
- `content_delete`: Before content is deleted
- `content_list`: When content list is requested

### User Hooks
- `user_login`: After user login
- `user_logout`: After user logout
- `user_register`: After user registration
- `user_profile_update`: After profile update

### Admin Hooks
- `admin_init`: Admin panel initialization
- `admin_menu`: Admin menu generation
- `admin_dashboard`: Dashboard rendering
- `admin_settings`: Settings page rendering

### System Hooks
- `system_startup`: CMS startup
- `system_shutdown`: CMS shutdown
- `plugin_activate`: Plugin activation
- `plugin_deactivate`: Plugin deactivation

## Plugin Development

### Basic Plugin Structure

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyPlugin {
    config: PluginConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub custom_setting: String,
}

impl MyPlugin {
    pub fn new() -> Self {
        Self {
            config: PluginConfig {
                enabled: true,
                custom_setting: "default".to_string(),
            },
        }
    }

    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Plugin initialization logic
        Ok(())
    }

    // Hook handlers
    pub fn on_content_render(&self, content: &str) -> String {
        if self.config.enabled {
            // Modify content
            format!("{}\n<!-- Modified by MyPlugin -->", content)
        } else {
            content.to_string()
        }
    }
}

// Plugin factory functions
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut MyPlugin {
    Box::into_raw(Box::new(MyPlugin::new()))
}

#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut MyPlugin) {
    if !plugin.is_null() {
        unsafe {
            Box::from_raw(plugin);
        }
    }
}
```

### Configuration Schema

Use JSON Schema to define plugin configuration:

```json
{
  "type": "object",
  "properties": {
    "enabled": {
      "type": "boolean",
      "default": true,
      "description": "Enable or disable the plugin"
    },
    "api_key": {
      "type": "string",
      "description": "API key for external service",
      "minLength": 10
    },
    "refresh_interval": {
      "type": "integer",
      "minimum": 60,
      "maximum": 3600,
      "default": 300,
      "description": "Refresh interval in seconds"
    }
  },
  "required": ["enabled"]
}
```

## Security Model

### Sandboxing
- Plugins run in isolated environments
- Limited file system access
- Network access restrictions
- Memory and CPU limits

### Validation
- Code signing verification
- Manifest validation
- Dependency checking
- Permission verification

### Permissions
- `read_content`: Read content data
- `write_content`: Modify content
- `read_users`: Access user information
- `manage_users`: Create/modify users
- `read_settings`: Access system settings
- `manage_settings`: Modify system settings
- `file_access`: File system access
- `network_access`: Network requests
- `database_access`: Direct database access

## Installation Methods

### 1. ZIP Upload
- Upload plugin ZIP file through admin interface
- Automatic extraction and validation
- Secure file type checking
- Size limits and security scanning

### 2. Git Repository
- Clone from public Git repositories
- Automatic dependency resolution
- Version management
- Update notifications

### 3. Manual Entry
- Direct plugin information entry
- For simple plugins or testing
- Immediate activation option

## Plugin Directory Structure

```
my-awesome-plugin/
├── plugin.json          # Plugin manifest
├── Cargo.toml           # Rust project file
├── src/
│   ├── lib.rs          # Main plugin code
│   ├── config.rs       # Configuration handling
│   └── hooks.rs        # Hook implementations
├── assets/             # Static assets
│   ├── styles.css
│   └── scripts.js
├── templates/          # Template files
│   └── admin.html
├── tests/              # Unit tests
│   └── lib.rs
├── README.md           # Documentation
└── LICENSE             # License file
```

## API Reference

### Plugin Context
```rust
pub struct PluginContext {
    pub config: serde_json::Value,
    pub cms_version: String,
    pub data_dir: std::path::PathBuf,
    pub temp_dir: std::path::PathBuf,
    pub logger: PluginLogger,
}
```

### Plugin Logger
```rust
impl PluginLogger {
    pub fn info(&self, message: &str);
    pub fn warn(&self, message: &str);
    pub fn error(&self, message: &str);
    pub fn debug(&self, message: &str);
}
```

### Hook Results
```rust
pub enum HookResult<T> {
    Continue(T),
    Stop(T),
    Error(String),
}
```

## Best Practices

### Development
1. **Error Handling**: Always handle errors gracefully
2. **Performance**: Keep plugin code efficient
3. **Security**: Validate all inputs and outputs
4. **Testing**: Write comprehensive tests
5. **Documentation**: Document all public APIs

### Configuration
1. **Defaults**: Provide sensible default values
2. **Validation**: Validate configuration on load
3. **Schema**: Use JSON Schema for validation
4. **Migration**: Handle configuration upgrades

### Deployment
1. **Versioning**: Use semantic versioning
2. **Dependencies**: Minimize external dependencies
3. **Compatibility**: Test with multiple CMS versions
4. **Rollback**: Support plugin deactivation

## Troubleshooting

### Common Issues

**Plugin won't activate:**
- Check plugin.json syntax
- Verify all required fields
- Ensure entry point exists
- Check CMS version compatibility

**Configuration not working:**
- Validate JSON schema
- Check default values
- Verify field types
- Test configuration UI

**Hooks not firing:**
- Verify hook names in manifest
- Check capability declarations
- Ensure plugin is activated
- Review hook registration

**Performance issues:**
- Profile plugin code
- Check for memory leaks
- Optimize database queries
- Use caching where appropriate

## Plugin Marketplace

### Publishing
1. Create plugin repository
2. Add comprehensive documentation
3. Include example configurations
4. Test with multiple CMS versions
5. Submit to plugin directory

### Discovery
- Browse plugin categories
- Search by functionality
- Read user reviews
- Check compatibility
- View installation statistics

## Future Enhancements

### Planned Features
- Hot-reloading during development
- Plugin dependency management
- Automatic updates
- Plugin marketplace integration
- Advanced sandboxing
- Plugin analytics
- Multi-language support

### SDK Development
- Plugin development SDK
- Code generation tools
- Testing frameworks
- Documentation generators
- Deployment automation

---

For more information, see:
- [Plugin Development Guide](PLUGIN_DEVELOPMENT.md)
- [API Documentation](API_REFERENCE.md)
- [Security Guidelines](SECURITY.md)
- [Examples Repository](https://github.com/rust-cms/plugin-examples)
