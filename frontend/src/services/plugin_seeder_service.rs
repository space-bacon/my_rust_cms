use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen::JsCast;
use crate::services::auth_service::get_auth_token;

const API_BASE_URL: &str = "http://localhost:8081/api";

#[derive(Debug)]
pub enum PluginSeederError {
    NetworkError(String),
    ParseError(String),
    AuthError(String),
}

impl std::fmt::Display for PluginSeederError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginSeederError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            PluginSeederError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            PluginSeederError::AuthError(msg) => write!(f, "Auth error: {}", msg),
        }
    }
}

impl std::error::Error for PluginSeederError {}

/// Seed sample plugins (admin only)
pub async fn seed_sample_plugins() -> Result<serde_json::Value, PluginSeederError> {
    let window = web_sys::window().ok_or_else(|| PluginSeederError::NetworkError("No window object".to_string()))?;
    
    let token = get_auth_token().map_err(|e| PluginSeederError::AuthError(format!("Auth error: {}", e)))?;
    
    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    
    let headers = web_sys::Headers::new().map_err(|_| PluginSeederError::NetworkError("Failed to create headers".to_string()))?;
    headers.set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| PluginSeederError::NetworkError("Failed to set auth header".to_string()))?;
    headers.set("Content-Type", "application/json")
        .map_err(|_| PluginSeederError::NetworkError("Failed to set content type".to_string()))?;
    
    opts.set_headers(&headers);
    
    let request = Request::new_with_str_and_init(&format!("{}/plugins/seed-samples", API_BASE_URL), &opts)
        .map_err(|_| PluginSeederError::NetworkError("Failed to create request".to_string()))?;
    
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|_| PluginSeederError::NetworkError("Fetch failed".to_string()))?;
    
    let resp: Response = resp_value.dyn_into()
        .map_err(|_| PluginSeederError::NetworkError("Failed to cast response".to_string()))?;
    
    if !resp.ok() {
        return Err(PluginSeederError::NetworkError(format!("HTTP {}", resp.status())));
    }
    
    let json = JsFuture::from(resp.json().map_err(|_| PluginSeederError::NetworkError("Failed to get JSON".to_string()))?)
        .await
        .map_err(|_| PluginSeederError::ParseError("Failed to parse JSON".to_string()))?;
    
    let response: serde_json::Value = serde_wasm_bindgen::from_value(json)
        .map_err(|e| PluginSeederError::ParseError(format!("Deserialization failed: {:?}", e)))?;
    
    Ok(response)
}
