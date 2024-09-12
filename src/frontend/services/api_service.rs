use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

// Post structure
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub content: String,
}

// AuthData structure for login
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

// Error Handling
#[derive(Debug)]
pub enum ApiServiceError {
    RequestError(String),
    ParseError(String),
    ServerError(u16),
}

impl From<gloo_net::Error> for ApiServiceError {
    fn from(error: gloo_net::Error) -> Self {
        ApiServiceError::RequestError(format!("Network error: {:?}", error))
    }
}

impl From<serde_json::Error> for ApiServiceError {
    fn from(error: serde_json::Error) -> Self {
        ApiServiceError::ParseError(format!("Serialization error: {:?}", error))
    }
}

// Fetch all posts
pub async fn get_posts() -> Result<Vec<Post>, ApiServiceError> {
    let response = Request::get("/api/posts")
        .send()
        .await?;
    if response.ok() {
        response.json().await.map_err(ApiServiceError::from)
    } else {
        Err(ApiServiceError::ServerError(response.status()))
    }
}

// Create a new post
pub async fn _create_post(new_post: &Post) -> Result<Post, ApiServiceError> {
    let body = serde_json::to_string(new_post)?;
    let response = Request::post("/api/posts")
        .header("Content-Type", "application/json")
        .body(body)?
        .send()
        .await?;

    if response.ok() {
        response.json().await.map_err(ApiServiceError::from)
    } else {
        Err(ApiServiceError::ServerError(response.status()))
    }
}

// Update an existing post
pub async fn update_post(post_id: i32, updated_post: &Post) -> Result<Post, ApiServiceError> {
    let body = serde_json::to_string(updated_post)?;
    let url = format!("/api/posts/{}", post_id);
    let response = Request::put(&url)
        .header("Content-Type", "application/json")
        .body(body)?
        .send()
        .await?;

    if response.ok() {
        response.json().await.map_err(ApiServiceError::from)
    } else {
        Err(ApiServiceError::ServerError(response.status()))
    }
}

// Delete a post
pub async fn _delete_post(post_id: i32) -> Result<(), ApiServiceError> {
    let url = format!("/api/posts/{}", post_id);
    let response = Request::delete(&url).send().await?;

    if response.ok() {
        Ok(())
    } else {
        Err(ApiServiceError::ServerError(response.status()))
    }
}

// Login API call
pub async fn login(auth_data: AuthData) -> Result<String, ApiServiceError> {
    let body = serde_json::to_string(&auth_data)?;
    let response = Request::post("/api/auth/login")
        .header("Content-Type", "application/json")
        .body(body)?
        .send()
        .await?;

    if response.ok() {
        response.text().await.map_err(ApiServiceError::from)
    } else {
        Err(ApiServiceError::ServerError(response.status()))
    }
}
