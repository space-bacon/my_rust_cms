use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen::JsCast;
use crate::services::auth_service::get_auth_token;

const API_BASE_URL: &str = "http://localhost:8081/api";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginActionRequest {
    pub action: String, // "activate", "deactivate", "install", "uninstall"
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginActionResponse {
    pub success: bool,
    pub message: String,
    pub plugin: Option<PluginInfo>,
}

#[derive(Debug)]
pub enum PluginServiceError {
    NetworkError(String),
    ParseError(String),
    AuthError(String),
    ServerError(String),
}

impl std::fmt::Display for PluginServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginServiceError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            PluginServiceError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            PluginServiceError::AuthError(msg) => write!(f, "Auth error: {}", msg),
            PluginServiceError::ServerError(msg) => write!(f, "Server error: {}", msg),
        }
    }
}

impl std::error::Error for PluginServiceError {}

/// Get all plugins (admin only)
pub async fn get_plugins() -> Result<Vec<PluginInfo>, PluginServiceError> {
    let window = web_sys::window().ok_or_else(|| PluginServiceError::NetworkError("No window object".to_string()))?;
    
    let token = get_auth_token().map_err(|e| PluginServiceError::AuthError(format!("Auth error: {}", e)))?;
    
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    
    let headers = web_sys::Headers::new().map_err(|_| PluginServiceError::NetworkError("Failed to create headers".to_string()))?;
    headers.set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| PluginServiceError::NetworkError("Failed to set auth header".to_string()))?;
    headers.set("Content-Type", "application/json")
        .map_err(|_| PluginServiceError::NetworkError("Failed to set content type".to_string()))?;
    
    opts.set_headers(&headers);
    
    let request = Request::new_with_str_and_init(&format!("{}/plugins", API_BASE_URL), &opts)
        .map_err(|_| PluginServiceError::NetworkError("Failed to create request".to_string()))?;
    
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|_| PluginServiceError::NetworkError("Fetch failed".to_string()))?;
    
    let resp: Response = resp_value.dyn_into()
        .map_err(|_| PluginServiceError::NetworkError("Failed to cast response".to_string()))?;
    
    if !resp.ok() {
        return Err(PluginServiceError::NetworkError(format!("HTTP {}", resp.status())));
    }
    
    let json = JsFuture::from(resp.json().map_err(|_| PluginServiceError::NetworkError("Failed to get JSON".to_string()))?)
        .await
        .map_err(|_| PluginServiceError::ParseError("Failed to parse JSON".to_string()))?;
    
    let plugins: Vec<PluginInfo> = serde_wasm_bindgen::from_value(json)
        .map_err(|e| PluginServiceError::ParseError(format!("Deserialization failed: {:?}", e)))?;
    
    Ok(plugins)
}

/// Get a specific plugin by ID
pub async fn get_plugin(id: i32) -> Result<PluginInfo, PluginServiceError> {
    let window = web_sys::window().ok_or_else(|| PluginServiceError::NetworkError("No window object".to_string()))?;
    
    let token = get_auth_token().map_err(|e| PluginServiceError::AuthError(format!("Auth error: {}", e)))?;
    
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    
    let headers = web_sys::Headers::new().map_err(|_| PluginServiceError::NetworkError("Failed to create headers".to_string()))?;
    headers.set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| PluginServiceError::NetworkError("Failed to set auth header".to_string()))?;
    headers.set("Content-Type", "application/json")
        .map_err(|_| PluginServiceError::NetworkError("Failed to set content type".to_string()))?;
    
    opts.set_headers(&headers);
    
    let url = format!("/api/plugins/{}", id);
    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|_| PluginServiceError::NetworkError("Failed to create request".to_string()))?;
    
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|_| PluginServiceError::NetworkError("Fetch failed".to_string()))?;
    
    let resp: Response = resp_value.dyn_into()
        .map_err(|_| PluginServiceError::NetworkError("Failed to cast response".to_string()))?;
    
    if !resp.ok() {
        return Err(PluginServiceError::NetworkError(format!("HTTP {}", resp.status())));
    }
    
    let json = JsFuture::from(resp.json().map_err(|_| PluginServiceError::NetworkError("Failed to get JSON".to_string()))?)
        .await
        .map_err(|_| PluginServiceError::ParseError("Failed to parse JSON".to_string()))?;
    
    let plugin: PluginInfo = serde_wasm_bindgen::from_value(json)
        .map_err(|e| PluginServiceError::ParseError(format!("Deserialization failed: {:?}", e)))?;
    
    Ok(plugin)
}

