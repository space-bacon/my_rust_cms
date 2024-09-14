use axum::{
    routing::{get, post, delete},
    extract::{Path, Json, State},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::backend::services::media_service::MediaServiceError;
use crate::backend::models::media::{Media, CreateMedia};
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

/// Handler for uploading media
async fn upload_media_handler(
    State(state): State<AppState>,
    Json(media_data): Json<CreateMedia>,
) -> impl IntoResponse {
    let media_service = &state.media_service;
    match media_service.upload_media(media_data).await {
        Ok(media) => (
            StatusCode::CREATED,
            Json(SuccessResponse { data: media }),
        ),
        Err(MediaServiceError::InvalidData) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid media data".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to upload media".to_string(),
            }),
        ),
    }
}

/// Handler for fetching all media
async fn get_all_media_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let media_service = &state.media_service;
    match media_service.list_media().await {
        Ok(media_list) => (
            StatusCode::OK,
            Json(SuccessResponse { data: media_list }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to retrieve media".to_string(),
            }),
        ),
    }
}

/// Handler for fetching a specific media by ID
async fn get_media_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let media_service = &state.media_service;
    match media_service.get_media(id).await {
        Ok(media) => (
            StatusCode::OK,
            Json(SuccessResponse { data: media }),
        ),
        Err(MediaServiceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Media not found".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to retrieve media".to_string(),
            }),
        ),
    }
}

/// Handler for deleting media by ID
async fn delete_media_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let media_service = &state.media_service;
    match media_service.delete_media(id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"message": "Media deleted"})),
        ),
        Err(MediaServiceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Media not found".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to delete media".to_string(),
            }),
        ),
    }
}

/// Initialize the media routes
pub fn routes() -> Router {
    Router::new()
        .route("/", post(upload_media_handler))
        .route("/", get(get_all_media_handler))
        .route("/:id", get(get_media_handler))
        .route("/:id", delete(delete_media_handler))
}
