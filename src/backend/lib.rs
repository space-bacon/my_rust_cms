// Declare all the modules used in the backend.
pub mod config;
pub mod controllers;
pub mod models;
pub mod repositories;
pub mod services;
pub mod middlewares;

// If you have any globally shared components or types,
// you might want to include a prelude module to simplify imports.
pub mod prelude {
    pub use crate::config::*;
    pub use crate::controllers::*;
    pub use crate::models::*;
    pub use crate::repositories::*;
    pub use crate::services::*;
    pub use crate::middlewares::*;
}
