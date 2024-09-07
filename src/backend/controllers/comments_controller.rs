use axum::{
    routing::{get, post, put, delete},
    extract::{Path, Json},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::services::comment_service::CommentService;
use crate::models::comment::Comment;

async fn create_comment_handler(Json(comment_data): Json<Comment>) -> impl IntoResponse {
    match CommentService::create_comment(comment_data) {
        Ok(comment) => (StatusCode::CREATED, Json(comment)),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)),
    }
}

async fn get_all_comments_handler() -> impl IntoResponse {
    match CommentService::list_comments() {
        Ok(comments) => (StatusCode::OK, Json(comments)),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)),
    }
}

async fn get_comment_handler(Path(id): Path<i32>) -> impl IntoResponse {
    match CommentService::get_comment(id) {
        Ok(comment) => (StatusCode::OK, Json(comment)),
        Err(err) => (StatusCode::NOT_FOUND, Json(err)),
    }
}

async fn update_comment_handler(Path(id): Path<i32>, Json(comment_data): Json<Comment>) -> impl IntoResponse {
    match CommentService::update_comment(id, comment_data) {
        Ok(comment) => (StatusCode::OK, Json(comment)),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)),
    }
}

async fn delete_comment_handler(Path(id): Path<i32>) -> impl IntoResponse {
    match CommentService::delete_comment(id) {
        Ok(_) => (StatusCode::OK, Json("Comment deleted")),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)),
    }
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/api/comments", post(create_comment_handler))
        .route("/api/comments", get(get_all_comments_handler))
        .route("/api/comments/:id", get(get_comment_handler))
        .route("/api/comments/:id", put(update_comment_handler))
        .route("/api/comments/:id", delete(delete_comment_handler))
}
