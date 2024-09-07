use axum::{
    routing::{get, post, put},
    extract::{Path, Json},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::services::builder_service::{BuilderService, NewPageData};

async fn create_page_handler(Json(new_page): Json<NewPageData>) -> impl IntoResponse {
    match BuilderService::create_page(new_page) {
        Ok(page) => (StatusCode::CREATED, Json(page)),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)),
    }
}

async fn get_page_handler(Path(id): Path<i32>) -> impl IntoResponse {
    match BuilderService::get_page(id) {
        Ok(page) => (StatusCode::OK, Json(page)),
        Err(err) => (StatusCode::NOT_FOUND, Json(err)),
    }
}

async fn update_page_handler(Path(id): Path<i32>, Json(updated_page): Json<NewPageData>) -> impl IntoResponse {
    match BuilderService::update_page(id, updated_page) {
        Ok(page) => (StatusCode::OK, Json(page)),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)),
    }
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/api/builder/pages", post(create_page_handler))
        .route("/api/builder/pages/:id", get(get_page_handler))
        .route("/api/builder/pages/:id", put(update_page_handler))
}
