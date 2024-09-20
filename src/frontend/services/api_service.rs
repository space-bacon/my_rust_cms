// src/frontend/services/api_service.rs

use serde::{Deserialize, Serialize};
use gloo_net::http::{Request, Response};
use gloo_storage::{LocalStorage, Storage};
use log::{error, info};
use thiserror::Error;
use web_sys::window;

// Define the storage key for the auth token
const AUTH_TOKEN_KEY: &str = "auth_token";

/// Post structure
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Post {
    pub id: Option<i32>, // Made optional to handle cases where the post hasn't been saved yet
    pub title: String,
    pub category: String,
    pub content: String,
}

/// AuthData structure for login
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

/// Define custom error types for API operations
#[derive(Debug, Error)]
pub enum ApiServiceError {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Server error with status code: {0}")]
    ServerError(u16),
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("Resource not found")]
    NotFound,
    #[error("Unknown error occurred")]
    UnknownError,
}

impl From<gloo_net::Error> for ApiServiceError {
    fn from(error: gloo_net::Error) -> Self {
        ApiServiceError::NetworkError(error.to_string())
    }
}

impl From<serde_json::Error> for ApiServiceError {
    fn from(error: serde_json::Error) -> Self {
        ApiServiceError::DeserializationError(error.to_string())
    }
}

impl From<String> for ApiServiceError {
    fn from(error: String) -> Self {
        ApiServiceError::SerializationError(error)
    }
}

/// Retrieves the API base URL from a global JavaScript variable.
/// Ensure that `API_BASE_URL` is defined in your JavaScript code.
fn get_api_base_url() -> String {
    window()
        .and_then(|win| win.get("API_BASE_URL"))
        .and_then(|val| val.as_string())
        .unwrap_or_else(|| "http://localhost:8080".to_string())
}

/// Enum representing HTTP methods
enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpMethod {
    fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
        }
    }
}

/// Centralized function to make HTTP requests
async fn make_request(
    method: HttpMethod,
    endpoint: &str,
    body: Option<&str>,
) -> Result<Response, ApiServiceError> {
    let url = format!("{}{}", get_api_base_url(), endpoint);
    
    // Initialize the RequestBuilder based on the HTTP method
    let request_builder = match method {
        HttpMethod::GET => Request::get(&url),
        HttpMethod::POST => Request::post(&url),
        HttpMethod::PUT => Request::put(&url),
        HttpMethod::DELETE => Request::delete(&url),
    }
    .header("Content-Type", "application/json"); // Add Content-Type header

    // Add Authorization header if token is present
    let request_builder = if let Some(token) = get_auth_token() {
        request_builder.header("Authorization", &format!("Bearer {}", token))
    } else {
        request_builder
    };

    // Conditionally set the body and send the request
    let response = if let Some(body_str) = body {
        request_builder
            .body(body_str)
            .map_err(ApiServiceError::from)?
            .send()
            .await
            .map_err(ApiServiceError::from)?
    } else {
        request_builder
            .send()
            .await
            .map_err(ApiServiceError::from)?
    };

    // Handle response status codes
    if response.ok() {
        Ok(response)
    } else {
        match response.status() {
            401 => {
                error!("Unauthorized access");
                Err(ApiServiceError::Unauthorized)
            }
            404 => {
                error!("Resource not found");
                Err(ApiServiceError::NotFound)
            }
            status => {
                error!("Server error with status code: {}", status);
                Err(ApiServiceError::ServerError(status))
            }
        }
    }
}

/// Helper function to handle API responses
async fn handle_api_response<T>(response: Response) -> Result<T, ApiServiceError>
where
    T: for<'de> Deserialize<'de>,
{
    response
        .json::<T>()
        .await
        .map_err(ApiServiceError::from)
}

/// Helper function to get auth token from local storage
fn get_auth_token() -> Option<String> {
    LocalStorage::get::<String>(AUTH_TOKEN_KEY).ok()
}

/// Helper function to set auth token in local storage
fn set_auth_token(token: String) -> Result<(), ApiServiceError> {
    LocalStorage::set(AUTH_TOKEN_KEY, token)
        .map_err(|e| ApiServiceError::SerializationError(e.to_string()))
}

/// Helper function to remove auth token from local storage
fn remove_auth_token() -> Result<(), ApiServiceError> {
    LocalStorage::delete(AUTH_TOKEN_KEY)
        .map_err(|e| ApiServiceError::SerializationError(e.to_string()))
}

/// Fetch all posts
pub async fn get_posts() -> Result<Vec<Post>, ApiServiceError> {
    info!("Fetching all posts...");
    let response = make_request(HttpMethod::GET, "/api/posts", None::<&str>).await?;
    handle_api_response::<Vec<Post>>(response).await
}

/// Create a new post
pub async fn create_post(new_post: &Post) -> Result<Post, ApiServiceError> {
    info!("Creating new post: {:?}", new_post);
    let body = serde_json::to_string(new_post)?;
    let response = make_request(HttpMethod::POST, "/api/posts", Some(&body)).await?;
    handle_api_response::<Post>(response).await
}

/// Update an existing post
pub async fn update_post(post_id: i32, updated_post: &Post) -> Result<Post, ApiServiceError> {
    info!("Updating post ID {}: {:?}", post_id, updated_post);
    let endpoint = format!("/api/posts/{}", post_id);
    let body = serde_json::to_string(updated_post)?;
    let response = make_request(HttpMethod::PUT, &endpoint, Some(&body)).await?;
    handle_api_response::<Post>(response).await
}

/// Delete a post
pub async fn delete_post(post_id: i32) -> Result<(), ApiServiceError> {
    info!("Deleting post with ID {}", post_id);
    let endpoint = format!("/api/posts/{}", post_id);
    let response = make_request(HttpMethod::DELETE, &endpoint, None::<&str>).await?;

    if response.ok() {
        info!("Post deleted successfully!");
        Ok(())
    } else {
        error!("Failed to delete post with status code: {}", response.status());
        Err(ApiServiceError::ServerError(response.status()))
    }
}

/// Login API call
pub async fn login(auth_data: AuthData) -> Result<String, ApiServiceError> {
    info!("Attempting to log in user: {}", auth_data.username);
    let body = serde_json::to_string(&auth_data)?;
    let response = make_request(HttpMethod::POST, "/api/auth/login", Some(&body)).await?;

    if response.ok() {
        let token = response.text().await.map_err(ApiServiceError::from)?;
        info!("User logged in successfully!");
        set_auth_token(token.clone())?;
        Ok(token)
    } else {
        match response.status() {
            401 => {
                error!("Unauthorized: Invalid credentials");
                Err(ApiServiceError::Unauthorized)
            }
            status => {
                error!("Failed to log in with status code: {}", status);
                Err(ApiServiceError::ServerError(status))
            }
        }
    }
}

/// Logout API call
pub fn logout() -> Result<(), ApiServiceError> {
    info!("Logging out user...");
    remove_auth_token()
}
