// backend/src/main.rs

mod config;
mod database;
mod schema;
mod models;
mod middleware;
mod services;
mod controllers;

use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    response::IntoResponse,
    Router,
    middleware as axum_middleware,
};
use std::net::SocketAddr;
use tracing::{info, error};
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;
use config::Config;
use database::{DbPool, establish_connection_pool};
// Removed unused global import
use middleware::auth::{auth_middleware_with_services, admin_auth_middleware_with_services};
// Rate limiting temporarily disabled due to API changes
// use middleware::rate_limiting::{create_auth_rate_limiter, create_upload_rate_limiter};
use middleware::security_headers::security_headers_middleware;

use services::{SessionManager, SessionConfig};


// Database connection pool state
use std::sync::Arc;



#[derive(Clone)]
pub struct AppServices {
    pub db_pool: Arc<DbPool>,
    pub session_manager: SessionManager,
    pub db_service: services::DbService,
}

// Re-export controller types for convenience
pub use controllers::auth::{LoginRequest, LoginResponse, UserProfile};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::new()?;
    
    // Load environment variables from .env file if it exists
    let _ = dotenv();
    
    // Set up logging
    std::env::set_var("RUST_LOG", &config.rust_log);
    tracing_subscriber::fmt::init();

    // Initialize database connection pool
    let pool = establish_connection_pool(&config.database_url)?;
    info!("Database connection pool established");
    
    // Store pool in state and initialize services
    let db_pool = Arc::new(pool);
    
    // Initialize session manager with custom config
    let session_config = SessionConfig {
        session_duration_hours: 24,
        cleanup_interval_minutes: 10, // More frequent cleanup for demo
        max_sessions_per_user: 3,
        enable_session_refresh: true,
        refresh_threshold_minutes: 30,
        enable_token_signing: true, // Enable HMAC-SHA256 token signing
    };
    
    let session_manager = SessionManager::new_with_signing(
        db_pool.clone(), 
        session_config,
        &config.session_secret
    );
    
    // Start background session cleanup
    let _cleanup_task = session_manager.clone().start_background_cleanup().await;
    info!("Session cleanup background task started");
    
    let db_service = services::DbService::new(db_pool.clone());
    
    let app_services = AppServices {
        db_pool: db_pool.clone(),
        session_manager,
        db_service,
    };
    
    // Initialize with default CMS data
    {
        use services::default_data_seeder::seed_default_cms_data;
        
        // Seed default data (users, categories, posts, templates, settings, etc.)
        if let Err(e) = seed_default_cms_data(db_pool.clone()).await {
            error!("Failed to seed default CMS data: {}", e);
        }
    }
    
    // Initialize sample plugins
    {
        use services::plugin_seeder::seed_sample_plugins;
        
        if let Err(e) = seed_sample_plugins(db_pool.clone()).await {
            error!("Failed to seed sample plugins: {}", e);
        }
    }
    
    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);
    
    // Create the application routes
    let public_routes = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/public/system/settings", get(controllers::system::get_public_settings))
        .route("/api/posts", get(controllers::posts::get_posts))
        .route("/api/auth/login", post(controllers::auth::login))
        .route("/api/auth/signup", post(controllers::auth::signup))
        .route("/api/auth/verify-email", post(controllers::auth::verify_email))
        // TODO: Re-enable rate limiting when API is stabilized
        // .layer(create_auth_rate_limiter())
        .route("/api/posts/:id", get(controllers::posts::get_post))
        .route("/api/categories", get(controllers::admin::get_categories))
        .route("/api/navigation", get(controllers::navigation::get_navigation))
        .route("/api/navigation/area/:area", get(controllers::navigation::get_navigation_by_area))
        .route("/api/menu-areas/:name", get(controllers::navigation::get_menu_area_by_name))
        .route("/api/component-templates", get(controllers::navigation::get_component_templates))
        .route("/api/pages", get(controllers::pages::get_pages))
        .route("/api/pages/:id", get(controllers::pages::get_page))
        .route("/api/pages/slug/:slug", get(controllers::pages::get_page_by_slug))
        .route("/api/comments/public", get(controllers::comments::get_post_comments))
        .route("/api/plugins/active", get(controllers::plugins::get_active_plugins))
        .route("/api/plugins/base-template", get(controllers::plugins::download_base_plugin))
        .route("/api/test", get(test_endpoint));

    // Authenticated routes (requires valid session)
    let auth_routes = Router::new()
        .route("/api/auth/logout", post(controllers::auth::logout))
        .route("/api/auth/me", get(controllers::auth::get_current_user))
        .route("/api/auth/sessions", get(controllers::sessions::get_user_sessions))
        .route("/api/auth/sessions/logout-all", post(controllers::sessions::logout_all_sessions))
        .route("/api/comments/create", post(controllers::comments::create_public_comment))
        .route("/api/pages/:id/content", put(controllers::pages::update_page_content))
        .layer(axum_middleware::from_fn_with_state(app_services.clone(), auth_middleware_with_services));

    // Admin-only routes (requires admin role)
    let admin_routes = Router::new()
        .route("/api/users", get(controllers::users::get_users).post(controllers::users::create_user))
        .route("/api/users/:id", put(controllers::users::update_user).delete(controllers::users::delete_user))
        .route("/api/users/:id/promote", put(controllers::users::promote_user))
        .route("/api/posts", post(controllers::posts::create_post))
        .route("/api/posts/:id", put(controllers::posts::update_post).delete(controllers::posts::delete_post))
        .route("/api/comments", get(controllers::comments::get_comments).post(controllers::comments::create_comment))
        .route("/api/comments/:id", put(controllers::comments::update_comment).delete(controllers::comments::delete_comment))
        .route("/api/media", get(controllers::media::get_media))
        .route("/api/media/upload", post(controllers::media::upload_media))
        // TODO: Re-enable upload rate limiting when API is stabilized
        // .layer(create_upload_rate_limiter())
        .route("/api/media/:id", delete(controllers::media::delete_media))
        .route("/api/sessions", get(controllers::admin::get_sessions))
        .route("/api/settings", get(controllers::admin::get_settings))
        .route("/api/templates", get(controllers::admin::get_templates))
        .route("/api/components", get(controllers::admin::get_components))
        .route("/api/navigation", post(controllers::navigation::create_navigation_item))
        .route("/api/navigation/:id", put(controllers::navigation::update_navigation_item).delete(controllers::navigation::delete_navigation_item))
        // Enhanced navigation management routes
        .route("/api/menu-areas", get(controllers::navigation::get_menu_areas))
        .route("/api/menu-areas/:name", put(controllers::navigation::update_menu_area))
        .route("/api/menu-templates", get(controllers::navigation::get_menu_templates).post(controllers::navigation::create_menu_template))
        .route("/api/menu-templates/type/:template_type", get(controllers::navigation::get_menu_templates_by_type))
        .route("/api/component-templates", post(controllers::navigation::create_component_template))
        .route("/api/component-templates/admin", get(controllers::navigation::get_all_component_templates_admin))
        .route("/api/component-templates/:id", put(controllers::navigation::update_component_template))
        .route("/api/component-templates/:id/toggle", post(controllers::navigation::toggle_component_template))
        .route("/api/component-templates/type/:component_type", get(controllers::navigation::get_component_templates_by_type))
        .route("/api/pages", post(controllers::pages::create_page))
        .route("/api/pages/:id", put(controllers::pages::update_page).delete(controllers::pages::delete_page))
        .route("/api/stats", get(controllers::admin::get_stats))
        .route("/api/performance", get(controllers::admin::get_performance_metrics))
        .route("/api/admin/sessions", get(controllers::sessions::get_all_session_stats))
        .route("/api/admin/sessions/cleanup", post(controllers::sessions::manual_session_cleanup))
        .route("/api/admin/users/:id/sessions", get(controllers::sessions::get_admin_user_sessions))
        .route("/api/admin/users/:id/force-logout", post(controllers::sessions::force_logout_user))
        // System management routes
        .route("/api/system/settings", get(controllers::system::get_settings).put(controllers::system::update_settings))
        .route("/api/system/settings/:key", get(controllers::system::get_setting))
        .route("/api/system/info", get(controllers::system::get_system_info))
        .route("/api/system/backup", post(controllers::system::create_backup))
        .route("/api/system/backups", get(controllers::system::list_backups))
        .route("/api/system/backup/:id/restore", post(controllers::system::restore_backup))
        .route("/api/system/snapshot", get(controllers::system::get_data_snapshot))
        // Plugin management routes
        .route("/api/plugins", get(controllers::plugins::get_plugins).post(controllers::plugins::create_plugin))
        .route("/api/plugins/:id", get(controllers::plugins::get_plugin).put(controllers::plugins::update_plugin).delete(controllers::plugins::delete_plugin))
        .route("/api/plugins/:id/action", post(controllers::plugins::plugin_action))
        .route("/api/plugins/seed-samples", post(controllers::plugins::seed_sample_plugins))
        .route("/api/plugins/upload-zip", post(controllers::plugins::upload_plugin_zip))
        .layer(axum_middleware::from_fn_with_state(app_services.clone(), admin_auth_middleware_with_services));

    // Combine all routes
    let app = Router::new()
        .merge(public_routes)
        .merge(auth_routes)
        .merge(admin_routes)
        .nest_service("/uploads", tower_http::services::ServeDir::new("uploads"))
        .with_state(app_services.clone())
        .layer(cors)
        .layer(axum_middleware::from_fn_with_state(
            config.clone(),
            security_headers_middleware
        ));

    // Run the server
    let addr = SocketAddr::new(config.backend_host.parse()?, config.backend_port);
    info!("Starting server at http://{}", addr);
    info!("Database URL: {}", config.database_url);
    info!("Environment: {}", config.rust_env);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn root() -> impl IntoResponse {
    "My Rust CMS Backend is running!"
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}


// Test endpoint
async fn test_endpoint() -> Result<axum::Json<serde_json::Value>, StatusCode> {
    Ok(axum::Json(serde_json::json!({
        "message": "Backend is working!",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
