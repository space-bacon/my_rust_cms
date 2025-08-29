use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::{plugins, plugin_hooks, plugin_settings};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = plugins)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Plugin {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub version: String,
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub homepage_url: Option<String>,
    pub repository_url: Option<String>,
    pub license: Option<String>,
    pub status: String,
    pub is_system: bool,
    pub install_path: Option<String>,
    pub config_schema: Option<serde_json::Value>,
    pub config_data: Option<serde_json::Value>,
    pub capabilities: Option<serde_json::Value>,
    pub dependencies: Option<serde_json::Value>,
    pub installed_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub last_activated_at: Option<NaiveDateTime>,
    pub activation_count: Option<i32>,
    pub last_error: Option<String>,
    pub error_count: Option<i32>,
    pub manifest_data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = plugins)]
pub struct NewPlugin {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub version: String,
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub homepage_url: Option<String>,
    pub repository_url: Option<String>,
    pub license: Option<String>,
    pub status: Option<String>,
    pub is_system: Option<bool>,
    pub install_path: Option<String>,
    pub config_schema: Option<serde_json::Value>,
    pub config_data: Option<serde_json::Value>,
    pub capabilities: Option<serde_json::Value>,
    pub dependencies: Option<serde_json::Value>,
    pub manifest_data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = plugins)]
pub struct UpdatePlugin {
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub homepage_url: Option<String>,
    pub repository_url: Option<String>,
    pub license: Option<String>,
    pub status: Option<String>,
    pub install_path: Option<String>,
    pub config_schema: Option<serde_json::Value>,
    pub config_data: Option<serde_json::Value>,
    pub capabilities: Option<serde_json::Value>,
    pub dependencies: Option<serde_json::Value>,
    pub updated_at: Option<NaiveDateTime>,
    pub last_activated_at: Option<NaiveDateTime>,
    pub activation_count: Option<i32>,
    pub last_error: Option<String>,
    pub error_count: Option<i32>,
    pub manifest_data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = plugin_hooks)]
#[diesel(belongs_to(Plugin, foreign_key = plugin_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PluginHook {
    pub id: i32,
    pub plugin_id: i32,
    pub hook_name: String,
    pub priority: Option<i32>,
    pub is_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = plugin_hooks)]
pub struct NewPluginHook {
    pub plugin_id: i32,
    pub hook_name: String,
    pub priority: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = plugin_settings)]
#[diesel(belongs_to(Plugin, foreign_key = plugin_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PluginSetting {
    pub id: i32,
    pub plugin_id: i32,
    pub setting_key: String,
    pub setting_value: Option<serde_json::Value>,
    pub setting_type: Option<String>,
    pub is_encrypted: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = plugin_settings)]
pub struct NewPluginSetting {
    pub plugin_id: i32,
    pub setting_key: String,
    pub setting_value: Option<serde_json::Value>,
    pub setting_type: Option<String>,
    pub is_encrypted: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = plugin_settings)]
pub struct UpdatePluginSetting {
    pub setting_value: Option<serde_json::Value>,
    pub setting_type: Option<String>,
    pub is_encrypted: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}

// Frontend-friendly plugin representation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginInfo {
    pub id: i32,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub version: String,
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub homepage_url: Option<String>,
    pub repository_url: Option<String>,
    pub license: Option<String>,
    pub status: String,
    pub is_system: bool,
    pub capabilities: Vec<String>,
    pub dependencies: Vec<String>,
    pub installed_at: Option<String>,
    pub updated_at: Option<String>,
    pub last_activated_at: Option<String>,
    pub activation_count: i32,
    pub has_errors: bool,
    pub last_error: Option<String>,
    pub config_schema: Option<serde_json::Value>,
    pub config_data: Option<serde_json::Value>,
}

