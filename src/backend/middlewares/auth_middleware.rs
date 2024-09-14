use axum::{
    extract::{Extension, TypedHeader},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    Request,
};
use crate::services::auth_service::AuthService;
use headers::Authorization;
use std::sync::Arc;

// Define the AuthError for handling authorization failures
#[derive(Debug)]
struct AuthError;

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }
}

// Auth middleware using Axum's Extension to pass AuthService and validate tokens
async fn auth_middleware<B>(
    TypedHeader(auth_header): TypedHeader<Authorization<String>>, // Extract Authorization header
    Extension(auth_service): Extension<Arc<AuthService>>, // Extract AuthService from the extension
    req: Request<B>,
    next: Next<B>,
) -> impl IntoResponse {
    // Validate the token using AuthService
    if let Ok(_) = auth_service.validate_token(&auth_header.0) {
        next.run(req).await // If valid, proceed to the next middleware/handler
    } else {
        AuthError.into_response() // If invalid, return an Unauthorized response
    }
}

// Define the middleware stack
pub fn with_auth() -> impl tower::Layer<axum::Router> + Clone {
    tower::ServiceBuilder::new()
        .layer(axum::middleware::from_fn(auth_middleware))
}

// Example of integrating the middleware with Axum's router
pub fn init_routes() -> axum::Router {
    axum::Router::new()
        .route("/api/protected_route", axum::routing::get(protected_handler))
        .layer(with_auth()) // Apply the auth middleware
}

// Example handler for a protected route
async fn protected_handler() -> impl IntoResponse {
    "Access granted to protected route"
}
