use axum::{
    routing::{get, put},
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::services::settings_service::SettingsService;
use crate::models::settings::Settings;

async fn get_settings_handler() -> impl IntoResponse {
    match SettingsService::get_settings() {
        Ok(settings) => (StatusCode::OK, Json(settings)),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)),
    }
}

async fn update_settings_handler(Json(settings_data): Json<Settings>) -> impl IntoResponse {
    match SettingsService::update_settings(settings_data) {
        Ok(settings) => (StatusCode::OK, Json(settings)),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)),
    }
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/api/settings", get(get_settings_handler))
        .route("/api/settings", put(update_settings_handler))
}
