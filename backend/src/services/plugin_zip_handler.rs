use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use walkdir::WalkDir;
use zip::ZipArchive;
use serde::{Deserialize, Serialize};
use crate::models::plugin::Plugin;

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub display_name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub author_email: Option<String>,
    pub homepage_url: Option<String>,
    pub repository_url: Option<String>,
    pub license: Option<String>,
    pub capabilities: Vec<String>,
    pub dependencies: Vec<String>,
    pub entry_point: String,
    pub config_schema: Option<serde_json::Value>,
    pub min_cms_version: Option<String>,
    pub max_cms_version: Option<String>,
    pub hooks: Option<Vec<String>>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum PluginZipError {
    InvalidZip(String),
    MissingManifest,
    InvalidManifest(String),
    IoError(std::io::Error),
    SecurityViolation(String),
    #[allow(dead_code)]
    UnsupportedVersion(String),
}

impl std::fmt::Display for PluginZipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginZipError::InvalidZip(msg) => write!(f, "Invalid ZIP file: {}", msg),
            PluginZipError::MissingManifest => write!(f, "Plugin manifest (plugin.json) not found"),
            PluginZipError::InvalidManifest(msg) => write!(f, "Invalid manifest: {}", msg),
            PluginZipError::IoError(err) => write!(f, "IO error: {}", err),
            PluginZipError::SecurityViolation(msg) => write!(f, "Security violation: {}", msg),
            PluginZipError::UnsupportedVersion(msg) => write!(f, "Unsupported version: {}", msg),
        }
    }
}

impl std::error::Error for PluginZipError {}

impl From<std::io::Error> for PluginZipError {
    fn from(err: std::io::Error) -> Self {
        PluginZipError::IoError(err)
    }
}

impl From<walkdir::Error> for PluginZipError {
    fn from(err: walkdir::Error) -> Self {
        PluginZipError::IoError(err.into())
    }
}

impl From<zip::result::ZipError> for PluginZipError {
    fn from(err: zip::result::ZipError) -> Self {
        PluginZipError::InvalidZip(err.to_string())
    }
}

pub struct PluginZipHandler {
    plugins_dir: PathBuf,
    max_file_size: u64,
    allowed_extensions: Vec<String>,
}

impl PluginZipHandler {
    pub fn new(plugins_dir: PathBuf) -> Self {
        Self {
            plugins_dir,
            max_file_size: 50 * 1024 * 1024, // 50MB
            allowed_extensions: vec![
                "rs".to_string(),
                "toml".to_string(),
                "json".to_string(),
                "md".to_string(),
                "txt".to_string(),
                "yml".to_string(),
                "yaml".to_string(),
                "css".to_string(),
                "js".to_string(),
                "html".to_string(),
                "svg".to_string(),
                "png".to_string(),
                "jpg".to_string(),
                "jpeg".to_string(),
                "gif".to_string(),
            ],
        }
    }

    /// Extract and validate a plugin ZIP file
    pub async fn extract_plugin_zip(&self, zip_data: &[u8]) -> Result<(PluginManifest, PathBuf), PluginZipError> {
        // Check file size
        if zip_data.len() as u64 > self.max_file_size {
            return Err(PluginZipError::SecurityViolation(
                format!("ZIP file too large: {} bytes (max: {} bytes)", zip_data.len(), self.max_file_size)
            ));
        }

        // Create temporary directory for extraction
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // Extract ZIP file
        let mut archive = ZipArchive::new(std::io::Cursor::new(zip_data))
            .map_err(|e| PluginZipError::InvalidZip(e.to_string()))?;

        let mut manifest_content = None;
        let mut total_extracted_size = 0u64;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)
                .map_err(|e| PluginZipError::InvalidZip(e.to_string()))?;

            // Security checks
            let file_path = file.name().to_string();
            
            // Check for directory traversal
            if file_path.contains("..") || file_path.starts_with('/') {
                return Err(PluginZipError::SecurityViolation(
                    format!("Invalid file path: {}", file_path)
                ));
            }

            // Check file extension
            if let Some(extension) = Path::new(&file_path).extension() {
                let ext_str = extension.to_string_lossy().to_lowercase();
                if !self.allowed_extensions.contains(&ext_str) {
                    return Err(PluginZipError::SecurityViolation(
                        format!("Disallowed file extension: {}", ext_str)
                    ));
                }
            }

            // Check total extracted size
            total_extracted_size += file.size();
            if total_extracted_size > self.max_file_size * 2 {
                return Err(PluginZipError::SecurityViolation(
                    "Extracted content too large".to_string()
                ));
            }

            let outpath = temp_path.join(&file_path);

