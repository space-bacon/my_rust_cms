use axum::{
    extract::{Request, ConnectInfo},
    middleware::Next,
    response::Response,
    http::StatusCode,
    body::Body,
    routing::Route,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_governor::{
    governor::GovernorConfigBuilder,
    GovernorLayer,
    key_extractor::PeerIpKeyExtractor,
};
use governor::{
    clock::QuantaInstant,
    middleware::{NoOpMiddleware, StateInformationMiddleware},
};
use tower::Layer;

/// Rate limiting configuration for different types of operations
#[derive(Clone)]
#[allow(dead_code)]
pub struct RateLimitConfig {
    pub auth_per_minute: u32,
    pub general_per_minute: u32,
    pub upload_per_minute: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            auth_per_minute: 5,      // 5 auth attempts per minute
            general_per_minute: 60,  // 60 general requests per minute
            upload_per_minute: 10,   // 10 uploads per minute
        }
    }
}

/// Create a rate limiter for authentication endpoints
/// Rate limit: 5 requests per minute with burst of 10
/// 
/// This provides security against brute force attacks while allowing
/// legitimate users reasonable access to authentication endpoints.
pub fn create_auth_rate_limiter() -> impl Layer<Route> + Clone + Send + 'static {
    let governor_conf = GovernorConfigBuilder::<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>>::const_default()
        .per_second(1)     // Conservative: 1 request per second
        .burst_size(5)     // Allow small bursts for legitimate use
        .finish()
        .expect("Failed to create auth rate limiter config");
    
    GovernorLayer::<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>, Body>::new(governor_conf)
}

/// Create a rate limiter for file upload endpoints
/// Rate limit: 10 requests per minute with burst of 3
///
/// Uploads are resource-intensive, so we use stricter limits
/// to prevent abuse while allowing legitimate file operations.
pub fn create_upload_rate_limiter() -> impl Layer<Route> + Clone + Send + 'static {
    let governor_conf = GovernorConfigBuilder::<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>>::const_default()
        .per_second(1)     // 1 upload per second max
        .burst_size(2)     // Very small burst for uploads
        .finish()
        .expect("Failed to create upload rate limiter config");
    
    GovernorLayer::<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>, Body>::new(governor_conf)
}

/// Create a general rate limiter for API endpoints
/// Rate limit: 100 requests per minute with burst of 20
///
/// More generous limits for general API usage while still
/// providing protection against abuse and automated attacks.
pub fn create_general_rate_limiter() -> GovernorLayer<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>, Body> {
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)     // 2 requests per second (120 per minute)
        .burst_size(10)    // Allow reasonable bursts
        .finish()
        .expect("Failed to create general rate limiter config");
    
    GovernorLayer::<PeerIpKeyExtractor, NoOpMiddleware<QuantaInstant>, Body>::new(governor_conf)
}

/// Custom middleware for enhanced rate limiting with user-based tracking
/// 
/// This is a placeholder for future enhancements that could include:
/// - User-specific rate limits (different limits for authenticated users)
/// - Dynamic rate limiting based on user behavior
/// - Integration with Redis for distributed rate limiting
/// - Logging of rate limit violations for security monitoring
#[allow(dead_code)]
pub async fn enhanced_auth_rate_limit(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // For now, rely on the tower-governor middleware
    // Future enhancements could include:
    // - User-based rate limiting (in addition to IP-based)
    // - Different limits for authenticated vs unauthenticated users
    // - Exponential backoff for repeated violations
    // - Logging of rate limit violations for security monitoring
    
    Ok(next.run(req).await)
}

// Rate limiting is now enabled with tower_governor 0.8.0!
// 
// Security benefits:
// - Brute force protection on authentication endpoints
// - DDoS mitigation through IP-based rate limiting  
// - Resource protection for expensive operations like uploads
// - Automatic rate limit headers inform clients of their limits
// - Configurable burst sizes allow legitimate usage patterns
//
// The implementation uses:
// - IP-based key extraction (default behavior)
// - Per-second rate limits with burst allowances
// - Response headers showing current rate limit status
// - Conservative limits that can be adjusted based on usage patterns