/// Create a new plugin
pub async fn create_plugin(plugin_data: CreatePluginRequest) -> Result<PluginInfo, PluginServiceError> {
    let window = web_sys::window().ok_or_else(|| PluginServiceError::NetworkError("No window object".to_string()))?;
    
    let token = get_auth_token().map_err(|e| PluginServiceError::AuthError(format!("Auth error: {}", e)))?;
    
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    
    let headers = web_sys::Headers::new().map_err(|_| PluginServiceError::NetworkError("Failed to create headers".to_string()))?;
    headers.set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| PluginServiceError::NetworkError("Failed to set auth header".to_string()))?;
    headers.set("Content-Type", "application/json")
        .map_err(|_| PluginServiceError::NetworkError("Failed to set content type".to_string()))?;
    
    let body = serde_json::to_string(&plugin_data)
        .map_err(|e| PluginServiceError::ParseError(format!("Serialization failed: {:?}", e)))?;
    opts.set_body(&wasm_bindgen::JsValue::from_str(&body));
    opts.set_headers(&headers);
    
    let request = Request::new_with_str_and_init(&format!("{}/plugins", API_BASE_URL), &opts)
        .map_err(|_| PluginServiceError::NetworkError("Failed to create request".to_string()))?;
    
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|_| PluginServiceError::NetworkError("Fetch failed".to_string()))?;
    
    let resp: Response = resp_value.dyn_into()
        .map_err(|_| PluginServiceError::NetworkError("Failed to cast response".to_string()))?;
    
    if !resp.ok() {
        return Err(PluginServiceError::NetworkError(format!("HTTP {}", resp.status())));
    }
    
    let json = JsFuture::from(resp.json().map_err(|_| PluginServiceError::NetworkError("Failed to get JSON".to_string()))?)
        .await
        .map_err(|_| PluginServiceError::ParseError("Failed to parse JSON".to_string()))?;
    
    let plugin: PluginInfo = serde_wasm_bindgen::from_value(json)
        .map_err(|e| PluginServiceError::ParseError(format!("Deserialization failed: {:?}", e)))?;
    
    Ok(plugin)
}

/// Perform an action on a plugin (activate, deactivate, etc.)
pub async fn plugin_action(id: i32, action: &str) -> Result<PluginActionResponse, PluginServiceError> {
    let window = web_sys::window().ok_or_else(|| PluginServiceError::NetworkError("No window object".to_string()))?;
    
    let token = get_auth_token().map_err(|e| PluginServiceError::AuthError(format!("Auth error: {}", e)))?;
    
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    
    let headers = web_sys::Headers::new().map_err(|_| PluginServiceError::NetworkError("Failed to create headers".to_string()))?;
    headers.set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| PluginServiceError::NetworkError("Failed to set auth header".to_string()))?;
    headers.set("Content-Type", "application/json")
        .map_err(|_| PluginServiceError::NetworkError("Failed to set content type".to_string()))?;
    
    let action_request = PluginActionRequest {
        action: action.to_string(),
    };
    
    let body = serde_json::to_string(&action_request)
        .map_err(|e| PluginServiceError::ParseError(format!("Serialization failed: {:?}", e)))?;
    opts.set_body(&wasm_bindgen::JsValue::from_str(&body));
    opts.set_headers(&headers);
    
    let url = format!("/api/plugins/{}/action", id);
    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|_| PluginServiceError::NetworkError("Failed to create request".to_string()))?;
    
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|_| PluginServiceError::NetworkError("Fetch failed".to_string()))?;
    
    let resp: Response = resp_value.dyn_into()
        .map_err(|_| PluginServiceError::NetworkError("Failed to cast response".to_string()))?;
    
    if !resp.ok() {
        return Err(PluginServiceError::NetworkError(format!("HTTP {}", resp.status())));
    }
    
    let json = JsFuture::from(resp.json().map_err(|_| PluginServiceError::NetworkError("Failed to get JSON".to_string()))?)
        .await
        .map_err(|_| PluginServiceError::ParseError("Failed to parse JSON".to_string()))?;
    
    let response: PluginActionResponse = serde_wasm_bindgen::from_value(json)
        .map_err(|e| PluginServiceError::ParseError(format!("Deserialization failed: {:?}", e)))?;
    
    Ok(response)
}