impl From<Plugin> for PluginInfo {
    fn from(plugin: Plugin) -> Self {
        let capabilities = plugin.capabilities
            .as_ref()
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();
            
        let dependencies = plugin.dependencies
            .as_ref()
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .unwrap_or_default();

        PluginInfo {
            id: plugin.id,
            name: plugin.name,
            display_name: plugin.display_name,
            description: plugin.description,
            version: plugin.version,
            author: plugin.author,
            author_email: plugin.author_email,
            homepage_url: plugin.homepage_url,
            repository_url: plugin.repository_url,
            license: plugin.license,
            status: plugin.status,
            is_system: plugin.is_system,
            capabilities,
            dependencies,
            installed_at: plugin.installed_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            updated_at: plugin.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            last_activated_at: plugin.last_activated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            activation_count: plugin.activation_count.unwrap_or(0),
            has_errors: plugin.error_count.unwrap_or(0) > 0,
            last_error: plugin.last_error,
            config_schema: plugin.config_schema,
            config_data: plugin.config_data,
        }
    }
}

impl Plugin {
    /// Get all plugins
    pub fn list(conn: &mut PgConnection) -> QueryResult<Vec<Plugin>> {
        plugins::table
            .order(plugins::display_name.asc())
            .load::<Plugin>(conn)
    }

    /// Find plugin by ID
    pub fn find_by_id(conn: &mut PgConnection, plugin_id: i32) -> QueryResult<Option<Plugin>> {
        plugins::table
            .find(plugin_id)
            .first::<Plugin>(conn)
            .optional()
    }

    /// Find plugin by name
    pub fn find_by_name(conn: &mut PgConnection, plugin_name: &str) -> QueryResult<Option<Plugin>> {
        plugins::table
            .filter(plugins::name.eq(plugin_name))
            .first::<Plugin>(conn)
            .optional()
    }

    /// Create a new plugin
    pub fn create(conn: &mut PgConnection, new_plugin: NewPlugin) -> QueryResult<Plugin> {
        diesel::insert_into(plugins::table)
            .values(&new_plugin)
            .get_result(conn)
    }

    /// Update plugin
    pub fn update(conn: &mut PgConnection, plugin_id: i32, update_plugin: UpdatePlugin) -> QueryResult<Plugin> {
        diesel::update(plugins::table.find(plugin_id))
            .set(&update_plugin)
            .get_result(conn)
    }

    /// Delete plugin (only if not system plugin)
    pub fn delete(conn: &mut PgConnection, plugin_id: i32) -> QueryResult<usize> {
        diesel::delete(
            plugins::table
                .filter(plugins::id.eq(plugin_id))
                .filter(plugins::is_system.eq(false))
        )
        .execute(conn)
    }

    /// Get active plugins
    pub fn get_active(conn: &mut PgConnection) -> QueryResult<Vec<Plugin>> {
        plugins::table
            .filter(plugins::status.eq("active"))
            .order(plugins::display_name.asc())
            .load::<Plugin>(conn)
    }

    /// Activate plugin
    pub fn activate(conn: &mut PgConnection, plugin_id: i32) -> QueryResult<Plugin> {
        use chrono::Utc;
        
        let update_data = UpdatePlugin {
            status: Some("active".to_string()),
            last_activated_at: Some(Utc::now().naive_utc()),
            activation_count: None, // Will be handled by a separate query
            ..Default::default()
        };

        // First update the status and activation time
        let _plugin = diesel::update(plugins::table.find(plugin_id))
            .set(&update_data)
            .get_result::<Plugin>(conn)?;

        // Then increment the activation count
        diesel::update(plugins::table.find(plugin_id))
            .set(plugins::activation_count.eq(plugins::activation_count + 1))
            .execute(conn)?;

        // Return the updated plugin
        plugins::table.find(plugin_id).first(conn)
    }

    /// Deactivate plugin
    pub fn deactivate(conn: &mut PgConnection, plugin_id: i32) -> QueryResult<Plugin> {
        diesel::update(plugins::table.find(plugin_id))
            .set(plugins::status.eq("inactive"))
            .get_result(conn)
    }
}

impl Default for UpdatePlugin {
    fn default() -> Self {
        Self {
            display_name: None,
            description: None,
            version: None,
            author: None,
            author_email: None,
            homepage_url: None,
            repository_url: None,
            license: None,
            status: None,
            install_path: None,
            config_schema: None,
            config_data: None,
            capabilities: None,
            dependencies: None,
            updated_at: None,
            last_activated_at: None,
            activation_count: None,
            last_error: None,
            error_count: None,
            manifest_data: None,
        }
    }
}
