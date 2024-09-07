use axum::{
    routing::post,
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::services::auth_service::AuthService;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

async fn login_handler(Json(auth_data): Json<AuthData>) -> impl IntoResponse {
    match AuthService::login(&auth_data.username, &auth_data.password) {
        Ok(token) => (StatusCode::OK, Json(token)),
        Err(err) => (StatusCode::UNAUTHORIZED, Json(err)),
    }
}

async fn register_handler(Json(auth_data): Json<AuthData>) -> impl IntoResponse {
    match AuthService::register(&auth_data.username, &auth_data.username, &auth_data.password) {
        Ok(_) => (StatusCode::CREATED, Json("User registered")),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)),
    }
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/register", post(register_handler))
}