/// Delete a plugin
pub async fn delete_plugin(id: i32) -> Result<(), PluginServiceError> {
    let window = web_sys::window().ok_or_else(|| PluginServiceError::NetworkError("No window object".to_string()))?;
    
    let token = get_auth_token().map_err(|e| PluginServiceError::AuthError(format!("Auth error: {}", e)))?;
    
    let opts = RequestInit::new();
    opts.set_method("DELETE");
    opts.set_mode(RequestMode::Cors);
    
    let headers = web_sys::Headers::new().map_err(|_| PluginServiceError::NetworkError("Failed to create headers".to_string()))?;
    headers.set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| PluginServiceError::NetworkError("Failed to set auth header".to_string()))?;
    
    opts.set_headers(&headers);
    
    let url = format!("/api/plugins/{}", id);
    let request = Request::new_with_str_and_init(&url, &opts)
        .map_err(|_| PluginServiceError::NetworkError("Failed to create request".to_string()))?;
    
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|_| PluginServiceError::NetworkError("Fetch failed".to_string()))?;
    
    let resp: Response = resp_value.dyn_into()
        .map_err(|_| PluginServiceError::NetworkError("Failed to cast response".to_string()))?;
    
    if !resp.ok() {
        return Err(PluginServiceError::NetworkError(format!("HTTP {}", resp.status())));
    }
    
    Ok(())
}

/// Get active plugins (public endpoint)
/// Upload a plugin ZIP file
pub async fn upload_plugin_zip(file: web_sys::File) -> Result<PluginInfo, PluginServiceError> {
    let window = web_sys::window().ok_or_else(|| PluginServiceError::NetworkError("No window object".to_string()))?;
    
    let token = get_auth_token().map_err(|e| PluginServiceError::AuthError(format!("Auth error: {}", e)))?;
    
    // Create FormData
    let form_data = web_sys::FormData::new()
        .map_err(|_| PluginServiceError::NetworkError("Failed to create FormData".to_string()))?;
    
    form_data.append_with_blob("plugin_zip", &file)
        .map_err(|_| PluginServiceError::NetworkError("Failed to append file to FormData".to_string()))?;
    
    let opts = web_sys::RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(web_sys::RequestMode::Cors);
    
    let headers = web_sys::Headers::new()
        .map_err(|_| PluginServiceError::NetworkError("Failed to create headers".to_string()))?;
    headers.set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| PluginServiceError::NetworkError("Failed to set auth header".to_string()))?;
    
    opts.set_headers(&headers);
    opts.set_body(&form_data);
    
    let request = web_sys::Request::new_with_str_and_init(&format!("{}/plugins/upload-zip", API_BASE_URL), &opts)
        .map_err(|_| PluginServiceError::NetworkError("Failed to create request".to_string()))?;
    
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|_| PluginServiceError::NetworkError("Fetch failed".to_string()))?;
    
    let resp: web_sys::Response = resp_value.dyn_into()
        .map_err(|_| PluginServiceError::NetworkError("Failed to cast response".to_string()))?;
    
    if resp.status() == 200 {
        let json = wasm_bindgen_futures::JsFuture::from(resp.json().map_err(|_| PluginServiceError::NetworkError("Failed to get JSON".to_string()))?)
            .await
            .map_err(|_| PluginServiceError::NetworkError("Failed to await JSON".to_string()))?;
        
        let plugin: PluginInfo = serde_wasm_bindgen::from_value(json)
            .map_err(|e| PluginServiceError::ParseError(format!("Deserialization failed: {:?}", e)))?;
        
        Ok(plugin)
    } else {
        let error_text = wasm_bindgen_futures::JsFuture::from(resp.text().map_err(|_| PluginServiceError::NetworkError("Failed to get error text".to_string()))?)
            .await
            .map_err(|_| PluginServiceError::NetworkError("Failed to await error text".to_string()))?
            .as_string()
            .unwrap_or_else(|| "Unknown error".to_string());
        Err(PluginServiceError::ServerError(format!("HTTP {} - {}", resp.status(), error_text)))
    }
}

