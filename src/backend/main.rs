// backend/src/main.rs

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;

use crate::controllers::{
    auth_controller,
    post_controller,
    media_controller,
    category_controller,
    builder_controller,
    settings_controller,
};
use crate::utils::{db::establish_connection_pool, auth::require_auth};

#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Establish database connection pool
    let db_pool = establish_connection_pool();

    // Build the application with routes and middleware
    let app = Router::new()
        // Authentication routes
        .nest(
            "/auth",
            auth_controller::routes(),
        )
        // Post routes (protected)
        .nest(
            "/posts",
            post_controller::routes()
                .layer(axum::middleware::from_fn(require_auth)),
        )
        // Media routes (protected)
        .nest(
            "/media",
            media_controller::routes()
                .layer(axum::middleware::from_fn(require_auth)),
        )
        // Category routes (protected)
        .nest(
            "/categories",
            category_controller::routes()
                .layer(axum::middleware::from_fn(require_auth)),
        )
        // Builder routes (protected)
        .nest(
            "/builder",
            builder_controller::routes()
                .layer(axum::middleware::from_fn(require_auth)),
        )
        // Settings routes (protected)
        .nest(
            "/settings",
            settings_controller::routes()
                .layer(axum::middleware::from_fn(require_auth)),
        )
        // Add state (database pool) to the application
        .with_state(db_pool)
        // Apply global middleware
        .layer(
            // CORS middleware
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Define the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Starting server at http://{}", addr);

    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
