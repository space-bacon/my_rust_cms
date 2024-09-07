use axum::{
    routing::{get, post, put, delete},
    extract::{Path, Json},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::services::user_service::UserService;
use crate::models::user::User;

async fn create_user_handler(Json(user_data): Json<User>) -> impl IntoResponse {
    match UserService::create_user(user_data) {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
    }
}

async fn get_all_users_handler() -> impl IntoResponse {
    match UserService::list_users() {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response(),
    }
}

async fn get_user_handler(Path(id): Path<i32>) -> impl IntoResponse {
    match UserService::get_user(id) {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(err) => (StatusCode::NOT_FOUND, Json(err)).into_response(),
    }
}

async fn update_user_handler(Path(id): Path<i32>, Json(user_data): Json<User>) -> impl IntoResponse {
    match UserService::update_user(id, user_data) {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
    }
}

async fn delete_user_handler(Path(id): Path<i32>) -> impl IntoResponse {
    match UserService::delete_user(id) {
        Ok(_) => (StatusCode::OK, Json("User deleted")).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response(),
    }
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/api/users", post(create_user_handler))
        .route("/api/users", get(get_all_users_handler))
        .route("/api/users/:id", get(get_user_handler))
        .route("/api/users/:id", put(update_user_handler))
        .route("/api/users/:id", delete(delete_user_handler))
}
