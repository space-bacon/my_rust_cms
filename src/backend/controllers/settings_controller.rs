use axum::{
    routing::{get, put},
    extract::{Json, Extension},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use std::sync::Arc;
use crate::services::settings_service::SettingsService;
use crate::models::settings::Settings;

/// Handler for retrieving settings
async fn get_settings_handler(
    Extension(settings_service): Extension<Arc<SettingsService>>
) -> impl IntoResponse {
    match settings_service.get_settings().await {
        Ok(settings) => (StatusCode::OK, Json(settings)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to retrieve settings")).into_response(),
    }
}

/// Handler for updating settings
async fn update_settings_handler(
    Json(settings_data): Json<Settings>,
    Extension(settings_service): Extension<Arc<SettingsService>>
) -> impl IntoResponse {
    match settings_service.update_settings(settings_data).await {
        Ok(settings) => (StatusCode::OK, Json(settings)).into_response(),
        Err(_) => (StatusCode::BAD_REQUEST, Json("Failed to update settings")).into_response(),
    }
}

/// Initialize the routes for settings
pub fn init_routes(settings_service: Arc<SettingsService>) -> Router {
    Router::new()
        .route("/api/settings", get(get_settings_handler).put(update_settings_handler))
        .layer(Extension(settings_service))
}
