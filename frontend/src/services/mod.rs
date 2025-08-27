// src/frontend/services/mod.rs (or src/frontend/services.rs)

pub mod api_service;
pub mod auth_service;
pub mod auth_context;
pub mod navigation_service;
pub mod page_service;
pub mod performance_service;
pub mod sample_page_data;
pub mod default_pages;
pub mod migrate_pages;
pub mod modern_menu_service;
pub mod user_service;

// Export modules for direct access
// Services are accessed via module::service syntax
