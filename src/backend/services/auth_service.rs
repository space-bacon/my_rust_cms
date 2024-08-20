use bcrypt::{hash, verify, DEFAULT_COST};
use crate::repositories::user_repository::UserRepository;
use crate::models::user::User;

pub struct AuthService;

impl AuthService {
    pub fn login(username: &str, password: &str) -> Result<String, &'static str> {
        if let Some(user) = UserRepository::find_by_username(username).ok() {
            if verify_password(password, &user.password_hash)? {
                // Generate JWT token or session token here
                Ok("JWT token".to_string())
            } else {
                Err("Invalid credentials")
            }
        } else {
            Err("User not found")
        }
    }

    pub fn register(username: &str, email: &str, password: &str) -> Result<(), &'static str> {
        let hashed_password = hash_password(password)?;
        let new_user = User {
            id: 0,
            username: username.to_string(),
            email: email.to_string(),
            password_hash: hashed_password,
            role: "user".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };
        UserRepository::create(new_user)
    }
}

fn hash_password(password: &str) -> Result<String, &'static str> {
    hash(password, DEFAULT_COST).map_err(|_| "Failed to hash password")
}

fn verify_password(password: &str, hash: &str) -> Result<bool, &'static str> {
    verify(password, hash).map_err(|_| "Failed to verify password")
}
