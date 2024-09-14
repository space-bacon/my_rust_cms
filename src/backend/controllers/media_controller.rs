use axum::{
    routing::{get, post, delete},
    extract::{Path, Json, Extension},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use std::sync::Arc;
use crate::services::media_service::MediaService;
use crate::models::media::{Media, MediaError};

/// Handler for uploading media
async fn upload_media_handler(
    Json(media_data): Json<Media>,
    Extension(media_service): Extension<Arc<MediaService>>,
) -> impl IntoResponse {
    match media_service.upload_media(media_data).await {
        Ok(media) => (StatusCode::CREATED, Json(media)),
        Err(MediaError::InvalidData) => (StatusCode::BAD_REQUEST, Json("Invalid media data")),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to upload media")),
    }
}

/// Handler for fetching all media
async fn get_all_media_handler(
    Extension(media_service): Extension<Arc<MediaService>>,
) -> impl IntoResponse {
    match media_service.list_media().await {
        Ok(media) => (StatusCode::OK, Json(media)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to retrieve media")),
    }
}

/// Handler for fetching a specific media by ID
async fn get_media_handler(
    Path(id): Path<i32>,
    Extension(media_service): Extension<Arc<MediaService>>,
) -> impl IntoResponse {
    match media_service.get_media(id).await {
        Ok(media) => (StatusCode::OK, Json(media)),
        Err(MediaError::NotFound) => (StatusCode::NOT_FOUND, Json("Media not found")),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to retrieve media")),
    }
}

/// Handler for deleting media by ID
async fn delete_media_handler(
    Path(id): Path<i32>,
    Extension(media_service): Extension<Arc<MediaService>>,
) -> impl IntoResponse {
    match media_service.delete_media(id).await {
        Ok(_) => (StatusCode::OK, Json("Media deleted")),
        Err(MediaError::NotFound) => (StatusCode::NOT_FOUND, Json("Media not found")),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to delete media")),
    }
}

/// Initialize the media routes
pub fn init_routes(media_service: Arc<MediaService>) -> Router {
    Router::new()
        .route("/api/media", post(upload_media_handler))
        .route("/api/media", get(get_all_media_handler))
        .route("/api/media/:id", get(get_media_handler))
        .route("/api/media/:id", delete(delete_media_handler))
        .layer(Extension(media_service))
}
