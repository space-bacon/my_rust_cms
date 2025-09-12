use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{header, HeaderValue},
};
use crate::config::Config;

/// Middleware to add security headers for production deployment
pub async fn security_headers_middleware(
    axum::extract::State(config): axum::extract::State<Config>,
    req: Request,
    next: Next,
) -> Response {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    // Add security headers for production
    if !config.is_development() {
        // HSTS (HTTP Strict Transport Security) - force HTTPS
        headers.insert(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains; preload")
        );
        
        // Enforce HTTPS by redirecting HTTP traffic
        // Note: This would typically be handled by a reverse proxy in production
        // But we add the header for defense in depth
    }

    // X-Content-Type-Options: Prevent MIME type sniffing
    headers.insert(
        header::HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff")
    );

    // X-Frame-Options: Prevent clickjacking
    headers.insert(
        header::HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY")
    );

    // X-XSS-Protection: Enable XSS filtering (legacy browsers)
    headers.insert(
        header::HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("1; mode=block")
    );

    // Referrer-Policy: Control referrer information
    headers.insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin")
    );

    // Content-Security-Policy: Prevent XSS and data injection
    let csp = if config.is_development() {
        // More permissive CSP for development
        "default-src 'self' 'unsafe-inline' 'unsafe-eval' http://localhost:* http://127.0.0.1:*; \
         img-src 'self' data: http://localhost:* http://127.0.0.1:*; \
         font-src 'self' data:; \
         connect-src 'self' http://localhost:* http://127.0.0.1:*"
    } else {
        // Strict CSP for production
        "default-src 'self'; \
         script-src 'self' 'wasm-unsafe-eval'; \
         style-src 'self' 'unsafe-inline'; \
         img-src 'self' data:; \
         font-src 'self' data:; \
         connect-src 'self'; \
         frame-ancestors 'none'; \
         base-uri 'self'; \
         form-action 'self'"
    };
    
    headers.insert(
        header::HeaderName::from_static("content-security-policy"),
        HeaderValue::from_str(csp).unwrap_or_else(|_| HeaderValue::from_static("default-src 'self'"))
    );

    // Permissions-Policy: Control browser features
    headers.insert(
        header::HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("geolocation=(), microphone=(), camera=()")
    );

    response
}

/// HTTPS redirect middleware for production
#[allow(dead_code)]
pub async fn https_redirect_middleware(
    config: Config,
    req: Request,
    next: Next,
) -> Response {
    // In production, redirect HTTP to HTTPS
    // Note: This is usually handled by a reverse proxy (nginx, Cloudflare, etc.)
    // This is a backup security measure
    
    if !config.is_development() {
        // Check if the request is HTTP in production
        // This would need to be implemented based on your deployment setup
        // For now, we'll rely on the HSTS header and reverse proxy configuration
    }
    
    next.run(req).await
}