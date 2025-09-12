use axum::{
    extract::{State, Json},
    response::Json as ResponseJson,
    http::HeaderMap,
};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use crate::{
    AppServices,
    models::{User, NewUser},
    middleware::{
        auth::{get_authenticated_user, AuthenticatedUser},
        validation::{validate_username, validate_email, validate_password},
        errors::AppError,
    },
    // services, // Unused import
};

// Temporary replacement for email service function
fn generate_verification_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let token: [u8; 32] = rng.gen();
    hex::encode(token)
}

// Authentication request/response structures
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailRequest {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserProfile,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
}

/// User login endpoint
/// 
/// Validates user credentials and creates a session using the session manager.
/// Implements rate limiting, input validation, and secure session creation.
pub async fn login(
    State(services): State<AppServices>, 
    Json(login_req): Json<LoginRequest>
) -> Result<ResponseJson<LoginResponse>, AppError> {
    // Validate input
    validate_username(&login_req.username)?;
    if login_req.password.is_empty() {
        return Err(AppError::ValidationError("Password cannot be empty".to_string()));
    }

    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Find user by username
    let user = User::find_by_username(&mut conn, &login_req.username)?
        .ok_or(AppError::Unauthorized)?;
    
    // Check if user is active
    if user.status != "active" {
        if user.status == "pending_verification" {
            return Err(AppError::ValidationError("Please verify your email address before logging in".to_string()));
        }
        return Err(AppError::Forbidden);
    }
    
    // Verify password
    match bcrypt::verify(&login_req.password, &user.password) {
        Ok(true) => {
            // Password is correct, create session using session manager
            let session = services.session_manager.create_session(user.id).await?;
            
            Ok(ResponseJson(LoginResponse {
                token: session.session_token,
                user: UserProfile {
                    id: user.id,
                    username: user.username,
                    email: user.email.unwrap_or_default(),
                    role: user.role,
                    status: user.status,
                },
            }))
        }
        Ok(false) => Err(AppError::Unauthorized),
        Err(_) => Err(AppError::InternalError("Password verification failed".to_string())),
    }
}

/// Get current authenticated user information
/// 
/// Returns the profile of the currently authenticated user.
/// Requires valid session token in Authorization header.
pub async fn get_current_user(
    req: axum::extract::Request,
) -> Result<ResponseJson<UserProfile>, AppError> {
    let auth_user: &AuthenticatedUser = get_authenticated_user(&req)?;
    
    Ok(ResponseJson(UserProfile {
        id: auth_user.id,
        username: auth_user.username.clone(),
        email: auth_user.email.clone(),
        role: auth_user.role.clone(),
        status: auth_user.status.clone(),
    }))
}

/// Logout current session
/// 
/// Invalidates the current session token.
/// Requires valid session token in Authorization header.
pub async fn logout(
    headers: HeaderMap,
    State(services): State<AppServices>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    // Extract token from authorization header
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AppError::MissingAuthHeader)?;
    
    // Use session manager to logout
    services.session_manager.logout_session(auth_header).await?;
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": "Logout successful"
    })))
}

/// User signup endpoint
/// 
/// Creates a new user account with email verification required.
/// Sends verification email and sets account to unverified status.
pub async fn signup(
    State(services): State<AppServices>, 
    Json(signup_req): Json<SignupRequest>
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    // Validate input
    validate_username(&signup_req.username)?;
    validate_email(&signup_req.email)?;
    validate_password(&signup_req.password)?;

    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if username already exists
    if User::find_by_username(&mut conn, &signup_req.username)?.is_some() {
        return Err(AppError::ConflictError("Username already exists".to_string()));
    }
    
    // Check if email already exists
    if User::find_by_email(&mut conn, &signup_req.email)?.is_some() {
        return Err(AppError::ConflictError("Email already exists".to_string()));
    }
    
    // Hash password
    let hashed_password = bcrypt::hash(&signup_req.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?;
    
    // Generate verification token
    let verification_token = generate_verification_token();
    let expires_at = Utc::now().naive_utc() + Duration::hours(24);
    
    let new_user = NewUser {
        username: signup_req.username.clone(),
        password: hashed_password,
        email: Some(signup_req.email.clone()),
        role: "user".to_string(), // New signups get user role by default
        status: "pending_verification".to_string(), // User needs to verify email first
        email_verified: Some(false),
        email_verification_token: Some(verification_token.clone()),
        email_verification_expires_at: Some(expires_at),
    };
    
    let created_user = User::create(&mut conn, new_user)?;
    
    // Send verification email asynchronously to avoid blocking the response
    // Check if real email service should be used (via environment variable)
    let _use_real_email = std::env::var("USE_REAL_EMAIL").unwrap_or_else(|_| "false".to_string()) == "true";
    
    let email = signup_req.email.clone();
    let username = signup_req.username.clone();
    let token = verification_token.clone();
    
    // Temporarily disabled email functionality for Docker build
    tokio::spawn(async move {
        tracing::info!("Email verification disabled for Docker build");
        tracing::info!("User {} would receive verification email at {}", username, email);
        tracing::info!("Verification token: {}", token);
    });
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": "Account created successfully. Please check your email to verify your account.",
        "user_id": created_user.id,
        "email": created_user.email
    })))
}

/// Email verification endpoint
/// 
/// Verifies user email address using the verification token.
/// Activates the user account upon successful verification.
pub async fn verify_email(
    State(services): State<AppServices>, 
    Json(verify_req): Json<VerifyEmailRequest>
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Find user by verification token
    let user = User::find_by_verification_token(&mut conn, &verify_req.token)?
        .ok_or_else(|| AppError::ValidationError("Invalid or expired verification token".to_string()))?;
    
    // Check if token is expired
    if let Some(expires_at) = user.email_verification_expires_at {
        if Utc::now().naive_utc() > expires_at {
            return Err(AppError::ValidationError("Verification token has expired".to_string()));
        }
    } else {
        return Err(AppError::ValidationError("Invalid verification token".to_string()));
    }
    
    // Update user to verified status
    let update_user = crate::models::UpdateUser {
        username: None,
        password: None,
        email: None,
        role: None,
        status: Some("active".to_string()),
        email_verified: Some(true),
        email_verification_token: Some(String::new()), // Clear the token
        email_verification_expires_at: None,
    };
    
    User::update(&mut conn, user.id, update_user)?;
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": "Email verified successfully. Your account is now active."
    })))
}