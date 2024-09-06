use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::window;
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub db_url: String,
    pub db_pool_size: u32,
}

impl DatabaseConfig {
    /// Fetches database configuration values from environment variables or defaults.
    pub fn from_env() -> Result<DatabaseConfig, JsValue> {
        let window = window().unwrap();
        let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            window
                .location()
                .origin()
                .unwrap_or_else(|_| "https://localhost/db".to_string())
        });

        let db_pool_size = env::var("DATABASE_POOL_SIZE")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u32>()
            .map_err(|_| JsValue::from_str("Failed to parse DATABASE_POOL_SIZE as u32"))?;

        Ok(DatabaseConfig { db_url, db_pool_size })
    }

    /// Mock function for establishing a database connection in WASM
    pub fn mock_database_connect() {
        web_sys::console::log_1(&"Database connection initialized (mocked for WASM)".into());
    }
}
