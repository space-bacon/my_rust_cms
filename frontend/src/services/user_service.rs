use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use gloo_storage::{LocalStorage, Storage};

use crate::services::auth_service::{get_auth_token, AuthError, User};

const API_BASE_URL: &str = "http://localhost:8081/api";

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PromoteUserRequest {
    pub role: String,
}

pub async fn get_users() -> Result<Vec<User>, AuthError> {
    let token = get_auth_token()?;
    
    let response = Request::get(&format!("{}/users", API_BASE_URL))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| AuthError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let users: Vec<User> = response
            .json()
            .await
            .map_err(|e| AuthError::ParseError(e.to_string()))?;
        Ok(users)
    } else if response.status() == 401 {
        // Clear invalid token
        LocalStorage::delete("auth_token");
        Err(AuthError::InvalidCredentials)
    } else {
        Err(AuthError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn create_user(user_data: &CreateUserRequest) -> Result<serde_json::Value, AuthError> {
    let token = get_auth_token()?;
    
    let response = Request::post(&format!("{}/users", API_BASE_URL))
        .header("Authorization", &format!("Bearer {}", token))
        .json(user_data)
        .map_err(|e| AuthError::NetworkError(e.to_string()))?
        .send()
        .await
        .map_err(|e| AuthError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AuthError::ParseError(e.to_string()))?;
        Ok(result)
    } else if response.status() == 401 {
        LocalStorage::delete("auth_token");
        Err(AuthError::InvalidCredentials)
    } else if response.status() == 409 {
        Err(AuthError::ServerError("Username or email already exists".to_string()))
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to create user".to_string());
        Err(AuthError::ServerError(error_text))
    }
}

pub async fn update_user(user_id: i32, user_data: &UpdateUserRequest) -> Result<serde_json::Value, AuthError> {
    let token = get_auth_token()?;
    
    let response = Request::put(&format!("{}/users/{}", API_BASE_URL, user_id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(user_data)
        .map_err(|e| AuthError::NetworkError(e.to_string()))?
        .send()
        .await
        .map_err(|e| AuthError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AuthError::ParseError(e.to_string()))?;
        Ok(result)
    } else if response.status() == 401 {
        LocalStorage::delete("auth_token");
        Err(AuthError::InvalidCredentials)
    } else if response.status() == 404 {
        Err(AuthError::ServerError("User not found".to_string()))
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to update user".to_string());
        Err(AuthError::ServerError(error_text))
    }
}

pub async fn promote_user(user_id: i32, role: &str) -> Result<serde_json::Value, AuthError> {
    let token = get_auth_token()?;
    
    let promote_data = PromoteUserRequest {
        role: role.to_string(),
    };
    
    let response = Request::put(&format!("{}/users/{}/promote", API_BASE_URL, user_id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(&promote_data)
        .map_err(|e| AuthError::NetworkError(e.to_string()))?
        .send()
        .await
        .map_err(|e| AuthError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AuthError::ParseError(e.to_string()))?;
        Ok(result)
    } else if response.status() == 401 {
        LocalStorage::delete("auth_token");
        Err(AuthError::InvalidCredentials)
    } else if response.status() == 404 {
        Err(AuthError::ServerError("User not found".to_string()))
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to promote user".to_string());
        Err(AuthError::ServerError(error_text))
    }
}

pub async fn delete_user(user_id: i32) -> Result<serde_json::Value, AuthError> {
    let token = get_auth_token()?;
    
    let response = Request::delete(&format!("{}/users/{}", API_BASE_URL, user_id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| AuthError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AuthError::ParseError(e.to_string()))?;
        Ok(result)
    } else if response.status() == 401 {
        LocalStorage::delete("auth_token");
        Err(AuthError::InvalidCredentials)
    } else if response.status() == 404 {
        Err(AuthError::ServerError("User not found".to_string()))
    } else {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to delete user".to_string());
        Err(AuthError::ServerError(error_text))
    }
}
