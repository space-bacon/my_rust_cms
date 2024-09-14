use axum::{
    routing::{get, post, put, delete},
    extract::{Path, Json, State},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use crate::backend::services::category_service::CategoryServiceError;
use crate::backend::models::category::{Category, CreateCategory, UpdateCategory};
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

/// Handler for creating a new category
async fn create_category_handler(
    State(state): State<AppState>,
    Json(category_data): Json<CreateCategory>,
) -> impl IntoResponse {
    let category_service = &state.category_service;
    match category_service.create_category(category_data).await {
        Ok(category) => (
            StatusCode::CREATED,
            Json(SuccessResponse { data: category }),
        ),
        Err(CategoryServiceError::InvalidData) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid category data".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to create category".to_string(),
            }),
        ),
    }
}

/// Handler for retrieving all categories
async fn get_all_categories_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let category_service = &state.category_service;
    match category_service.list_categories().await {
        Ok(categories) => (
            StatusCode::OK,
            Json(SuccessResponse { data: categories }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to retrieve categories".to_string(),
            }),
        ),
    }
}

/// Handler for retrieving a specific category by ID
async fn get_category_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let category_service = &state.category_service;
    match category_service.get_category(id).await {
        Ok(category) => (
            StatusCode::OK,
            Json(SuccessResponse { data: category }),
        ),
        Err(CategoryServiceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Category not found".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to retrieve category".to_string(),
            }),
        ),
    }
}

/// Handler for updating a category by ID
async fn update_category_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(category_data): Json<UpdateCategory>,
) -> impl IntoResponse {
    let category_service = &state.category_service;
    match category_service.update_category(id, category_data).await {
        Ok(category) => (
            StatusCode::OK,
            Json(SuccessResponse { data: category }),
        ),
        Err(CategoryServiceError::InvalidData) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid category data".to_string(),
            }),
        ),
        Err(CategoryServiceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Category not found".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to update category".to_string(),
            }),
        ),
    }
}

/// Handler for deleting a category by ID
async fn delete_category_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let category_service = &state.category_service;
    match category_service.delete_category(id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({"message": "Category deleted"})),
        ),
        Err(CategoryServiceError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Category not found".to_string(),
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "Failed to delete category".to_string(),
            }),
        ),
    }
}

/// Initialize the category routes
pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_category_handler).get(get_all_categories_handler))
        .route(
            "/:id",
            get(get_category_handler)
                .put(update_category_handler)
                .delete(delete_category_handler),
        )
}
