use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use gloo_net::http::Request;
use log::{info, error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug)]
pub enum UserServiceError {
    NetworkError(String),
    SerializationError(String),
    DeserializationError(String),
    NotFound,
    Unauthorized,
    ApiError(u16),
}

impl From<gloo_net::Error> for UserServiceError {
    fn from(error: gloo_net::Error) -> Self {
        UserServiceError::NetworkError(format!("Network error: {:?}", error))
    }
}

impl From<serde_json::Error> for UserServiceError {
    fn from(error: serde_json::Error) -> Self {
        UserServiceError::DeserializationError(format!("JSON error: {:?}", error))
    }
}

impl From<String> for UserServiceError {
    fn from(error: String) -> Self {
        UserServiceError::SerializationError(error)
    }
}

// Handle API responses with specific status checks
async fn handle_api_response<T>(response: gloo_net::http::Response) -> Result<T, UserServiceError>
where
    T: for<'de> Deserialize<'de>,
{
    match response.status() {
        200 => response.json::<T>().await.map_err(UserServiceError::from),
        401 => {
            error!("Unauthorized access");
            Err(UserServiceError::Unauthorized)
        }
        404 => {
            error!("Resource not found");
            Err(UserServiceError::NotFound)
        }
        _ => Err(UserServiceError::ApiError(response.status())),
    }
}

// Fetch all users
pub async fn fetch_users() -> Result<Vec<User>, UserServiceError> {
    info!("Fetching all users...");
    let response = Request::get("/api/users")
        .send()
        .await?;

    handle_api_response::<Vec<User>>(response).await
}

// Fetch user by ID
pub async fn fetch_user_by_id(user_id: i32) -> Result<User, UserServiceError> {
    info!("Fetching user with ID: {}", user_id);
    let url = format!("/api/users/{}", user_id);

    let response = Request::get(&url)
        .send()
        .await?;

    handle_api_response::<User>(response).await
}

// Create a new user
pub async fn create_user(new_user: &User) -> Result<User, UserServiceError> {
    info!("Creating new user: {:?}", new_user);
    let body = serde_json::to_string(new_user).map_err(UserServiceError::from)?;

    let response = Request::post("/api/users")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    handle_api_response::<User>(response).await
}

// Update an existing user
pub async fn update_user(user_id: i32, updated_user: &User) -> Result<User, UserServiceError> {
    info!("Updating user ID {}: {:?}", user_id, updated_user);
    let url = format!("/api/users/{}", user_id);
    let body = serde_json::to_string(updated_user).map_err(UserServiceError::from)?;

    let response = Request::put(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    handle_api_response::<User>(response).await
}

// Delete a user
pub async fn delete_user(user_id: i32) -> Result<(), UserServiceError> {
    info!("Deleting user with ID {}", user_id);
    let url = format!("/api/users/{}", user_id);

    let response = Request::delete(&url)
        .send()
        .await?;

    if response.ok() {
        Ok(())
    } else {
        error!("Failed to delete user: {:?}", response.status());
        Err(UserServiceError::ApiError(response.status()))
    }
}
