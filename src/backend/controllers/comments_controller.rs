use axum::{
    routing::{get, post, put, delete},
    extract::{Path, Json, Extension},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use std::sync::Arc;
use crate::services::comment_service::CommentService;
use crate::models::comment::Comment;
use crate::models::comment::CommentError;

/// Handler for creating a comment
async fn create_comment_handler(
    Json(comment_data): Json<Comment>,
    Extension(comment_service): Extension<Arc<CommentService>>,
) -> impl IntoResponse {
    match comment_service.create_comment(comment_data).await {
        Ok(comment) => (StatusCode::CREATED, Json(comment)),
        Err(CommentError::InvalidData) => (StatusCode::BAD_REQUEST, Json("Invalid comment data")),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to create comment")),
    }
}

/// Handler for fetching all comments
async fn get_all_comments_handler(
    Extension(comment_service): Extension<Arc<CommentService>>,
) -> impl IntoResponse {
    match comment_service.list_comments().await {
        Ok(comments) => (StatusCode::OK, Json(comments)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to retrieve comments")),
    }
}

/// Handler for fetching a comment by ID
async fn get_comment_handler(
    Path(id): Path<i32>,
    Extension(comment_service): Extension<Arc<CommentService>>,
) -> impl IntoResponse {
    match comment_service.get_comment(id).await {
        Ok(comment) => (StatusCode::OK, Json(comment)),
        Err(CommentError::NotFound) => (StatusCode::NOT_FOUND, Json("Comment not found")),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to retrieve comment")),
    }
}

/// Handler for updating a comment by ID
async fn update_comment_handler(
    Path(id): Path<i32>,
    Json(comment_data): Json<Comment>,
    Extension(comment_service): Extension<Arc<CommentService>>,
) -> impl IntoResponse {
    match comment_service.update_comment(id, comment_data).await {
        Ok(comment) => (StatusCode::OK, Json(comment)),
        Err(CommentError::NotFound) => (StatusCode::NOT_FOUND, Json("Comment not found")),
        Err(CommentError::InvalidData) => (StatusCode::BAD_REQUEST, Json("Invalid comment data")),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to update comment")),
    }
}

/// Handler for deleting a comment by ID
async fn delete_comment_handler(
    Path(id): Path<i32>,
    Extension(comment_service): Extension<Arc<CommentService>>,
) -> impl IntoResponse {
    match comment_service.delete_comment(id).await {
        Ok(_) => (StatusCode::OK, Json("Comment deleted")),
        Err(CommentError::NotFound) => (StatusCode::NOT_FOUND, Json("Comment not found")),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to delete comment")),
    }
}

/// Initialize the comment routes
pub fn init_routes(comment_service: Arc<CommentService>) -> Router {
    Router::new()
        .route("/api/comments", post(create_comment_handler))
        .route("/api/comments", get(get_all_comments_handler))
        .route("/api/comments/:id", get(get_comment_handler))
        .route("/api/comments/:id", put(update_comment_handler))
        .route("/api/comments/:id", delete(delete_comment_handler))
        .layer(Extension(comment_service))
}
