use axum::{
    extract::{Path, Json, Extension},
    response::{IntoResponse, Response},
    routing::{get, post, put, delete},
    http::StatusCode,
    Router,
};
use std::sync::Arc;
use crate::services::post_service::{PostService, PostServiceError};
use crate::models::post::Post;

/// Handler for creating a new post
async fn create_post_handler(
    Json(post_data): Json<Post>,
    Extension(post_service): Extension<Arc<PostService>>,
) -> Response {
    match post_service.create_post(post_data).await {
        Ok(post) => (StatusCode::CREATED, Json(post)).into_response(),
        Err(PostServiceError::InvalidData) => (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid post data"}))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to create post"}))).into_response(),
    }
}

/// Handler for retrieving all posts
async fn get_all_posts_handler(
    Extension(post_service): Extension<Arc<PostService>>,
) -> Response {
    match post_service.list_posts().await {
        Ok(posts) => (StatusCode::OK, Json(posts)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to retrieve posts"}))).into_response(),
    }
}

/// Handler for retrieving a specific post by ID
async fn get_post_handler(
    Path(id): Path<i32>,
    Extension(post_service): Extension<Arc<PostService>>,
) -> Response {
    match post_service.get_post(id).await {
        Ok(post) => (StatusCode::OK, Json(post)).into_response(),
        Err(PostServiceError::NotFound) => (StatusCode::NOT_FOUND, Json(json!({"error": "Post not found"}))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to retrieve post"}))).into_response(),
    }
}

/// Handler for updating a post by ID
async fn update_post_handler(
    Path(id): Path<i32>,
    Json(post_data): Json<Post>,
    Extension(post_service): Extension<Arc<PostService>>,
) -> Response {
    match post_service.update_post(id, post_data).await {
        Ok(post) => (StatusCode::OK, Json(post)).into_response(),
        Err(PostServiceError::InvalidData) => (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid post data"}))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to update post"}))).into_response(),
    }
}

/// Handler for deleting a post by ID
async fn delete_post_handler(
    Path(id): Path<i32>,
    Extension(post_service): Extension<Arc<PostService>>,
) -> Response {
    match post_service.delete_post(id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Post deleted"}))).into_response(),
        Err(PostServiceError::NotFound) => (StatusCode::NOT_FOUND, Json(json!({"error": "Post not found"}))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to delete post"}))).into_response(),
    }
}

/// Initialize the routes for posts
pub fn init_routes(post_service: Arc<PostService>) -> Router {
    Router::new()
        .route(
            "/api/posts", 
            post(create_post_handler)
            .get(get_all_posts_handler)
        )
        .route(
            "/api/posts/:id", 
            get(get_post_handler)
            .put(update_post_handler)
            .delete(delete_post_handler)
        )
        .layer(Extension(post_service))
}
