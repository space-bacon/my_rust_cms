use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum UserRole {
    Admin,
    Editor,
    Author,
    Contributor,
    Subscriber,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub data: T,
    pub message: String,
    pub success: bool,
}
