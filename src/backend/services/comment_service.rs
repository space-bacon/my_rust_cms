use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use reqwasm::http::Request;
use std::fmt;
use thiserror::Error;
use web_sys::console;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Comment {
    pub id: Option<i32>,
    pub post_id: i32,
    pub author: String,
    pub content: String,
    pub created_at: String,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = API_BASE_URL)]
    static API_BASE_URL: JsValue;
}

fn get_api_base_url() -> String {
    API_BASE_URL
        .as_string()
        .unwrap_or_else(|| "http://localhost:8080".to_string())
}

#[derive(Debug, Error)]
pub enum CommentError {
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Unknown error occurred")]
    UnknownError,
}

impl From<reqwasm::Error> for CommentError {
    fn from(err: reqwasm::Error) -> Self {
        CommentError::RequestError(err.to_string())
    }
}

impl From<JsValue> for CommentError {
    fn from(_: JsValue) -> Self {
        CommentError::UnknownError
    }
}

enum HttpMethod {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
}

impl HttpMethod {
    fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PUT => "PUT",
            HttpMethod::PATCH => "PATCH",
        }
    }
}

async fn make_request(
    method: HttpMethod,
    url: &str,
    body: Option<impl Into<JsValue>>,
) -> Result<reqwasm::Response, CommentError> {
    let mut request = Request::new(url).method(method.as_str());

    if let Some(body) = body {
        request = request
            .header("Content-Type", "application/json")
            .body(body);
    }

    let response = request.send().await.map_err(CommentError::from)?;

    if response.ok() {
        Ok(response)
    } else {
        Err(CommentError::RequestError(format!(
            "Failed with status: {}",
            response.status()
        )))
    }
}

/// Sends a POST request to save a new comment on the server.
pub async fn save_comment(comment: Comment) -> Result<(), CommentError> {
    let url = format!("{}/api/comments", get_api_base_url());
    let body = serde_json::to_string(&comment)
        .map_err(|e| CommentError::ParseError(e.to_string()))?;

    let result = make_request(HttpMethod::POST, &url, Some(body)).await;

    match result {
        Ok(_) => {
            console::log_1(&"Comment saved successfully!".into());
            Ok(())
        }
        Err(e) => {
            console::error_1(&format!("Failed to save comment: {}", e).into());
            Err(e)
        }
    }
}

/// Fetches all comments related to a specific post.
pub async fn fetch_comments(post_id: i32) -> Result<Vec<Comment>, CommentError> {
    let url = format!("{}/api/comments?post_id={}", get_api_base_url(), post_id);

    let response = make_request(HttpMethod::GET, &url, None::<JsValue>).await?;
    let comments = response
        .json::<Vec<Comment>>()
        .await
        .map_err(|e| CommentError::ParseError(e.to_string()))?;

    Ok(comments)
}

/// Deletes a comment by sending a DELETE request to the server.
pub async fn delete_comment(id: i32) -> Result<(), CommentError> {
    let url = format!("{}/api/comments/{}", get_api_base_url(), id);

    let result = make_request(HttpMethod::DELETE, &url, None::<JsValue>).await;

    match result {
        Ok(_) => {
            console::log_1(&"Comment deleted successfully!".into());
            Ok(())
        }
        Err(e) => {
            console::error_1(&format!("Failed to delete comment: {}", e).into());
            Err(e)
        }
    }
}
