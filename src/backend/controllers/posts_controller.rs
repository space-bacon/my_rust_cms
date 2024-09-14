use axum::{
    extract::{Path, Json, State},
    response::IntoResponse,
    http::StatusCode,
    Router,
};
use crate::services::post_service::PostServiceError;
use crate::models::post::{Post, CreatePost, UpdatePost};
use crate::AppState; // Assuming AppState is defined in a common module
use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct SuccessResponse<T> {
    data: T,
}

/// Handler for creating a new post
async fn create_post_handler(
    State(state): State<AppState>,
    Json(post_data): Json<CreatePost>,
) -> impl IntoResponse {
    let post_service = &state.post_service;
    match post_service.create_post(post_data).await {
        Ok(post) => (
            StatusCode::CREATED,
            Json(SuccessResponse { data: post }),
        ),
        Err(PostServiceError::InvalidData) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid post data".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create post".to_string(),
            }),
        ),
    }
}

/// Handler for retrieving all posts
async fn get_all_posts_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let post_service = &state.post_service;
    match post_service.list_posts().await {
        Ok(posts) => (
            StatusCode::OK,
            Json(SuccessResponse { data: posts }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to retrieve posts".to_string(),
            }),
        ),
    }
}

/// Handler for retrieving a specific post by ID
async fn get_post_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let post_service = &state.post_service;
    match post_service.get_post(id).await {
        Ok(post) => (
            StatusCode::OK,
            Json(SuccessResponse { data: post }),
        ),
        Err(PostServiceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Post not found".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to retrieve post".to_string(),
            }),
        ),
    }
}

/// Handler for updating a post by ID
async fn update_post_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(post_data): Json<UpdatePost>,
) -> impl IntoResponse {
    let post_service = &state.post_service;
    match post_service.update_post(id, post_data).await {
        Ok(post) => (
            StatusCode::OK,
            Json(SuccessResponse { data: post }),
        ),
        Err(PostServiceError::InvalidData) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid post data".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to update post".to_string(),
            }),
        ),
    }
}

/// Handler for deleting a post by ID
async fn delete_post_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let post_service = &state.post_service;
    match post_service.delete_post(id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"message": "Post deleted"})),
        ),
        Err(PostServiceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Post not found".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to delete post".to_string(),
            }),
        ),
    }
}

/// Initialize the routes for posts
pub fn routes() -> Router {
    Router::new()
        .route(
            "/",
            post(create_post_handler).get(get_all_posts_handler),
        )
        .route(
            "/:id",
            get(get_post_handler)
                .put(update_post_handler)
                .delete(delete_post_handler),
        )
}
