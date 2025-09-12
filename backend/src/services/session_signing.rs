//! Session Token Signing Service
//! 
//! Provides HMAC-SHA256 signing for session tokens to add an additional
//! layer of security beyond database validation.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose};
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

/// Service for signing and verifying session tokens with HMAC-SHA256
#[derive(Clone)]
pub struct SessionSigner {
    secret: Vec<u8>,
}

impl SessionSigner {
    /// Create a new session signer with the provided secret
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.as_bytes().to_vec(),
        }
    }

    /// Generate a new signed session token
    /// 
    /// Format: {uuid}.{base64_signature}
    /// Example: 123e4567-e89b-12d3-a456-426614174000.dGhpc19pc19hX3NpZ25hdHVyZQ
    #[allow(dead_code)]
    pub fn create_signed_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        let uuid_token = Uuid::new_v4().to_string();
        let signature = self.sign_token(&uuid_token)?;
        Ok(format!("{}.{}", uuid_token, signature))
    }

    /// Create a signed token from an existing UUID
    /// 
    /// Format: {uuid}.{base64_signature}
    pub fn create_signed_token_from_uuid(&self, uuid: &str) -> Result<String, Box<dyn std::error::Error>> {
        let signature = self.sign_token(uuid)?;
        Ok(format!("{}.{}", uuid, signature))
    }

    /// Sign a token with HMAC-SHA256
    fn sign_token(&self, token: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut mac = HmacSha256::new_from_slice(&self.secret)?;
        mac.update(token.as_bytes());
        let result = mac.finalize();
        let signature = general_purpose::URL_SAFE_NO_PAD.encode(result.into_bytes());
        Ok(signature)
    }

    /// Verify a signed token and extract the UUID if valid
    /// 
    /// Returns the UUID part if the signature is valid, None if invalid
    pub fn verify_signed_token(&self, signed_token: &str) -> Option<String> {
        let parts: Vec<&str> = signed_token.split('.').collect();
        if parts.len() != 2 {
            return None;
        }

        let uuid_token = parts[0];
        let provided_signature = parts[1];

        // Verify the signature
        match self.sign_token(uuid_token) {
            Ok(expected_signature) => {
                if constant_time_eq(&expected_signature, provided_signature) {
                    Some(uuid_token.to_string())
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    /// Check if a token looks like a signed token (has a dot separator)
    pub fn is_signed_token(token: &str) -> bool {
        token.contains('.') && token.split('.').count() == 2
    }
}

/// Constant-time string comparison to prevent timing attacks
fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (byte_a, byte_b) in a.bytes().zip(b.bytes()) {
        result |= byte_a ^ byte_b;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_verify_signed_token() {
        let signer = SessionSigner::new("test-secret-key");
        
        let signed_token = signer.create_signed_token().unwrap();
        assert!(signed_token.contains('.'));
        
        let verified_uuid = signer.verify_signed_token(&signed_token);
        assert!(verified_uuid.is_some());
        
        // The UUID should be valid
        let uuid_str = verified_uuid.unwrap();
        assert!(Uuid::parse_str(&uuid_str).is_ok());
    }

    #[test]
    fn test_verify_invalid_signature() {
        let signer = SessionSigner::new("test-secret-key");
        
        // Create a token with wrong signature
        let uuid = Uuid::new_v4().to_string();
        let fake_signed_token = format!("{}.fake_signature", uuid);
        
        let result = signer.verify_signed_token(&fake_signed_token);
        assert!(result.is_none());
    }

    #[test]
    fn test_verify_malformed_token() {
        let signer = SessionSigner::new("test-secret-key");
        
        // Test various malformed tokens
        assert!(signer.verify_signed_token("no-dot-here").is_none());
        assert!(signer.verify_signed_token("too.many.dots.here").is_none());
        assert!(signer.verify_signed_token("").is_none());
    }

    #[test]
    fn test_different_secrets_produce_different_signatures() {
        let signer1 = SessionSigner::new("secret1");
        let signer2 = SessionSigner::new("secret2");
        
        let token1 = signer1.create_signed_token().unwrap();
        let token2 = signer2.create_signed_token().unwrap();
        
        // signer2 should not be able to verify signer1's token
        assert!(signer2.verify_signed_token(&token1).is_none());
        assert!(signer1.verify_signed_token(&token2).is_none());
    }

    #[test]
    fn test_is_signed_token() {
        assert!(SessionSigner::is_signed_token("uuid.signature"));
        assert!(!SessionSigner::is_signed_token("just-uuid"));
        assert!(!SessionSigner::is_signed_token("too.many.dots"));
        assert!(!SessionSigner::is_signed_token(""));
    }
}
