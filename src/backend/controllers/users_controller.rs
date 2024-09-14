use axum::{
    routing::{get, post, put, delete},
    extract::{Path, Json, Extension},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::services::user_service::UserService;
use crate::models::user::User;
use std::sync::Arc;
use axum::Json;

async fn create_user_handler(
    Json(user_data): Json<User>,
    Extension(user_service): Extension<Arc<UserService>>
) -> impl IntoResponse {
    match user_service.create_user(user_data).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
    }
}

async fn get_all_users_handler(
    Extension(user_service): Extension<Arc<UserService>>
) -> impl IntoResponse {
    match user_service.list_users().await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response(),
    }
}

async fn get_user_handler(
    Path(id): Path<i32>,
    Extension(user_service): Extension<Arc<UserService>>
) -> impl IntoResponse {
    match user_service.get_user(id).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(err) => (StatusCode::NOT_FOUND, Json(err)).into_response(),
    }
}

async fn update_user_handler(
    Path(id): Path<i32>,
    Json(user_data): Json<User>,
    Extension(user_service): Extension<Arc<UserService>>
) -> impl IntoResponse {
    match user_service.update_user(id, user_data).await {
        Ok(user) => (StatusCode::OK, Json(user)).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
    }
}

async fn delete_user_handler(
    Path(id): Path<i32>,
    Extension(user_service): Extension<Arc<UserService>>
) -> impl IntoResponse {
    match user_service.delete_user(id).await {
        Ok(_) => (StatusCode::OK, Json("User deleted")).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(err)).into_response(),
    }
}

pub fn init_routes(user_service: Arc<UserService>) -> Router {
    Router::new()
        .route("/api/users", get(get_all_users_handler))
        .route("/api/users", post(create_user_handler))
        .route("/api/users/:id", get(get_user_handler))
        .route("/api/users/:id", put(update_user_handler))
        .route("/api/users/:id", delete(delete_user_handler))
        .layer(Extension(user_service))
}
