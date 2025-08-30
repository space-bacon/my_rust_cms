# My Awesome Plugin

A sample plugin template for the Rust CMS system.

## Overview

This plugin demonstrates the basic structure and capabilities of a Rust CMS plugin. It includes:

- Content filtering hooks
- Admin panel integration
- Configuration schema
- Proper plugin lifecycle management

## Features

- ✅ Content rendering hooks
- ✅ Admin initialization
- ✅ Configurable settings
- ✅ Error handling
- ✅ Proper plugin lifecycle

## Installation

1. Download this plugin as a ZIP file
2. Go to your CMS Admin Panel → Plugins
3. Click "Add Plugin" → "ZIP Upload"
4. Select this ZIP file and click "Install Plugin"
5. Activate the plugin from the plugin manager

## Configuration

The plugin supports the following configuration options:

- **enabled** (boolean): Enable or disable the plugin functionality
- **message** (string): Custom message to display in content

## Development

### Prerequisites

- Rust 1.70+
- Cargo

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

## Plugin Structure

```
my-awesome-plugin/
├── plugin.json          # Plugin manifest
├── Cargo.toml           # Rust project configuration
├── src/
│   └── lib.rs          # Main plugin code
├── README.md           # This file
└── .gitignore         # Git ignore rules
```

## Plugin Manifest (plugin.json)

The `plugin.json` file contains metadata about your plugin:

```json
{
  "name": "my-awesome-plugin",
  "display_name": "My Awesome Plugin",
  "version": "1.0.0",
  "description": "A sample plugin to get you started",
  "author": "Your Name",
  "author_email": "your.email@example.com",
  "capabilities": ["content_filter", "admin_menu"],
  "entry_point": "src/lib.rs",
  "hooks": ["content_render", "admin_init"]
}
```

## Available Hooks

### Content Hooks
- `content_render`: Modify content before rendering
- `content_save`: Process content before saving
- `content_delete`: Handle content deletion

### Admin Hooks
- `admin_init`: Initialize admin panel features
- `admin_menu`: Add custom admin menu items
- `admin_dashboard`: Add dashboard widgets

### User Hooks
- `user_login`: Handle user login events
- `user_logout`: Handle user logout events
- `user_register`: Handle user registration

## Plugin Capabilities

Declare what your plugin can do:

- `content_filter`: Modify content
- `admin_menu`: Add admin menu items
- `user_management`: Manage users
- `media_handler`: Handle media files
- `api_endpoint`: Provide API endpoints
- `theme_modifier`: Modify themes
- `database_access`: Access database directly

## Configuration Schema

Define your plugin's configuration using JSON Schema:

```json
{
  "type": "object",
  "properties": {
    "enabled": {
      "type": "boolean",
      "default": true,
      "description": "Enable or disable the plugin"
    },
    "message": {
      "type": "string",
      "default": "Hello World!",
      "description": "Custom message"
    }
  }
}
```

## Best Practices

1. **Error Handling**: Always handle errors gracefully
2. **Configuration**: Use the config schema for user settings
3. **Performance**: Keep plugin code efficient
4. **Security**: Validate all inputs and outputs
5. **Documentation**: Document your plugin thoroughly

## Troubleshooting

### Common Issues

**Plugin won't activate:**
- Check the plugin.json syntax
- Ensure all required fields are present
- Verify the entry point file exists

**Configuration not working:**
- Validate your config schema
- Check for JSON syntax errors
- Ensure default values are provided

**Hooks not firing:**
- Verify hook names in plugin.json
- Check that capabilities are declared
- Ensure plugin is activated

## Support

For help with plugin development:

1. Check the CMS documentation
2. Review other plugin examples
3. Join the community forum
4. Submit issues on GitHub

## License

This template is provided under the MIT License. You can use it as a starting point for your own plugins.

---

Happy plugin development! 🦀✨

