use axum::{
    routing::{get, post, delete},
    extract::{Path, Json},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::services::media_service::MediaService;
use crate::models::media::Media;

async fn upload_media_handler(Json(media_data): Json<Media>) -> impl IntoResponse {
    match MediaService::upload_media(media_data) {
        Ok(media) => (StatusCode::CREATED, Json(media)),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)),
    }
}

async fn get_all_media_handler() -> impl IntoResponse {
    match MediaService::list_media() {
        Ok(media) => (StatusCode::OK, Json(media)),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)),
    }
}

async fn get_media_handler(Path(id): Path<i32>) -> impl IntoResponse {
    match MediaService::get_media(id) {
        Ok(media) => (StatusCode::OK, Json(media)),
        Err(err) => (StatusCode::NOT_FOUND, Json(err)),
    }
}

async fn delete_media_handler(Path(id): Path<i32>) -> impl IntoResponse {
    match MediaService::delete_media(id) {
        Ok(_) => (StatusCode::OK, Json("Media deleted")),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)),
    }
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/api/media", post(upload_media_handler))
        .route("/api/media", get(get_all_media_handler))
        .route("/api/media/:id", get(get_media_handler))
        .route("/api/media/:id", delete(delete_media_handler))
}
