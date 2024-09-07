use axum::{
    extract::{Path, Json},
    response::{IntoResponse, Response},
    routing::{get, post, put, delete},
    http::StatusCode,
    Router,
};
use crate::services::post_service::PostService;
use crate::models::post::Post;

async fn create_post_handler(Json(post_data): Json<Post>) -> Response {
    match PostService::create_post(post_data) {
        Ok(post) => (StatusCode::CREATED, Json(post)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
    }
}

async fn get_all_posts_handler() -> Response {
    match PostService::list_posts() {
        Ok(posts) => (StatusCode::OK, Json(posts)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response(),
    }
}

async fn get_post_handler(Path(id): Path<i32>) -> Response {
    match PostService::get_post(id) {
        Ok(post) => (StatusCode::OK, Json(post)).into_response(),
        Err(err) => (StatusCode::NOT_FOUND, Json(err)).into_response(),
    }
}

async fn update_post_handler(Path(id): Path<i32>, Json(post_data): Json<Post>) -> Response {
    match PostService::update_post(id, post_data) {
        Ok(post) => (StatusCode::OK, Json(post)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
    }
}

async fn delete_post_handler(Path(id): Path<i32>) -> Response {
    match PostService::delete_post(id) {
        Ok(_) => (StatusCode::OK, Json("Post deleted")).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response(),
    }
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/api/posts", post(create_post_handler).get(get_all_posts_handler))
        .route("/api/posts/:id", get(get_post_handler).put(update_post_handler).delete(delete_post_handler))
}
