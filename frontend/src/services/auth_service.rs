use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use gloo_storage::{LocalStorage, Storage};

const API_BASE_URL: &str = "http://localhost:8081/api";

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SignupCredentials {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub email_verified: Option<bool>,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug)]
pub enum AuthError {
    NetworkError(String),
    ParseError(String),
    ServerError(String),
    InvalidCredentials,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            AuthError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AuthError::ServerError(msg) => write!(f, "Server error: {}", msg),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
        }
    }
}

pub async fn login(credentials: &LoginCredentials) -> Result<AuthResponse, AuthError> {
    let response = Request::post(&format!("{}/auth/login", API_BASE_URL))
        .json(credentials)
        .map_err(|e| AuthError::ParseError(e.to_string()))?
        .send()
        .await
        .map_err(|e| AuthError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let auth_response: AuthResponse = response
            .json()
            .await
            .map_err(|e| AuthError::ParseError(e.to_string()))?;
        
        // Store the token
        LocalStorage::set("auth_token", &auth_response.token)
            .map_err(|e| AuthError::ParseError(e.to_string()))?;
        
        Ok(auth_response)
    } else if response.status() == 401 {
        Err(AuthError::InvalidCredentials)
    } else {
        Err(AuthError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn logout() -> Result<(), AuthError> {
    let token = get_auth_token()?;
    
    let response = Request::post(&format!("{}/auth/logout", API_BASE_URL))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| AuthError::NetworkError(e.to_string()))?;

    // Clear the token regardless of response
    LocalStorage::delete("auth_token");
    
    if response.status() == 200 {
        Ok(())
    } else {
        Err(AuthError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn get_current_user() -> Result<User, AuthError> {
    let token = get_auth_token()?;
    
    let response = Request::get(&format!("{}/auth/me", API_BASE_URL))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| AuthError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let user: User = response
            .json()
            .await
            .map_err(|e| AuthError::ParseError(e.to_string()))?;
        Ok(user)
    } else if response.status() == 401 {
        // Clear invalid token
        LocalStorage::delete("auth_token");
        Err(AuthError::InvalidCredentials)
    } else {
        Err(AuthError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub fn get_auth_token() -> Result<String, AuthError> {
    LocalStorage::get("auth_token")
        .map_err(|e| AuthError::ParseError(e.to_string()))
}

pub fn is_authenticated() -> bool {
    get_auth_token().is_ok()
}

#[allow(dead_code)]
pub async fn verify_token() -> Result<User, AuthError> {
    get_current_user().await
}

pub async fn signup(credentials: &SignupCredentials) -> Result<serde_json::Value, AuthError> {
    let response = Request::post(&format!("{}/auth/signup", API_BASE_URL))
        .json(credentials)
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
    } else if response.status() == 409 {
        Err(AuthError::ServerError("Username or email already exists".to_string()))
    } else if response.status() == 400 {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Invalid input".to_string());
        Err(AuthError::ServerError(error_text))
    } else {
        Err(AuthError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn verify_email(token: &str) -> Result<serde_json::Value, AuthError> {
    let request = VerifyEmailRequest {
        token: token.to_string(),
    };

    let response = Request::post(&format!("{}/auth/verify-email", API_BASE_URL))
        .json(&request)
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
    } else if response.status() == 400 {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Invalid or expired token".to_string());
        Err(AuthError::ServerError(error_text))
    } else {
        Err(AuthError::ServerError(format!("HTTP {}", response.status())))
    }
}

#[allow(dead_code)]
pub async fn refresh_session() -> Result<User, AuthError> {
    // For now, just verify the current token
    // In the future, this could implement actual token refresh
    get_current_user().await
}

pub fn clear_auth() {
    LocalStorage::delete("auth_token");
} 