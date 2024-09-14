use axum::{
    routing::{get, put},
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::backend::services::settings_service::SettingsServiceError;
use crate::backend::models::settings::{Settings, UpdateSettings};
use crate::backend::AppState;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct SuccessResponse<T> {
    data: T,
}

/// Handler for retrieving settings
async fn get_settings_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let settings_service = &state.settings_service;
    match settings_service.get_settings().await {
        Ok(settings) => (
            StatusCode::OK,
            Json(SuccessResponse { data: settings }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to retrieve settings".to_string(),
            }),
        ),
    }
}

/// Handler for updating settings
async fn update_settings_handler(
    State(state): State<AppState>,
    Json(settings_data): Json<UpdateSettings>,
) -> impl IntoResponse {
    let settings_service = &state.settings_service;
    match settings_service.update_settings(settings_data).await {
        Ok(settings) => (
            StatusCode::OK,
            Json(SuccessResponse { data: settings }),
        ),
        Err(SettingsServiceError::InvalidData) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid settings data".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to update settings".to_string(),
            }),
        ),
    }
}

/// Initialize the routes for settings
pub fn routes() -> Router {
    Router::new()
        .route(
            "/",
            get(get_settings_handler).put(update_settings_handler),
        )
}
