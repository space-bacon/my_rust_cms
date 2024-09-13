use argon2::{self, Config};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;
use crate::schema::users::dsl::{users, username as db_username, password_hash as db_password};
use diesel::PgConnection;
use js_sys::Error;

/// Fetch the salt from environment variables or fallback to a static value.
const DEFAULT_SALT: &[u8] = b"default-salt";
fn get_salt() -> Vec<u8> {
    std::env::var("SALT")
        .unwrap_or_else(|_| String::from_utf8(DEFAULT_SALT.to_vec()).unwrap())
        .into_bytes()
}

/// Custom error handling for authentication.
#[derive(Debug)]
enum AuthError {
    HashingError(String),
    VerificationError(String),
    InvalidCredentials,
    ServerError(String),
}

impl From<AuthError> for JsValue {
    fn from(error: AuthError) -> Self {
        let msg = match error {
            AuthError::HashingError(e) => format!("Hashing error: {}", e),
            AuthError::VerificationError(e) => format!("Verification error: {}", e),
            AuthError::InvalidCredentials => "Invalid credentials".to_string(),
            AuthError::ServerError(e) => format!("Server error: {}", e),
        };
        JsValue::from_str(&msg)
    }
}

// Hash password using Argon2 with environment-based salt
pub fn hash_password(password: &str) -> Result<String, JsValue> {
    let config = Config::default();
    let salt = get_salt();

    argon2::hash_encoded(password.as_bytes(), &salt, &config)
        .map_err(|e| {
            console::log_1(&JsValue::from_str(&format!("Hashing error: {:?}", e)));
            AuthError::HashingError(e.to_string()).into()
        })
}

// Verify password with detailed error handling
pub fn verify_password(hash: &str, password: &str) -> Result<bool, JsValue> {
    argon2::verify_encoded(hash, password.as_bytes())
        .map_err(|e| {
            console::log_1(&JsValue::from_str(&format!("Verification error: {:?}", e)));
            AuthError::VerificationError(e.to_string()).into()
        })
}

// Credentials structure for login
#[derive(Serialize, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

// Fetch password hash from the database
pub fn get_stored_hash(username_input: &str, conn: &PgConnection) -> Result<String, JsValue> {
    users.filter(db_username.eq(username_input))
        .select(db_password)
        .first::<String>(conn)
        .map_err(|_| AuthError::InvalidCredentials.into())
}

// Login function
#[wasm_bindgen]
pub async fn login(credentials: LoginCredentials, conn: PgConnection) -> Result<JsValue, JsValue> {
    let stored_hash = get_stored_hash(&credentials.username, &conn)?;

    if verify_password(&stored_hash, &credentials.password)? {
        Ok(JsValue::from_str("Login success"))
    } else {
        Err(AuthError::InvalidCredentials.into())
    }
}
