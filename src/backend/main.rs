// backend/src/main.rs

use axum::{
    routing::{get, post, put, delete},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use std::net::SocketAddr;

use crate::backend::controllers::{
    auth_controller,
    post_controller,
    media_controller,
    category_controller,
    builder_controller,
    settings_controller,
};
use crate::backend::utils::{db::establish_connection_pool, auth::require_auth};
use crate::backend::services::{
    auth_service::AuthService,
    post_service::PostService,
    media_service::MediaService,
    category_service::CategoryService,
    builder_service::BuilderService,
    settings_service::SettingsService,
};
use crate::backend::utils::db::DbPool;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db_pool: DbPool,
    auth_service: Arc<AuthService>,
    post_service: Arc<PostService>,
    media_service: Arc<MediaService>,
    category_service: Arc<CategoryService>,
    builder_service: Arc<BuilderService>,
    settings_service: Arc<SettingsService>,
    // All shared services have been added
}

#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber for logging
    tracing_subscriber::fmt::init();

    // Establish database connection pool
    let db_pool = establish_connection_pool();

    // Initialize shared services
    let auth_service = Arc::new(AuthService::new(db_pool.clone()));
    let post_service = Arc::new(PostService::new(db_pool.clone()));
    let media_service = Arc::new(MediaService::new(db_pool.clone()));
    let category_service = Arc::new(CategoryService::new(db_pool.clone()));
    let builder_service = Arc::new(BuilderService::new(db_pool.clone()));
    let settings_service = Arc::new(SettingsService::new(db_pool.clone()));

    // Create shared application state
    let app_state = AppState {
        db_pool: db_pool.clone(),
        auth_service: auth_service.clone(),
        post_service: post_service.clone(),
        media_service: media_service.clone(),
        category_service: category_service.clone(),
        builder_service: builder_service.clone(),
        settings_service: settings_service.clone(),
    };

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
        // Add shared application state
        .with_state(app_state)
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
        .expect("Server failed to start");
}
