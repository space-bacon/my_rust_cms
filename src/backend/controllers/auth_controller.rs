use axum::{
    routing::post,
    extract::{Json, Extension},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::services::auth_service::AuthService;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::models::auth::{AuthError, AuthToken};

#[derive(Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Login Handler
async fn login_handler(
    Json(auth_data): Json<AuthData>,
    Extension(auth_service): Extension<Arc<AuthService>>,
) -> impl IntoResponse {
    match auth_service.login(&auth_data.username, &auth_data.password).await {
        Ok(token) => (StatusCode::OK, Json(AuthToken { token })),
        Err(AuthError::InvalidCredentials) => (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Invalid credentials".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".to_string(),
            }),
        ),
    }
}

// Register Handler
async fn register_handler(
    Json(auth_data): Json<AuthData>,
    Extension(auth_service): Extension<Arc<AuthService>>,
) -> impl IntoResponse {
    match auth_service
        .register(&auth_data.username, &auth_data.password)
        .await
    {
        Ok(_) => (
            StatusCode::CREATED,
            Json("User registered".to_string()),
        ),
        Err(AuthError::UserAlreadyExists) => (
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "User already exists".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Internal server error".to_string(),
            }),
        ),
    }
}

// Initialize Routes
pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler))
        // `AuthService` should be added to the application state, not per module
}