            if file.is_dir() {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(parent) = outpath.parent() {
                    fs::create_dir_all(parent)?;
                }

                let mut outfile = fs::File::create(&outpath)?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer)?;
                outfile.write_all(&buffer)?;

                // Check if this is the manifest file
                if file_path == "plugin.json" {
                    manifest_content = Some(String::from_utf8_lossy(&buffer).to_string());
                }
            }
        }

        // Parse and validate manifest
        let manifest_str = manifest_content.ok_or(PluginZipError::MissingManifest)?;
        let manifest: PluginManifest = serde_json::from_str(&manifest_str)
            .map_err(|e| PluginZipError::InvalidManifest(e.to_string()))?;

        // Validate manifest
        self.validate_manifest(&manifest)?;

        // Create plugin directory
        let plugin_dir = self.plugins_dir.join(&manifest.name);
        if plugin_dir.exists() {
            fs::remove_dir_all(&plugin_dir)?;
        }
        fs::create_dir_all(&plugin_dir)?;

        // Copy extracted files to plugin directory
        self.copy_directory(temp_path, &plugin_dir)?;

        Ok((manifest, plugin_dir))
    }

    /// Validate plugin manifest
    fn validate_manifest(&self, manifest: &PluginManifest) -> Result<(), PluginZipError> {
        // Check required fields
        if manifest.name.is_empty() {
            return Err(PluginZipError::InvalidManifest("Plugin name is required".to_string()));
        }

        if manifest.version.is_empty() {
            return Err(PluginZipError::InvalidManifest("Plugin version is required".to_string()));
        }

        if manifest.entry_point.is_empty() {
            return Err(PluginZipError::InvalidManifest("Entry point is required".to_string()));
        }

        // Validate plugin name (alphanumeric, hyphens, underscores only)
        if !manifest.name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(PluginZipError::InvalidManifest(
                "Plugin name can only contain alphanumeric characters, hyphens, and underscores".to_string()
            ));
        }

        // Validate version format (basic semver check)
        let version_parts: Vec<&str> = manifest.version.split('.').collect();
        if version_parts.len() != 3 || !version_parts.iter().all(|part| part.parse::<u32>().is_ok()) {
            return Err(PluginZipError::InvalidManifest(
                "Version must be in semver format (e.g., 1.0.0)".to_string()
            ));
        }

        // Check CMS version compatibility (if specified)
        if let Some(min_version) = &manifest.min_cms_version {
            // TODO: Implement version comparison with current CMS version
            tracing::info!("Plugin requires minimum CMS version: {}", min_version);
        }

        Ok(())
    }

    /// Copy directory recursively
    fn copy_directory(&self, src: &Path, dst: &Path) -> Result<(), PluginZipError> {
        for entry in WalkDir::new(src) {
            let entry = entry?;
            let path = entry.path();
            let relative_path = path.strip_prefix(src)
                .map_err(|e| PluginZipError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
            let dest_path = dst.join(relative_path);

            if path.is_dir() {
                fs::create_dir_all(&dest_path)?;
            } else {
                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(path, &dest_path)?;
            }
        }
        Ok(())
    }

    /// Create a plugin from manifest and extracted files
    pub fn create_plugin_from_manifest(&self, manifest: &PluginManifest, plugin_dir: &Path) -> Plugin {
        Plugin {
            id: 0, // Will be set by database
            name: manifest.name.clone(),
            display_name: manifest.display_name.clone(),
            description: manifest.description.clone(),
            version: manifest.version.clone(),
            author: manifest.author.clone(),
            author_email: manifest.author_email.clone(),
            homepage_url: manifest.homepage_url.clone(),
            repository_url: manifest.repository_url.clone(),
            license: manifest.license.clone(),
            status: "inactive".to_string(),
            is_system: false,
            capabilities: Some(serde_json::to_value(&manifest.capabilities).unwrap_or_default()),
            dependencies: Some(serde_json::to_value(&manifest.dependencies).unwrap_or_default()),
            installed_at: Some(chrono::Utc::now().naive_utc()),
            updated_at: Some(chrono::Utc::now().naive_utc()),
            last_activated_at: None,
            activation_count: Some(0),
            error_count: Some(0),
            last_error: None,
            config_schema: manifest.config_schema.clone(),
            config_data: None,
            manifest_data: Some(serde_json::to_value(manifest).unwrap_or_default()),
            install_path: Some(plugin_dir.to_string_lossy().to_string()),
        }
    }

    /// Generate a base plugin template ZIP
    pub fn generate_base_plugin_zip(&self) -> Result<Vec<u8>, PluginZipError> {
        let mut zip_buffer = Vec::new();
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));

        // Add plugin.json manifest
        let manifest = PluginManifest {
            name: "my-awesome-plugin".to_string(),
            display_name: "My Awesome Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: Some("A sample plugin to get you started".to_string()),
            author: Some("Your Name".to_string()),
            author_email: Some("your.email@example.com".to_string()),
            homepage_url: Some("https://example.com/my-awesome-plugin".to_string()),
            repository_url: Some("https://github.com/yourusername/my-awesome-plugin".to_string()),
            license: Some("MIT".to_string()),
            capabilities: vec!["content_filter".to_string(), "admin_menu".to_string()],
            dependencies: vec![],
            entry_point: "src/lib.rs".to_string(),
            config_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "enabled": {
                        "type": "boolean",
                        "default": true,
                        "description": "Enable or disable the plugin"
                    },
                    "message": {
                        "type": "string",
                        "default": "Hello from My Awesome Plugin!",
                        "description": "Custom message to display"
                    }
                }
            })),
            min_cms_version: Some("1.0.0".to_string()),
            max_cms_version: None,
            hooks: Some(vec!["content_render".to_string(), "admin_init".to_string()]),
            permissions: Some(vec!["read_content".to_string(), "manage_settings".to_string()]),
        };

        let manifest_json = serde_json::to_string_pretty(&manifest)
            .map_err(|e| PluginZipError::InvalidManifest(e.to_string()))?;

        zip.start_file("plugin.json", zip::write::FileOptions::default())?;
        zip.write_all(manifest_json.as_bytes())?;

        // Add README.md
        let readme_content = std::fs::read_to_string("docs/PLUGIN_TEMPLATE_README.md")
            .unwrap_or_else(|_| "# Plugin Template\n\nThis is a template for creating plugins.".to_string());
        zip.start_file("README.md", zip::write::FileOptions::default())?;
        zip.write_all(readme_content.as_bytes())?;

        // Add Cargo.toml
        let cargo_toml = r#"[package]
