use bcrypt::{hash, verify, DEFAULT_COST};
use crate::repositories::user_repository::UserRepository;
use crate::models::user::User;
use std::sync::Arc;
use warp::reject::Reject;

pub struct AuthService;

#[derive(Debug)]
struct AuthError;
impl Reject for AuthError {}

impl AuthService {
    pub async fn login(username: &str, password: &str) -> Result<String, warp::Rejection> {
        if let Some(user) = UserRepository::find_by_username(username).ok() {
            if verify_password(password, &user.password_hash)? {
                // Generate JWT token or session token here
                Ok("JWT token".to_string())
            } else {
                Err(warp::reject::custom(AuthError))
            }
        } else {
            Err(warp::reject::custom(AuthError))
        }
    }

    pub async fn register(username: &str, email: &str, password: &str) -> Result<(), warp::Rejection> {
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
        UserRepository::create(new_user).map_err(|_| warp::reject::custom(AuthError))
    }
}

fn hash_password(password: &str) -> Result<String, warp::Rejection> {
    hash(password, DEFAULT_COST).map_err(|_| warp::reject::custom(AuthError))
}

fn verify_password(password: &str, hash: &str) -> Result<bool, warp::Rejection> {
    verify(password, hash).map_err(|_| warp::reject::custom(AuthError))
}
