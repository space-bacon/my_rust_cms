use tower_http::cors::{CorsLayer, Any};
use http::Method;

pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<http::HeaderValue>().unwrap()) // Allow only specific origins
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])    // Allowed HTTP methods
        .allow_headers([http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE]) // Allowed headers
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(3600)) // CORS preflight cache duration
}
