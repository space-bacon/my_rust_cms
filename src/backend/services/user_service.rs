use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use gloo_net::http::Request;
use log::{info, error};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Error)]
pub enum UserServiceError {
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
}

impl From<gloo_net::Error> for UserServiceError {
    fn from(error: gloo_net::Error) -> Self {
        UserServiceError::NetworkError(error.to_string())
    }
}

impl From<serde_json::Error> for UserServiceError {
    fn from(error: serde_json::Error) -> Self {
        UserServiceError::DeserializationError(error.to_string())
    }
}

impl From<String> for UserServiceError {
    fn from(error: String) -> Self {
        UserServiceError::SerializationError(error)
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = API_BASE_URL)]
    static API_BASE_URL: JsValue;
}

fn get_api_base_url() -> String {
    API_BASE_URL.as_string().unwrap_or_else(|| "".to_string())
}

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

async fn make_request(
    method: HttpMethod,
    endpoint: &str,
    body: Option<impl Into<JsValue>>,
) -> Result<gloo_net::http::Response, UserServiceError> {
    let url = format!("{}{}", get_api_base_url(), endpoint);
    let mut request = Request::new(&url).method(method.as_str());

    if let Some(body) = body {
        request = request
            .header("Content-Type", "application/json")
            .body(body);
    }

    let response = request.send().await.map_err(UserServiceError::from)?;

    if response.ok() {
        Ok(response)
    } else {
        match response.status() {
            401 => {
                error!("Unauthorized access");
                Err(UserServiceError::Unauthorized)
            }
            404 => {
                error!("Resource not found");
                Err(UserServiceError::NotFound)
            }
            status => Err(UserServiceError::ApiError(status)),
        }
    }
}

async fn handle_api_response<T>(response: gloo_net::http::Response) -> Result<T, UserServiceError>
where
    T: for<'de> Deserialize<'de>,
{
    response
        .json::<T>()
        .await
        .map_err(UserServiceError::from)
}

// Fetch all users
pub async fn fetch_users() -> Result<Vec<User>, UserServiceError> {
    info!("Fetching all users...");
    let response = make_request(HttpMethod::GET, "/api/users", None::<JsValue>).await?;
    handle_api_response::<Vec<User>>(response).await
}

// Fetch user by ID
pub async fn fetch_user_by_id(user_id: i32) -> Result<User, UserServiceError> {
    info!("Fetching user with ID: {}", user_id);
    let endpoint = format!("/api/users/{}", user_id);

    let response = make_request(HttpMethod::GET, &endpoint, None::<JsValue>).await?;
    handle_api_response::<User>(response).await
}

// Create a new user
pub async fn create_user(new_user: &User) -> Result<User, UserServiceError> {
    info!("Creating new user: {:?}", new_user);
    let body = serde_json::to_string(new_user).map_err(UserServiceError::from)?;

    let response = make_request(HttpMethod::POST, "/api/users", Some(body)).await?;
    handle_api_response::<User>(response).await
}

// Update an existing user
pub async fn update_user(user_id: i32, updated_user: &User) -> Result<User, UserServiceError> {
    info!("Updating user ID {}: {:?}", user_id, updated_user);
    let endpoint = format!("/api/users/{}", user_id);
    let body = serde_json::to_string(updated_user).map_err(UserServiceError::from)?;

    let response = make_request(HttpMethod::PUT, &endpoint, Some(body)).await?;
    handle_api_response::<User>(response).await
}

// Delete a user
pub async fn delete_user(user_id: i32) -> Result<(), UserServiceError> {
    info!("Deleting user with ID {}", user_id);
    let endpoint = format!("/api/users/{}", user_id);

    let response = make_request(HttpMethod::DELETE, &endpoint, None::<JsValue>).await?;

    if response.ok() {
        Ok(())
    } else {
        error!("Failed to delete user: {:?}", response.status());
        match response.status() {
            401 => Err(UserServiceError::Unauthorized),
            404 => Err(UserServiceError::NotFound),
            status => Err(UserServiceError::ApiError(status)),
        }
    }
}
