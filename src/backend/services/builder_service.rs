use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use reqwasm::http::Request;
use std::fmt;
use thiserror::Error;
use web_sys::console;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CMSComponent {
    pub id: Option<i32>,
    pub name: String,
    pub html: String,
    pub css: String,
    pub js: String,
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
pub enum BuilderError {
    #[error("Request error: {0}")]
    RequestError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Unknown error occurred")]
    UnknownError,
}

impl From<reqwasm::Error> for BuilderError {
    fn from(err: reqwasm::Error) -> Self {
        BuilderError::RequestError(err.to_string())
    }
}

impl From<JsValue> for BuilderError {
    fn from(_: JsValue) -> Self {
        BuilderError::UnknownError
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
) -> Result<reqwasm::Response, BuilderError> {
    let mut request = Request::new(url).method(method.as_str());

    if let Some(body) = body {
        request = request
            .header("Content-Type", "application/json")
            .body(body);
    }

    let response = request.send().await.map_err(BuilderError::from)?;

    if response.ok() {
        Ok(response)
    } else {
        Err(BuilderError::RequestError(format!(
            "Failed with status: {}",
            response.status()
        )))
    }
}

/// Saves a CMS component
pub async fn save_component(component: CMSComponent) -> Result<(), BuilderError> {
    let url = format!("{}/api/components", get_api_base_url());
    let body = serde_json::to_string(&component)
        .map_err(|e| BuilderError::ParseError(e.to_string()))?;

    let result = make_request(HttpMethod::POST, &url, Some(body)).await;

    match result {
        Ok(_) => {
            console::log_1(&"Component saved successfully!".into());
            Ok(())
        }
        Err(e) => {
            console::error_1(&format!("Failed to save component: {}", e).into());
            Err(e)
        }
    }
}

/// Fetches all CMS components
pub async fn fetch_components() -> Result<Vec<CMSComponent>, BuilderError> {
    let url = format!("{}/api/components", get_api_base_url());

    let response = make_request(HttpMethod::GET, &url, None::<JsValue>).await?;
    let components = response
        .json::<Vec<CMSComponent>>()
        .await
        .map_err(|e| BuilderError::ParseError(e.to_string()))?;

    Ok(components)
}

/// Deletes a CMS component by ID
pub async fn delete_component(id: i32) -> Result<(), BuilderError> {
    let url = format!("{}/api/components/{}", get_api_base_url(), id);

    let result = make_request(HttpMethod::DELETE, &url, None::<JsValue>).await;

    match result {
        Ok(_) => {
            console::log_1(&"Component deleted successfully!".into());
            Ok(())
        }
        Err(e) => {
            console::error_1(&format!("Failed to delete component: {}", e).into());
            Err(e)
        }
    }
}
