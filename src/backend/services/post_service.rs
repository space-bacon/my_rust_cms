use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use gloo_net::http::Request;
use log::{error, info}; // Add logging for better insights.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Debug)]
pub enum PostServiceError {
    NetworkError(String),
    SerializationError(String),
    DeserializationError(String),
    ApiError(u16),
}

impl From<gloo_net::Error> for PostServiceError {
    fn from(error: gloo_net::Error) -> Self {
        PostServiceError::NetworkError(format!("Network error: {:?}", error))
    }
}

impl From<serde_json::Error> for PostServiceError {
    fn from(error: serde_json::Error) -> Self {
        PostServiceError::DeserializationError(format!("JSON error: {:?}", error))
    }
}

impl From<String> for PostServiceError {
    fn from(error: String) -> Self {
        PostServiceError::SerializationError(error)
    }
}

// Helper function to handle API responses
async fn handle_api_response<T>(response: gloo_net::http::Response) -> Result<T, PostServiceError>
where
    T: for<'de> Deserialize<'de>,
{
    if response.ok() {
        response
            .json::<T>()
            .await
            .map_err(PostServiceError::from)
    } else {
        Err(PostServiceError::ApiError(response.status()))
    }
}

// Fetch all posts
pub async fn fetch_posts() -> Result<Vec<Post>, PostServiceError> {
    info!("Fetching all posts...");
    let response = Request::get("/api/posts")
        .send()
        .await?;
        
    handle_api_response::<Vec<Post>>(response).await
}

// Create a new post
pub async fn create_post(new_post: &Post) -> Result<Post, PostServiceError> {
    info!("Creating new post: {:?}", new_post);
    let body = serde_json::to_string(new_post).map_err(PostServiceError::from)?;

    let response = Request::post("/api/posts")
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    handle_api_response::<Post>(response).await
}

// Update an existing post
pub async fn update_post(post_id: i32, updated_post: &Post) -> Result<Post, PostServiceError> {
    info!("Updating post ID {}: {:?}", post_id, updated_post);
    let url = format!("/api/posts/{}", post_id);
    let body = serde_json::to_string(updated_post).map_err(PostServiceError::from)?;

    let response = Request::put(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    handle_api_response::<Post>(response).await
}

// Delete a post
pub async fn delete_post(post_id: i32) -> Result<(), PostServiceError> {
    info!("Deleting post with ID {}", post_id);
    let url = format!("/api/posts/{}", post_id);

    let response = Request::delete(&url)
        .send()
        .await?;

    if response.ok() {
        Ok(())
    } else {
        Err(PostServiceError::ApiError(response.status()))
    }
}
