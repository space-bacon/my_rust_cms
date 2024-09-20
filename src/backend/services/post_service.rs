use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use gloo_net::http::Request;
use log::{error, info};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
}

#[derive(Debug, Error)]
pub enum PostServiceError {
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("API error with status code: {0}")]
    ApiError(u16),
}

impl From<gloo_net::Error> for PostServiceError {
    fn from(error: gloo_net::Error) -> Self {
        PostServiceError::NetworkError(error.to_string())
    }
}

impl From<serde_json::Error> for PostServiceError {
    fn from(error: serde_json::Error) -> Self {
        PostServiceError::DeserializationError(error.to_string())
    }
}

impl From<String> for PostServiceError {
    fn from(error: String) -> Self {
        PostServiceError::SerializationError(error)
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = API_BASE_URL)]
    static API_BASE_URL: JsValue;
}

fn get_api_base_url() -> String {
    API_BASE_URL.as_string().unwrap_or_else(|| "".to_string())
}

enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpMethod {
    fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
        }
    }
}

async fn make_request(
    method: HttpMethod,
    endpoint: &str,
    body: Option<impl Into<JsValue>>,
) -> Result<gloo_net::http::Response, PostServiceError> {
    let url = format!("{}{}", get_api_base_url(), endpoint);
    let mut request = Request::new(&url).method(method.as_str());

    if let Some(body) = body {
        request = request
            .header("Content-Type", "application/json")
            .body(body);
    }

    let response = request.send().await.map_err(PostServiceError::from)?;

    if response.ok() {
        Ok(response)
    } else {
        Err(PostServiceError::ApiError(response.status()))
    }
}

async fn handle_api_response<T>(response: gloo_net::http::Response) -> Result<T, PostServiceError>
where
    T: for<'de> Deserialize<'de>,
{
    response
        .json::<T>()
        .await
        .map_err(PostServiceError::from)
}

// Fetch all posts
pub async fn fetch_posts() -> Result<Vec<Post>, PostServiceError> {
    info!("Fetching all posts...");
    let response = make_request(HttpMethod::GET, "/api/posts", None::<JsValue>).await?;
    handle_api_response::<Vec<Post>>(response).await
}

// Create a new post
pub async fn create_post(new_post: &Post) -> Result<Post, PostServiceError> {
    info!("Creating new post: {:?}", new_post);
    let body = serde_json::to_string(new_post).map_err(PostServiceError::from)?;

    let response = make_request(HttpMethod::POST, "/api/posts", Some(body)).await?;
    handle_api_response::<Post>(response).await
}

// Update an existing post
pub async fn update_post(post_id: i32, updated_post: &Post) -> Result<Post, PostServiceError> {
    info!("Updating post ID {}: {:?}", post_id, updated_post);
    let endpoint = format!("/api/posts/{}", post_id);
    let body = serde_json::to_string(updated_post).map_err(PostServiceError::from)?;

    let response = make_request(HttpMethod::PUT, &endpoint, Some(body)).await?;
    handle_api_response::<Post>(response).await
}

// Delete a post
pub async fn delete_post(post_id: i32) -> Result<(), PostServiceError> {
    info!("Deleting post with ID {}", post_id);
    let endpoint = format!("/api/posts/{}", post_id);

    let response = make_request(HttpMethod::DELETE, &endpoint, None::<JsValue>).await?;

    if response.ok() {
        Ok(())
    } else {
        Err(PostServiceError::ApiError(response.status()))
    }
}
