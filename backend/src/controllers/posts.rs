use axum::{
    extract::{State, Path, Json, Extension},
    response::Json as ResponseJson,
    http::StatusCode,
};

use crate::{
    AppServices,
    models::{Post, NewPost, UpdatePost},
    middleware::{
        validation::validate_text_content,
        errors::AppError,
        auth::AuthenticatedUser,
    },
};

// Frontend-compatible Post structure
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FrontendPost {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub author: String,
    pub status: String,
    pub category_id: Option<i32>,
    pub created_at: Option<String>,
}

impl From<Post> for FrontendPost {
    fn from(post: Post) -> Self {
        FrontendPost {
            id: Some(post.id),
            title: post.title,
            content: post.content,
            author: "Admin".to_string(), // Default for now
            status: "published".to_string(), // Default for now
            category_id: post.category_id,
            created_at: post.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

/// Get all posts (public endpoint)
/// 
/// Returns a list of all published posts.
/// No authentication required for public access.
pub async fn get_posts(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<FrontendPost>>, AppError> {
    let posts = services.db_service.execute(|conn| {
        Post::list(conn)
    }).await?;
    
    let frontend_posts: Vec<FrontendPost> = posts.into_iter().map(FrontendPost::from).collect();
    Ok(ResponseJson(frontend_posts))
}

/// Get a specific post by ID (public endpoint)
/// 
/// Returns a single post by its ID.
/// No authentication required for public access.
pub async fn get_post(
    State(services): State<AppServices>, 
    Path(id): Path<i32>
) -> Result<ResponseJson<FrontendPost>, AppError> {
    let post = services.db_service.execute_optional(move |conn| {
        Ok(Post::find_by_id(conn, id)?)
    }).await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;
    
    Ok(ResponseJson(FrontendPost::from(post)))
}

/// Create a new post (admin only)
/// 
/// Creates a new blog post with validation.
/// Content is sanitized and validated for security.
/// Requires admin authentication.
pub async fn create_post(
    Extension(auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>, 
    Json(frontend_post): Json<FrontendPost>
) -> Result<(StatusCode, ResponseJson<FrontendPost>), AppError> {
    // Validate input
    if frontend_post.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    
    validate_text_content(&frontend_post.title, 200)?;
    validate_text_content(&frontend_post.content, 50000)?;
    
    let new_post = NewPost {
        title: frontend_post.title.trim().to_string(),
        content: frontend_post.content.trim().to_string(),
        category_id: frontend_post.category_id,
        user_id: Some(auth_user.id),
    };
    
    let created_post = services.db_service.execute(move |conn| {
        Post::create(conn, new_post)
    }).await?;
    let response = FrontendPost {
        id: Some(created_post.id),
        title: created_post.title,
        content: created_post.content,
        author: frontend_post.author,
        status: frontend_post.status,
        category_id: created_post.category_id,
        created_at: created_post.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
    };
    
    Ok((StatusCode::CREATED, ResponseJson(response)))
}

/// Update an existing post (admin only)
/// 
/// Updates a post with validation and sanitization.
/// Requires admin authentication.
pub async fn update_post(
    Extension(_auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>, 
    Path(id): Path<i32>, 
    Json(frontend_post): Json<FrontendPost>
) -> Result<ResponseJson<FrontendPost>, AppError> {
    // Validate input
    if frontend_post.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    
    validate_text_content(&frontend_post.title, 200)?;
    validate_text_content(&frontend_post.content, 50000)?;
    
    // Check if post exists and update in one operation
    let update_post = UpdatePost {
        title: Some(frontend_post.title.trim().to_string()),
        content: Some(frontend_post.content.trim().to_string()),
        category_id: frontend_post.category_id,
        user_id: None,
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };
    
    let updated_post = services.db_service.execute(move |conn| {
        // Check if post exists
        let _existing_post = Post::find_by_id(conn, id)?
            .ok_or_else(|| diesel::result::Error::NotFound)?;
        
        Post::update(conn, id, update_post)
    }).await.map_err(|e| match e {
        AppError::DatabaseError(msg) if msg.contains("NotFound") => 
            AppError::NotFound("Post not found".to_string()),
        other => other,
    })?;
    Ok(ResponseJson(FrontendPost::from(updated_post)))
}

/// Delete a post (admin only)
/// 
/// Permanently deletes a post and associated data.
/// Requires admin authentication.
pub async fn delete_post(
    Extension(_auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>, 
    Path(id): Path<i32>
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    services.db_service.execute(move |conn| {
        // Check if post exists
        let _existing_post = Post::find_by_id(conn, id)?
            .ok_or_else(|| diesel::result::Error::NotFound)?;
        
        Post::delete(conn, id)
    }).await.map_err(|e| match e {
        AppError::DatabaseError(msg) if msg.contains("NotFound") => 
            AppError::NotFound("Post not found".to_string()),
        other => other,
    })?;
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": "Post deleted successfully"
    })))
}