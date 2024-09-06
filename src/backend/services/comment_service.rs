use std::env;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use reqwasm::http::Request;
use web_sys::console;
use yew::Callback;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Comment {
    pub id: Option<i32>,
    pub post_id: i32,
    pub author: String,
    pub content: String,
    pub created_at: String,
}

/// Retrieves the API base URL from the environment, with a fallback to localhost.
fn get_api_base_url() -> String {
    env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string())
}

/// Sends a POST request to save a new comment on the server.
pub async fn save_comment(comment: Comment) -> Result<(), JsValue> {
    let url = format!("{}/api/comments", get_api_base_url());

    let response = Request::post(&url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&comment).unwrap())
        .send()
        .await;

    match response {
        Ok(res) if res.ok() => {
            console::log_1(&"Comment saved successfully!".into());
            Ok(())
        }
        Ok(res) => {
            let error_message = format!("Failed to save comment: {:?}", res);
            console::log_1(&error_message.into());
            Err(JsValue::from_str(&error_message))
        }
        Err(err) => {
            let error_message = format!("Request error: {:?}", err);
            console::log_1(&error_message.into());
            Err(JsValue::from_str(&error_message))
        }
    }
}

/// Fetches all comments related to a specific post by sending a GET request.
pub fn fetch_comments(
    post_id: i32,
    on_success: Callback<Vec<Comment>>,
    on_failure: Callback<JsValue>,
) {
    let url = format!("{}/api/comments?post_id={}", get_api_base_url(), post_id);

    wasm_bindgen_futures::spawn_local(async move {
        let response = Request::get(&url).send().await;

        match response {
            Ok(res) if res.ok() => {
                let json_result = res.json::<Vec<Comment>>().await;
                match json_result {
                    Ok(comments) => on_success.emit(comments),
                    Err(err) => {
                        let error_message = format!("Failed to parse comments: {:?}", err);
                        console::log_1(&error_message.into());
                        on_failure.emit(JsValue::from_str(&error_message));
                    }
                }
            }
            Ok(res) => {
                let error_message = format!("Failed to fetch comments: {:?}", res);
                console::log_1(&error_message.into());
                on_failure.emit(JsValue::from_str(&error_message));
            }
            Err(err) => {
                let error_message = format!("Request error: {:?}", err);
                console::log_1(&error_message.into());
                on_failure.emit(JsValue::from_str(&error_message));
            }
        }
    });
}

/// Deletes a comment by sending a DELETE request to the server.
pub async fn delete_comment(id: i32) -> Result<(), JsValue> {
    let url = format!("{}/api/comments/{}", get_api_base_url(), id);

    let response = Request::delete(&url).send().await;

    match response {
        Ok(res) if res.ok() => {
            console::log_1(&"Comment deleted successfully!".into());
            Ok(())
        }
        Ok(res) => {
            let error_message = format!("Failed to delete comment: {:?}", res);
            console::log_1(&error_message.into());
            Err(JsValue::from_str(&error_message))
        }
        Err(err) => {
            let error_message = format!("Request error: {:?}", err);
            console::log_1(&error_message.into());
            Err(JsValue::from_str(&error_message))
        }
    }
}