/// Download base plugin template
pub async fn download_base_plugin_template() -> Result<web_sys::Blob, PluginServiceError> {
    let window = web_sys::window().ok_or_else(|| PluginServiceError::NetworkError("No window object".to_string()))?;
    
    let opts = web_sys::RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(web_sys::RequestMode::Cors);
    
    let request = web_sys::Request::new_with_str_and_init(&format!("{}/plugins/base-template", API_BASE_URL), &opts)
        .map_err(|_| PluginServiceError::NetworkError("Failed to create request".to_string()))?;
    
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|_| PluginServiceError::NetworkError("Fetch failed".to_string()))?;
    
    let resp: web_sys::Response = resp_value.dyn_into()
        .map_err(|_| PluginServiceError::NetworkError("Failed to cast response".to_string()))?;
    
    if resp.status() == 200 {
        let blob = wasm_bindgen_futures::JsFuture::from(resp.blob().map_err(|_| PluginServiceError::NetworkError("Failed to get blob".to_string()))?)
            .await
            .map_err(|_| PluginServiceError::NetworkError("Failed to await blob".to_string()))?;
        
        let blob: web_sys::Blob = blob.dyn_into()
            .map_err(|_| PluginServiceError::NetworkError("Failed to cast blob".to_string()))?;
        
        Ok(blob)
    } else {
        Err(PluginServiceError::ServerError(format!("HTTP {}", resp.status())))
    }
}

pub async fn get_active_plugins() -> Result<Vec<PluginInfo>, PluginServiceError> {
    let window = web_sys::window().ok_or_else(|| PluginServiceError::NetworkError("No window object".to_string()))?;
    
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    
    let request = Request::new_with_str_and_init(&format!("{}/plugins/active", API_BASE_URL), &opts)
        .map_err(|_| PluginServiceError::NetworkError("Failed to create request".to_string()))?;
    
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|_| PluginServiceError::NetworkError("Fetch failed".to_string()))?;
    
    let resp: Response = resp_value.dyn_into()
        .map_err(|_| PluginServiceError::NetworkError("Failed to cast response".to_string()))?;
    
    if !resp.ok() {
        return Err(PluginServiceError::NetworkError(format!("HTTP {}", resp.status())));
    }
    
    let json = JsFuture::from(resp.json().map_err(|_| PluginServiceError::NetworkError("Failed to get JSON".to_string()))?)
        .await
        .map_err(|_| PluginServiceError::ParseError("Failed to parse JSON".to_string()))?;
    
    let plugins: Vec<PluginInfo> = serde_wasm_bindgen::from_value(json)
        .map_err(|e| PluginServiceError::ParseError(format!("Deserialization failed: {:?}", e)))?;
    
    Ok(plugins)
}
