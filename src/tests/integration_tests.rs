use wasm_bindgen_test::*;
use crate::backend::services::{auth_service::*, post_service::*};
use crate::config::Config;
use serde::{Deserialize, Serialize};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Serialize, Deserialize)]
struct LoginCredentials {
    username: String,
    password: String,
}

#[wasm_bindgen_test]
async fn test_login_successful() {
    let credentials = LoginCredentials {
        username: "test_user".to_string(),
        password: "test_password".to_string(),
    };
    let result = login(credentials).await;
    assert!(result.is_ok(), "Login should succeed with correct credentials.");
}

#[wasm_bindgen_test]
async fn test_login_failed() {
    let credentials = LoginCredentials {
        username: "wrong_user".to_string(),
        password: "wrong_password".to_string(),
    };
    let result = login(credentials).await;
    assert!(result.is_err(), "Login should fail with incorrect credentials.");
}

#[wasm_bindgen_test]
async fn test_fetch_posts() {
    let posts = fetch_posts().await.unwrap();
    assert!(!posts.is_empty(), "There should be posts available.");
}

#[wasm_bindgen_test]
async fn test_create_post() {
    let new_post = Post {
        id: 0,
        title: "New Post".to_string(),
        content: "This is a test post.".to_string(),
    };
    let created_post = create_post(&new_post).await.unwrap();
    assert_eq!(created_post.title, "New Post", "Post title should match.");
    assert_eq!(created_post.content, "This is a test post.", "Post content should match.");
}

#[wasm_bindgen_test]
fn test_config_loading() {
    let config = Config::from_env();
    let expected_url = std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    assert_eq!(config.api_base_url, expected_url, "API base URL should load correctly.");
}

#[wasm_bindgen_test]
async fn test_logout() {
    let result = logout().await;
    assert!(result.is_ok(), "Logout should succeed.");
}
