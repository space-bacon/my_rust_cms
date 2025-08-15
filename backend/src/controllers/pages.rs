use axum::{
    extract::{State, Path, Json, Extension},
    response::Json as ResponseJson,
    http::StatusCode,
};
use crate::{
    AppServices,
    models::{Page, NewPage, UpdatePage},
    middleware::{
        validation::validate_text_content,
        errors::AppError,
        auth::AuthenticatedUser,
    },
};

// Frontend-compatible Page structure
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FrontendPage {
    pub id: Option<i32>,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<Page> for FrontendPage {
    fn from(page: Page) -> Self {
        FrontendPage {
            id: Some(page.id),
            title: page.title,
            slug: page.slug,
            content: page.content,
            status: page.status,
            created_at: page.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            updated_at: page.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

/// Get all pages (public endpoint)
/// 
/// Returns a list of all published pages.
/// No authentication required for public access.
pub async fn get_pages(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<FrontendPage>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let pages = Page::list(&mut conn)?;
    let frontend_pages: Vec<FrontendPage> = pages.into_iter().map(FrontendPage::from).collect();
    Ok(ResponseJson(frontend_pages))
}

/// Get a specific page by ID (public endpoint)
/// 
/// Returns a single page by its ID.
/// No authentication required for public access.
pub async fn get_page(
    State(services): State<AppServices>, 
    Path(id): Path<i32>
) -> Result<ResponseJson<FrontendPage>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let page = Page::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Page not found".to_string()))?;
    
    Ok(ResponseJson(FrontendPage::from(page)))
}

/// Get a page by slug (public endpoint)
/// 
/// Returns a page by its URL slug.
/// Generates slug from title since database doesn't store slugs.
/// No authentication required for public access.
pub async fn get_page_by_slug(
    State(services): State<AppServices>, 
    Path(slug): Path<String>
) -> Result<ResponseJson<FrontendPage>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let normalized_slug = slug.trim().to_lowercase();
    let page = Page::find_by_slug(&mut conn, &normalized_slug)?
        .ok_or_else(|| AppError::NotFound("Page not found".to_string()))?;
    Ok(ResponseJson(FrontendPage::from(page)))
}

/// Create a new page (admin only)
/// 
/// Creates a new page with validation.
/// Content is sanitized and validated for security.
/// Requires admin authentication.
pub async fn create_page(
    Extension(auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>, 
    Json(page): Json<FrontendPage>
) -> Result<(StatusCode, ResponseJson<FrontendPage>), AppError> {
    // Validate input
    if page.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    
    validate_text_content(&page.title, 200)?;
    validate_text_content(&page.content, 200000)?;
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Generate slug if missing and normalize
    let mut slug_value = if page.slug.trim().is_empty() {
        page.title.trim().to_lowercase().replace(' ', "-")
    } else {
        page.slug.trim().to_lowercase().replace(' ', "-")
    };
    // collapse consecutive dashes
    while slug_value.contains("--") { slug_value = slug_value.replace("--", "-"); }

    // Default status
    let status_value = if page.status.trim().is_empty() { "draft".to_string() } else { page.status.trim().to_string() };

    // Pre-check unique slug
    if let Ok(Some(_)) = Page::find_by_slug(&mut conn, &slug_value) {
        return Err(AppError::ConflictError("Slug already exists".to_string()));
    }

    let new_page = NewPage {
        title: page.title.trim().to_string(),
        content: page.content.trim().to_string(),
        user_id: Some(auth_user.id),
        slug: slug_value,
        status: status_value,
    };
    
    let created_page = Page::create(&mut conn, new_page)?;
    let response = FrontendPage::from(created_page);
    
    Ok((StatusCode::CREATED, ResponseJson(response)))
}

/// Update an existing page (admin only)
/// 
/// Updates a page with validation and sanitization.
/// Requires admin authentication.
pub async fn update_page(
    State(services): State<AppServices>, 
    Path(id): Path<i32>, 
    Json(page): Json<FrontendPage>
) -> Result<ResponseJson<FrontendPage>, AppError> {
    // Validate input
    if page.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    
    validate_text_content(&page.title, 200)?;
    validate_text_content(&page.content, 200000)?;
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if page exists
    let _existing_page = Page::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Page not found".to_string()))?;
    
    // Normalize slug and default status
    let mut slug_value = if page.slug.trim().is_empty() {
        page.title.trim().to_lowercase().replace(' ', "-")
    } else {
        page.slug.trim().to_lowercase().replace(' ', "-")
    };
    while slug_value.contains("--") { slug_value = slug_value.replace("--", "-"); }
    let status_value = if page.status.trim().is_empty() { "draft".to_string() } else { page.status.trim().to_string() };

    // If slug changed, ensure uniqueness (excluding this id)
    if let Ok(Some(existing)) = Page::find_by_slug(&mut conn, &slug_value) {
        if existing.id != id { return Err(AppError::ConflictError("Slug already exists".to_string())); }
    }

    let update_page = UpdatePage {
        title: Some(page.title.trim().to_string()),
        content: Some(page.content.trim().to_string()),
        user_id: None,
        updated_at: Some(chrono::Utc::now().naive_utc()),
        slug: Some(slug_value),
        status: Some(status_value),
    };
    
    let updated_page = Page::update(&mut conn, id, update_page)?;
    Ok(ResponseJson(FrontendPage::from(updated_page)))
}

/// Update page content for live editing (authenticated users)
/// 
/// Updates only the content field of a page for live editing.
/// Requires user authentication but not admin privileges.
/// More restrictive than full page update - only allows content changes.
pub async fn update_page_content(
    State(services): State<AppServices>,
    Path(id): Path<i32>,
    Json(request): Json<serde_json::Value>
) -> Result<ResponseJson<FrontendPage>, AppError> {
    // Extract content from request
    let content = request.get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::ValidationError("Content field is required".to_string()))?;
    
    // Validate content length
    validate_text_content(content, 200000)?;
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if page exists
    let existing_page = Page::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Page not found".to_string()))?;
    
    // Update only the content field
    let update_page = UpdatePage {
        title: None,
        content: Some(content.to_string()),
        user_id: None,
        updated_at: Some(chrono::Utc::now().naive_utc()),
        slug: None,
        status: None,
    };
    
    let updated_page = Page::update(&mut conn, id, update_page)?;
    Ok(ResponseJson(FrontendPage::from(updated_page)))
}

/// Delete a page (admin only)
/// 
/// Permanently deletes a page.
/// Requires admin authentication.
pub async fn delete_page(
    State(services): State<AppServices>, 
    Path(id): Path<i32>
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if page exists
    let _existing_page = Page::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Page not found".to_string()))?;
    
    Page::delete(&mut conn, id)?;
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": "Page deleted successfully"
    })))
}