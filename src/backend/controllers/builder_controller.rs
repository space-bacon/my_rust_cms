use axum::{
    routing::{get, post, put},
    extract::{Path, Json, Extension},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use std::sync::Arc;
use crate::services::builder_service::{BuilderService, NewPageData};
use crate::models::builder::PageError;

/// Handler for creating a new page
async fn create_page_handler(
    Json(new_page): Json<NewPageData>,
    Extension(builder_service): Extension<Arc<BuilderService>>,
) -> impl IntoResponse {
    match builder_service.create_page(new_page).await {
        Ok(page) => (StatusCode::CREATED, Json(page)),
        Err(PageError::PageAlreadyExists) => (StatusCode::CONFLICT, Json("Page already exists".to_string())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to create page".to_string())),
    }
}

/// Handler for fetching an existing page by ID
async fn get_page_handler(
    Path(id): Path<i32>,
    Extension(builder_service): Extension<Arc<BuilderService>>,
) -> impl IntoResponse {
    match builder_service.get_page(id).await {
        Ok(page) => (StatusCode::OK, Json(page)),
        Err(PageError::NotFound) => (StatusCode::NOT_FOUND, Json("Page not found".to_string())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to retrieve page".to_string())),
    }
}

/// Handler for updating an existing page by ID
async fn update_page_handler(
    Path(id): Path<i32>,
    Json(updated_page): Json<NewPageData>,
    Extension(builder_service): Extension<Arc<BuilderService>>,
) -> impl IntoResponse {
    match builder_service.update_page(id, updated_page).await {
        Ok(page) => (StatusCode::OK, Json(page)),
        Err(PageError::NotFound) => (StatusCode::NOT_FOUND, Json("Page not found".to_string())),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json("Failed to update page".to_string())),
    }
}

/// Initialize the builder routes
pub fn init_routes(builder_service: Arc<BuilderService>) -> Router {
    Router::new()
        .route("/api/builder/pages", post(create_page_handler))
        .route("/api/builder/pages/:id", get(get_page_handler))
        .route("/api/builder/pages/:id", put(update_page_handler))
        .layer(Extension(builder_service))
}