name = "my-awesome-plugin"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
async-trait = "0.1"

# CMS Plugin SDK (would be provided by the CMS)
# cms-plugin-sdk = { path = "../cms-plugin-sdk" }
"#;
        zip.start_file("Cargo.toml", zip::write::FileOptions::default())?;
        zip.write_all(cargo_toml.as_bytes())?;

        // Add src/lib.rs
        let lib_rs = r#"use serde::{Deserialize, Serialize};
// use cms_plugin_sdk::{Plugin, PluginResult, Context};

#[derive(Debug, Serialize, Deserialize)]
pub struct MyAwesomePlugin {
    config: PluginConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub message: String,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            message: "Hello from My Awesome Plugin!".to_string(),
        }
    }
}

impl MyAwesomePlugin {
    pub fn new() -> Self {
        Self {
            config: PluginConfig::default(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Initializing My Awesome Plugin");
        Ok(())
    }

    pub fn on_content_render(&self, content: &str) -> String {
        if self.config.enabled {
            format!("{}\n<!-- {} -->", content, self.config.message)
        } else {
            content.to_string()
        }
    }

    pub fn on_admin_init(&self) {
        if self.config.enabled {
            println!("Admin initialized with plugin: {}", self.config.message);
        }
    }
}

// Plugin factory function
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut MyAwesomePlugin {
    Box::into_raw(Box::new(MyAwesomePlugin::new()))
}

#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut MyAwesomePlugin) {
    if !plugin.is_null() {
        unsafe {
            Box::from_raw(plugin);
        }
    }
}
"#;
        zip.start_file("src/lib.rs", zip::write::FileOptions::default())?;
        zip.write_all(lib_rs.as_bytes())?;

        // Add .gitignore
        let gitignore = r#"/target/
Cargo.lock
*.swp
*.swo
*~
.DS_Store
"#;
        zip.start_file(".gitignore", zip::write::FileOptions::default())?;
        zip.write_all(gitignore.as_bytes())?;

        zip.finish()?;
        drop(zip);

        Ok(zip_buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_generate_base_plugin_zip() {
        let temp_dir = TempDir::new().unwrap();
        let handler = PluginZipHandler::new(temp_dir.path().to_path_buf());
        
        let zip_data = handler.generate_base_plugin_zip().unwrap();
        assert!(!zip_data.is_empty());
        
        // Test that we can extract the generated ZIP
        let (manifest, _) = handler.extract_plugin_zip(&zip_data).await.unwrap();
        assert_eq!(manifest.name, "my-awesome-plugin");
        assert_eq!(manifest.version, "1.0.0");
    }
}
