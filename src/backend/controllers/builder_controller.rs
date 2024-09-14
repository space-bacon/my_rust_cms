use axum::{
    routing::{get, post, put},
    extract::{Path, Json, State},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::backend::services::builder_service::BuilderServiceError;
use crate::backend::models::builder::{Page, CreatePageData, UpdatePageData};
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

/// Handler for creating a new page
async fn create_page_handler(
    State(state): State<AppState>,
    Json(new_page): Json<CreatePageData>,
) -> impl IntoResponse {
    let builder_service = &state.builder_service;
    match builder_service.create_page(new_page).await {
        Ok(page) => (
            StatusCode::CREATED,
            Json(SuccessResponse { data: page }),
        ),
        Err(BuilderServiceError::PageAlreadyExists) => (
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                error: "Page already exists".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create page".to_string(),
            }),
        ),
    }
}

/// Handler for fetching an existing page by ID
async fn get_page_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let builder_service = &state.builder_service;
    match builder_service.get_page(id).await {
        Ok(page) => (
            StatusCode::OK,
            Json(SuccessResponse { data: page }),
        ),
        Err(BuilderServiceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Page not found".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to retrieve page".to_string(),
            }),
        ),
    }
}

/// Handler for updating an existing page by ID
async fn update_page_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(updated_page): Json<UpdatePageData>,
) -> impl IntoResponse {
    let builder_service = &state.builder_service;
    match builder_service.update_page(id, updated_page).await {
        Ok(page) => (
            StatusCode::OK,
            Json(SuccessResponse { data: page }),
        ),
        Err(BuilderServiceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Page not found".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to update page".to_string(),
            }),
        ),
    }
}

/// Initialize the builder routes
pub fn routes() -> Router {
    Router::new()
        .route("/pages", post(create_page_handler))
        .route("/pages/:id", get(get_page_handler).put(update_page_handler))
}
