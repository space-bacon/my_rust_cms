use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{console, File, FormData};
use yew::Callback;
use reqwasm::http::Request;
use std::fmt;
use thiserror::Error;

/// Define custom error types for media operations
#[derive(Debug, Error)]
pub enum MediaError {
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Unknown error occurred")]
    UnknownError,
}

impl From<reqwasm::Error> for MediaError {
    fn from(err: reqwasm::Error) -> Self {
        MediaError::RequestError(format!("{:?}", err))
    }
}

impl From<JsValue> for MediaError {
    fn from(_: JsValue) -> Self {
        MediaError::UnknownError
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Media {
    pub id: Option<i32>,
    pub file_name: String,
    pub file_url: String,
    pub mime_type: String,
    pub uploaded_at: String,
}

/// Retrieves the API base URL from a global JavaScript variable.
/// Make sure to define `API_BASE_URL` in your JavaScript code.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = API_BASE_URL)]
    static API_BASE_URL: JsValue;
}

fn get_api_base_url() -> String {
    API_BASE_URL.as_string().unwrap_or_else(|| "http://localhost:8080".to_string())
}

/// Enum representing HTTP methods
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

/// Makes a request to the API
async fn make_request(
    method: HttpMethod,
    url: &str,
    body: Option<impl Into<JsValue>>,
) -> Result<reqwasm::Response, MediaError> {
    let mut request = Request::new(url).method(method.as_str());

    if let Some(body) = body {
        request = request.body(body);
    }

    let response = request.send().await.map_err(MediaError::from)?;

    if response.ok() {
        Ok(response)
    } else {
        Err(MediaError::RequestError(format!(
            "Failed with status: {}",
            response.status()
        )))
    }
}

/// Uploads new media using FormData
pub async fn upload_media(file: File) -> Result<(), MediaError> {
    let url = format!("{}/api/media", get_api_base_url());

    let form_data = FormData::new().map_err(|_| MediaError::UnknownError)?;
    form_data
        .append_with_blob("file", &file)
        .map_err(|_| MediaError::UnknownError)?;

    let result = make_request(HttpMethod::POST, &url, Some(form_data)).await;

    match result {
        Ok(_) => {
            console::log_1(&"Media uploaded successfully!".into());
            Ok(())
        }
        Err(e) => {
            console::error_1(&format!("Failed to upload media: {:?}", e).into());
            Err(e)
        }
    }
}

/// Fetches all media items from the server
pub async fn fetch_all_media() -> Result<Vec<Media>, MediaError> {
    let url = format!("{}/api/media", get_api_base_url());

    let response = make_request(HttpMethod::GET, &url, None::<JsValue>).await?;
    let media_items = response
        .json::<Vec<Media>>()
        .await
        .map_err(|e| MediaError::ParseError(e.to_string()))?;

    Ok(media_items)
}

/// Deletes media
pub async fn delete_media(id: i32) -> Result<(), MediaError> {
    let url = format!("{}/api/media/{}", get_api_base_url(), id);
    let result = make_request(HttpMethod::DELETE, &url, None::<JsValue>).await;

    match result {
        Ok(_) => {
            console::log_1(&"Media deleted successfully!".into());
            Ok(())
        }
        Err(e) => {
            console::error_1(&format!("Failed to delete media: {:?}", e).into());
            Err(e)
        }
    }
}
