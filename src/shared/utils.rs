use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, &'static str> {
    hash(password, DEFAULT_COST).map_err(|_| "Failed to hash password")
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, &'static str> {
    verify(password, hash).map_err(|_| "Failed to verify password")
}
