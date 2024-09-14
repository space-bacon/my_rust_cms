use axum::{
    routing::post,
    extract::{Json, Extension},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::services::auth_service::AuthService;
use serde::Deserialize;
use std::sync::Arc;
use crate::models::auth::{AuthError, AuthToken};

#[derive(Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

// Login Handler
async fn login_handler(
    Json(auth_data): Json<AuthData>,
    Extension(auth_service): Extension<Arc<AuthService>>,
) -> impl IntoResponse {
    match auth_service.login(&auth_data.username, &auth_data.password).await {
        Ok(token) => (StatusCode::OK, Json(AuthToken { token })),
        Err(AuthError::InvalidCredentials) => (StatusCode::UNAUTHORIZED, Json("Invalid credentials".to_string())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error".to_string())),
    }
}

// Register Handler
async fn register_handler(
    Json(auth_data): Json<AuthData>,
    Extension(auth_service): Extension<Arc<AuthService>>,
) -> impl IntoResponse {
    match auth_service.register(&auth_data.username, &auth_data.password).await {
        Ok(_) => (StatusCode::CREATED, Json("User registered".to_string())),
        Err(AuthError::UserAlreadyExists) => (StatusCode::CONFLICT, Json("User already exists".to_string())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Internal server error".to_string())),
    }
}

// Initialize Routes
pub fn init_routes(auth_service: Arc<AuthService>) -> Router {
    Router::new()
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/register", post(register_handler))
        .layer(Extension(auth_service))
}
