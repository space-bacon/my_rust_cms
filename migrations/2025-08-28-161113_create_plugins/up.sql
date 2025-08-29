-- Create plugins table for managing CMS plugins
CREATE TABLE plugins (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    display_name VARCHAR NOT NULL,
    description TEXT,
    version VARCHAR NOT NULL,
    author VARCHAR,
    author_email VARCHAR,
    homepage_url VARCHAR,
    repository_url VARCHAR,
    license VARCHAR,
    
    -- Plugin status and management
    status VARCHAR NOT NULL DEFAULT 'inactive', -- active, inactive, error
    is_system BOOLEAN NOT NULL DEFAULT FALSE,   -- System plugins cannot be deleted
    install_path VARCHAR,                       -- Path where plugin files are stored
    
    -- Plugin configuration and metadata
    config_schema JSONB,                        -- JSON schema for plugin configuration
    config_data JSONB DEFAULT '{}',             -- Current plugin configuration
    capabilities JSONB DEFAULT '[]',            -- What the plugin can do (hooks, features)
    dependencies JSONB DEFAULT '[]',            -- Other plugins this depends on
    
    -- Installation and update tracking
    installed_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    last_activated_at TIMESTAMP,
    activation_count INTEGER DEFAULT 0,
    
    -- Error tracking
    last_error TEXT,
    error_count INTEGER DEFAULT 0,
    
    -- Plugin manifest data
    manifest_data JSONB,                        -- Full plugin manifest/metadata
    
    -- Constraints
    CONSTRAINT plugins_status_check CHECK (status IN ('active', 'inactive', 'error', 'installing', 'updating'))
);

-- Create indexes for better performance
CREATE INDEX idx_plugins_status ON plugins(status);
CREATE INDEX idx_plugins_name ON plugins(name);
CREATE INDEX idx_plugins_is_system ON plugins(is_system);
CREATE INDEX idx_plugins_capabilities ON plugins USING GIN(capabilities);

-- Create plugin hooks table for managing plugin hook registrations
CREATE TABLE plugin_hooks (
    id SERIAL PRIMARY KEY,
    plugin_id INTEGER NOT NULL REFERENCES plugins(id) ON DELETE CASCADE,
    hook_name VARCHAR NOT NULL,              -- e.g., 'before_post_save', 'after_user_login'
    priority INTEGER DEFAULT 10,            -- Lower numbers = higher priority
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    
    UNIQUE(plugin_id, hook_name)
);

CREATE INDEX idx_plugin_hooks_name ON plugin_hooks(hook_name);
CREATE INDEX idx_plugin_hooks_priority ON plugin_hooks(priority);
CREATE INDEX idx_plugin_hooks_active ON plugin_hooks(is_active);

-- Create plugin settings table for plugin-specific settings
CREATE TABLE plugin_settings (
    id SERIAL PRIMARY KEY,
    plugin_id INTEGER NOT NULL REFERENCES plugins(id) ON DELETE CASCADE,
    setting_key VARCHAR NOT NULL,
    setting_value JSONB,
    setting_type VARCHAR DEFAULT 'string',   -- string, number, boolean, object, array
    is_encrypted BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    
    UNIQUE(plugin_id, setting_key)
);

CREATE INDEX idx_plugin_settings_key ON plugin_settings(setting_key);
CREATE INDEX idx_plugin_settings_plugin ON plugin_settings(plugin_id);