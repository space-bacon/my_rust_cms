use axum::{
    extract::Request,
    middleware::Next,
    response::{Response, IntoResponse},
    http::StatusCode,
};
use crate::middleware::errors::{AppError, ApiResult};
use regex::Regex;
use once_cell::sync::Lazy;

// Validation patterns
static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
});

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9_-]{3,50}$").unwrap()
});

#[allow(dead_code)]
static SAFE_TEXT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9\s\-_.,!?()]+$").unwrap()
});

// Input validation functions
pub fn validate_email(email: &str) -> ApiResult<()> {
    if email.is_empty() {
        return Err(AppError::ValidationError("Email cannot be empty".to_string()));
    }
    
    if email.len() > 254 {
        return Err(AppError::ValidationError("Email too long".to_string()));
    }
    
    if !EMAIL_REGEX.is_match(email) {
        return Err(AppError::ValidationError("Invalid email format".to_string()));
    }
    
    Ok(())
}

pub fn validate_username(username: &str) -> ApiResult<()> {
    if username.is_empty() {
        return Err(AppError::ValidationError("Username cannot be empty".to_string()));
    }
    
    if username.len() < 3 {
        return Err(AppError::ValidationError("Username must be at least 3 characters".to_string()));
    }
    
    if username.len() > 50 {
        return Err(AppError::ValidationError("Username too long".to_string()));
    }
    
    if !USERNAME_REGEX.is_match(username) {
        return Err(AppError::ValidationError(
            "Username can only contain letters, numbers, hyphens, and underscores".to_string()
        ));
    }
    
    Ok(())
}

pub fn validate_password(password: &str) -> ApiResult<()> {
    if password.is_empty() {
        return Err(AppError::ValidationError("Password cannot be empty".to_string()));
    }
    
    if password.len() < 8 {
        return Err(AppError::ValidationError("Password must be at least 8 characters".to_string()));
    }
    
    if password.len() > 128 {
        return Err(AppError::ValidationError("Password too long".to_string()));
    }
    
    // Check for at least one uppercase, one lowercase, and one digit
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    
    if !has_upper || !has_lower || !has_digit {
        return Err(AppError::ValidationError(
            "Password must contain at least one uppercase letter, one lowercase letter, and one digit".to_string()
        ));
    }
    
    Ok(())
}

#[allow(dead_code)]
pub fn sanitize_text_input(input: &str) -> String {
    // Remove HTML tags and dangerous characters
    input
        .trim()
        .replace(['<', '>', '"', '\'', '&'], "")
        .chars()
        .take(1000) // Limit length
        .collect()
}

pub fn validate_text_content(content: &str, max_length: usize) -> ApiResult<()> {
    if content.len() > max_length {
        return Err(AppError::ValidationError(format!(
            "Content too long. Maximum length is {} characters",
            max_length
        )));
    }
    
    // Check for potential XSS/injection patterns (more precise)
    let dangerous_patterns = [
        "<script", "javascript:", "vbscript:", "onload=", "onerror=", "onclick=",
        "drop table", "delete from", "insert into", "update set", "union select"
    ];
    
    let content_lower = content.to_lowercase();
    for pattern in &dangerous_patterns {
        if content_lower.contains(pattern) {
            return Err(AppError::ValidationError(
                "Content contains potentially dangerous patterns".to_string()
            ));
        }
    }
    
    // Check for standalone script tags or javascript execution
    if content_lower.contains("<script>") || 
       content_lower.contains("</script>") ||
       (content_lower.contains("script") && content_lower.contains("src=")) {
        return Err(AppError::ValidationError(
            "Content contains potentially dangerous patterns".to_string()
        ));
    }
    
    Ok(())
}

pub fn validate_file_upload(filename: &str, content_type: &str, size: usize) -> ApiResult<()> {
    // Check file size (10MB limit)
    const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;
    if size > MAX_FILE_SIZE {
        return Err(AppError::ValidationError("File size too large".to_string()));
    }
    
    // Validate filename
    if filename.is_empty() {
        return Err(AppError::ValidationError("Filename cannot be empty".to_string()));
    }
    
    if filename.len() > 255 {
        return Err(AppError::ValidationError("Filename too long".to_string()));
    }
    
    // Check for dangerous filename patterns
    if filename.contains("..") || filename.contains("/") || filename.contains("\\") {
        return Err(AppError::ValidationError("Invalid filename".to_string()));
    }
    
    // Validate allowed file types
    let allowed_types = [
        "image/jpeg", "image/png", "image/gif", "image/webp", "image/svg+xml",
        "text/plain", "application/pdf", "text/markdown",
        "application/msword", "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    ];
    
    if !allowed_types.contains(&content_type) {
        return Err(AppError::ValidationError("File type not allowed".to_string()));
    }
    
    Ok(())
}

// Rate limiting middleware (simple in-memory implementation)
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};


#[allow(dead_code)]
static RATE_LIMITER: Lazy<Mutex<HashMap<String, (u32, Instant)>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

#[allow(dead_code)]
pub async fn rate_limit_middleware(
    req: Request,
    next: Next,
) -> Response {
    let client_ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown")
        .to_string();
    
    let mut limiter = RATE_LIMITER.lock().unwrap();
    let now = Instant::now();
    
    // Clean up old entries (older than 1 minute)
    limiter.retain(|_, (_, timestamp)| now.duration_since(*timestamp) < Duration::from_secs(60));
    
    let (count, timestamp) = limiter.entry(client_ip).or_insert((0, now));
    
    // Reset count if more than 1 minute has passed
    if now.duration_since(*timestamp) >= Duration::from_secs(60) {
        *count = 0;
        *timestamp = now;
    }
    
    *count += 1;
    
    // Allow 100 requests per minute per IP
    if *count > 100 {
        drop(limiter); // Release the lock
        return (StatusCode::TOO_MANY_REQUESTS, "Rate limit exceeded").into_response();
    }
    
    drop(limiter); // Release the lock
    next.run(req).await
}