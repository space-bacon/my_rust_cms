use serde::{Deserialize, Serialize};
use reqwasm::http::Request;
use wasm_bindgen::JsValue;
use std::env;
use web_sys::console;
use yew::Callback;

/// Define custom error types for media operations
#[derive(Debug)]
pub enum MediaError {
    RequestError(String),
    ParseError(String),
    UnknownError,
}

impl From<reqwasm::Error> for MediaError {
    fn from(err: reqwasm::Error) -> Self {
        MediaError::RequestError(format!("Request error: {:?}", err))
    }
}

impl From<JsValue> for MediaError {
    fn from(value: JsValue) -> Self {
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

/// Retrieves the API base URL from the environment, with a fallback to localhost.
fn get_api_base_url() -> String {
    env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string())
}

/// Common function to log error messages
fn log_error(error: &str) {
    console::error_1(&error.into());
}

/// Common function to log info messages
fn log_info(info: &str) {
    console::log_1(&info.into());
}

/// Makes a request to the API
async fn make_request(method: &str, url: &str, body: Option<&str>) -> Result<reqwasm::Response, MediaError> {
    let request = match method {
        "POST" => Request::post(url),
        "DELETE" => Request::delete(url),
        "GET" => Request::get(url),
        _ => return Err(MediaError::RequestError("Unsupported method".to_string())),
    };

    let response = if let Some(body) = body {
        request
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .map_err(MediaError::from)?
    } else {
        request.send().await.map_err(MediaError::from)?
    };

    if response.ok() {
        Ok(response)
    } else {
        Err(MediaError::RequestError(format!(
            "Failed with status: {}",
            response.status()
        )))
    }
}

/// Uploads new media
pub async fn upload_media(media: Media) -> Result<(), MediaError> {
    let url = format!("{}/api/media", get_api_base_url());
    let body = serde_json::to_string(&media).map_err(|e| MediaError::ParseError(e.to_string()))?;
    
    let result = make_request("POST", &url, Some(&body)).await;

    match result {
        Ok(_) => {
            log_info("Media uploaded successfully!");
            Ok(())
        }
        Err(e) => {
            log_error(&format!("Failed to upload media: {:?}", e));
            Err(e)
        }
    }
}

/// Fetches all media items from the server
pub fn fetch_all_media(
    on_success: Callback<Vec<Media>>,
    on_failure: Callback<MediaError>,
) {
    let url = format!("{}/api/media", get_api_base_url());

    wasm_bindgen_futures::spawn_local(async move {
        let result = make_request("GET", &url, None).await;

        match result {
            Ok(res) => {
                let json_result = res.json::<Vec<Media>>().await;
                match json_result {
                    Ok(media_items) => on_success.emit(media_items),
                    Err(err) => {
                        let error_message = MediaError::ParseError(format!("Failed to parse media items: {:?}", err));
                        log_error(&format!("{:?}", error_message));
                        on_failure.emit(error_message);
                    }
                }
            }
            Err(err) => {
                log_error(&format!("Failed to fetch media: {:?}", err));
                on_failure.emit(err);
            }
        }
    });
}

/// Deletes media
pub async fn delete_media(id: i32) -> Result<(), MediaError> {
    let url = format!("{}/api/media/{}", get_api_base_url(), id);
    let result = make_request("DELETE", &url, None).await;

    match result {
        Ok(_) => {
            log_info("Media deleted successfully!");
            Ok(())
        }
        Err(e) => {
            log_error(&format!("Failed to delete media: {:?}", e));
            Err(e)
        }
    }
}
