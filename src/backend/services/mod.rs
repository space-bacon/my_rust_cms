// src/backend/services/mod.rs

pub mod media_service;
pub mod builder_service;
pub mod comment_service;
pub mod post_service;
pub mod user_service;

// Common imports
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use gloo_net::http::Request;
use thiserror::Error;
use log::error;

/// Expose common types and functions to be used across service modules

/// Enum representing HTTP methods
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
        }
    }
}

/// Common error type for all services
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Resource not found")]
    NotFound,
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("API error with status code: {0}")]
    ApiError(u16),
    #[error("Unknown error occurred")]
    UnknownError,
}

impl From<gloo_net::Error> for ServiceError {
    fn from(error: gloo_net::Error) -> Self {
        ServiceError::NetworkError(error.to_string())
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(error: serde_json::Error) -> Self {
        ServiceError::DeserializationError(error.to_string())
    }
}

impl From<String> for ServiceError {
    fn from(error: String) -> Self {
        ServiceError::SerializationError(error)
    }
}

/// Expose the API base URL to service modules
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = API_BASE_URL)]
    static API_BASE_URL: JsValue;
}

pub fn get_api_base_url() -> String {
    API_BASE_URL
        .as_string()
        .unwrap_or_else(|| "http://localhost:8080".to_string())
}

/// Centralized function to make HTTP requests
pub async fn make_request(
    method: HttpMethod,
    endpoint: &str,
    body: Option<impl Into<JsValue>>,
) -> Result<gloo_net::http::Response, ServiceError> {
    let url = format!("{}{}", get_api_base_url(), endpoint);
    let mut request = Request::new(&url).method(method.as_str());

    if let Some(body) = body {
        request = request
            .header("Content-Type", "application/json")
            .body(body);
    }

    let response = request.send().await.map_err(ServiceError::from)?;

    if response.ok() {
        Ok(response)
    } else {
        match response.status() {
            401 => {
                error!("Unauthorized access");
                Err(ServiceError::Unauthorized)
            }
            404 => {
                error!("Resource not found");
                Err(ServiceError::NotFound)
            }
            status => Err(ServiceError::ApiError(status)),
        }
    }
}

/// Helper function to handle API responses
pub async fn handle_api_response<T>(
    response: gloo_net::http::Response,
) -> Result<T, ServiceError>
where
    T: for<'de> serde::Deserialize<'de>,
{
    response
        .json::<T>()
        .await
        .map_err(ServiceError::from)
}
