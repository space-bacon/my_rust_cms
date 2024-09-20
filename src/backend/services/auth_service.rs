// src/backend/services/auth_service.rs

use argon2::{self, Config};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

use crate::backend::models::user::{NewUser, User};
use crate::backend::schema::users::dsl::*;
use crate::backend::utils::db::DbPool;

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("Hashing error: {0}")]
    HashingError(String),
    #[error("Verification error: {0}")]
    VerificationError(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub struct AuthService {
    db_pool: DbPool,
}

impl AuthService {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    /// Hash password using Argon2
    fn hash_password(&self, password: &str) -> Result<String, AuthServiceError> {
        let config = Config::default();
        let salt = argon2::password_hash::SaltString::generate(&mut rand::thread_rng());
        argon2::hash_encoded(password.as_bytes(), salt.as_ref(), &config)
            .map_err(|e| {
                error!("Hashing error: {:?}", e);
                AuthServiceError::HashingError(e.to_string())
            })
    }

    /// Verify password
    fn verify_password(&self, hash: &str, password: &str) -> Result<bool, AuthServiceError> {
        argon2::verify_encoded(hash, password.as_bytes())
            .map_err(|e| {
                error!("Verification error: {:?}", e);
                AuthServiceError::VerificationError(e.to_string())
            })
    }

    /// Register a new user
    pub async fn register_user(
        &self,
        new_user: NewUser,
    ) -> Result<User, AuthServiceError> {
        let hashed_password = self.hash_password(&new_user.password_hash)?;

        let conn = self.get_connection()?;
        let new_user = NewUser {
            password_hash: hashed_password,
            ..new_user
        };

        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(&conn)
            .map_err(|e| {
                error!("Database error: {:?}", e);
                AuthServiceError::DatabaseError(e.to_string())
            })
    }

    /// Authenticate a user
    pub async fn authenticate_user(
        &self,
        username_input: &str,
        password_input: &str,
    ) -> Result<User, AuthServiceError> {
        let conn = self.get_connection()?;
        let user = users
            .filter(username.eq(username_input))
            .first::<User>(&conn)
            .map_err(|_| AuthServiceError::InvalidCredentials)?;

        let is_valid = self.verify_password(&user.password_hash, password_input)?;
        if is_valid {
            Ok(user)
        } else {
            Err(AuthServiceError::InvalidCredentials)
        }
    }

    /// Helper function to get a database connection
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, AuthServiceError> {
        self.db_pool.get().map_err(|e| {
            error!("Database connection error: {:?}", e);
            AuthServiceError::DatabaseError(e.to_string())
        })
    }
}
