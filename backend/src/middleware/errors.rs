use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug)]
pub enum AppError {
    // Authentication errors
    Unauthorized,
    InvalidToken,
    ExpiredToken,
    MissingAuthHeader,
    
    // Authorization errors
    Forbidden,
    InsufficientPermissions,
    
    // Validation errors
    ValidationError(String),
    InvalidInput(String),
    
    // Database errors
    DatabaseError(String),
    DatabaseConnection(String),
    DatabaseQuery(String),
    NotFound(String),
    
    // Business logic errors
    ConflictError(String),
    BadRequest(String),
    
    // System errors
    InternalError(String),
    InternalServerError(String),
    ExternalServiceError(String),
    Configuration(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Unauthorized => write!(f, "Authentication required"),
            AppError::InvalidToken => write!(f, "Invalid authentication token"),
            AppError::ExpiredToken => write!(f, "Authentication token has expired"),
            AppError::MissingAuthHeader => write!(f, "Missing authorization header"),
            AppError::Forbidden => write!(f, "Access denied"),
            AppError::InsufficientPermissions => write!(f, "Insufficient permissions"),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::DatabaseConnection(msg) => write!(f, "Database connection error: {}", msg),
            AppError::DatabaseQuery(msg) => write!(f, "Database query error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::ConflictError(msg) => write!(f, "Conflict: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            AppError::Configuration(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match &self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", "Authentication required"),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "INVALID_TOKEN", "Invalid authentication token"),
            AppError::ExpiredToken => (StatusCode::UNAUTHORIZED, "EXPIRED_TOKEN", "Authentication token has expired"),
            AppError::MissingAuthHeader => (StatusCode::UNAUTHORIZED, "MISSING_AUTH_HEADER", "Missing authorization header"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "FORBIDDEN", "Access denied"),
            AppError::InsufficientPermissions => (StatusCode::FORBIDDEN, "INSUFFICIENT_PERMISSIONS", "Insufficient permissions"),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg.as_str()),
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, "INVALID_INPUT", msg.as_str()),
            AppError::DatabaseError(msg) => {
                // Detect unique violation to surface 409 instead of 500
                if msg.contains("unique") || msg.contains("UNIQUE") || msg.contains("duplicate key value violates unique constraint") {
                    (StatusCode::CONFLICT, "CONFLICT", "Unique constraint violation")
                } else {
                    (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_ERROR", "Internal server error")
                }
            }
            AppError::DatabaseConnection(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_CONNECTION_ERROR", "Database connection error"),
            AppError::DatabaseQuery(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE_QUERY_ERROR", "Database query error"),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg.as_str()),
            AppError::ConflictError(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.as_str()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.as_str()),
            AppError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "Internal server error"),
            AppError::InternalServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR", "Internal server error"),
            AppError::ExternalServiceError(_) => (StatusCode::SERVICE_UNAVAILABLE, "EXTERNAL_SERVICE_ERROR", "External service unavailable"),
            AppError::Configuration(_) => (StatusCode::INTERNAL_SERVER_ERROR, "CONFIGURATION_ERROR", "Configuration error"),
        };

        let api_error = ApiError {
            code: error_code.to_string(),
            message: message.to_string(),
            details: match &self {
                AppError::ValidationError(msg) | AppError::InvalidInput(msg) | AppError::ConflictError(msg) | AppError::NotFound(msg) | AppError::BadRequest(msg) => {
                    Some(serde_json::json!({ "error": msg }))
                }
                AppError::InternalError(msg) | AppError::InternalServerError(msg) | AppError::DatabaseError(msg) | AppError::Configuration(msg) => {
                    Some(serde_json::json!({ "error": msg }))
                }
                _ => None,
            },
        };

        (status, Json(api_error)).into_response()
    }
}

// Conversion from diesel errors
impl From<diesel::result::Error> for AppError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::DatabaseError(err.to_string()),
        }
    }
}

// Result type alias for our API
pub type ApiResult<T> = Result<T, AppError>;

// Standard JSON response wrapper
#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ResponseJson<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
}

#[allow(dead_code)]
impl<T> ResponseJson<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
        }
    }

    pub fn success_with_message(data: T, message: String) -> Self {
        Self {
            success: true,
            data,
            message: Some(message),
        }
    }
}