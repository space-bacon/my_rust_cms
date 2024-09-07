pub mod config;
pub mod services {
    pub mod auth_service;
    pub mod builder_service;
    pub mod comment_service;
    pub mod media_service;
    pub mod post_service;
    pub mod user_service;
    pub mod frontend;
}

#[cfg(test)]
mod tests {
    mod integration_tests;
    mod network_tests;
    mod unit_tests;
}

// Re-export commonly used functions and structs for easier access
pub use config::*;
pub use services::{
    auth_service::*, builder_service::*, comment_service::*,
    media_service::*, post_service::*, user_service::*,
};
