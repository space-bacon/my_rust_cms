// Declare all the modules used in the backend.
pub mod config;
pub mod controllers;
pub mod models;
pub mod repositories;
pub mod services;
pub mod middlewares;

// Re-export services for external use
pub use services::{
    auth_service::*, builder_service::*, comment_service::*,
    media_service::*, post_service::*, user_service::*,
};

// Re-export config, models, repositories, etc., for easy access
pub use config::*;
pub use models::*;
pub use repositories::*;
pub use middlewares::*;

// Pre-export commonly used items to simplify imports elsewhere
pub mod prelude {
    pub use crate::config::*;
    pub use crate::controllers::*;
    pub use crate::models::*;
    pub use crate::repositories::*;
    pub use crate::services::*;
    pub use crate::middlewares::*;
}
