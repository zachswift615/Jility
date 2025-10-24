use anyhow::{anyhow, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const JWT_EXPIRATION_DAYS: i64 = 7;

#[derive(Clone)]
pub struct AuthService {
    jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: i64,    // expiration time
    pub iat: i64,    // issued at
}

impl AuthService {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    /// Hash a password using bcrypt
    pub fn hash_password(&self, password: &str) -> Result<String> {
        hash(password, DEFAULT_COST).map_err(|e| anyhow!("Failed to hash password: {}", e))
    }

    /// Verify a password against a hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        verify(password, hash).map_err(|e| anyhow!("Failed to verify password: {}", e))
    }

    /// Generate a JWT token for a user
    pub fn generate_jwt(&self, user_id: Uuid) -> Result<String> {
        let now = chrono::Utc::now();
        let expiration = now + chrono::Duration::days(JWT_EXPIRATION_DAYS);

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(|e| anyhow!("Failed to generate JWT: {}", e))
    }

    /// Validate a JWT token and return the claims
    pub fn validate_jwt(&self, token: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| anyhow!("Failed to validate JWT: {}", e))?;

        Ok(token_data.claims)
    }

    /// Generate an API key with the format "jil_live_" + 32 random characters
    /// Returns (key, hash) tuple
    pub fn generate_api_key(&self) -> Result<(String, String)> {
        use rand::Rng;
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        const KEY_LENGTH: usize = 32;

        let mut rng = rand::thread_rng();
        let random_part: String = (0..KEY_LENGTH)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        let key = format!("jil_live_{}", random_part);
        let key_hash = self.hash_password(&key)?;

        Ok((key, key_hash))
    }

    /// Get prefix from API key (first 8 chars after "jil_live_")
    pub fn get_key_prefix(key: &str) -> Result<String> {
        if !key.starts_with("jil_live_") {
            return Err(anyhow!("Invalid API key format"));
        }

        let suffix = &key[9..]; // Skip "jil_live_"
        if suffix.len() < 8 {
            return Err(anyhow!("API key too short"));
        }

        Ok(format!("jil_live_{}", &suffix[..8]))
    }

    /// Hash a token for storage (used for session revocation)
    pub fn hash_token(&self, token: &str) -> Result<String> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let service = AuthService::new("test_secret".to_string());
        let password = "test_password_123";

        let hash = service.hash_password(password).unwrap();
        assert!(service.verify_password(password, &hash).unwrap());
        assert!(!service.verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_jwt_generation() {
        let service = AuthService::new("test_secret".to_string());
        let user_id = Uuid::new_v4();

        let token = service.generate_jwt(user_id).unwrap();
        let claims = service.validate_jwt(&token).unwrap();

        assert_eq!(claims.sub, user_id.to_string());
    }

    #[test]
    fn test_api_key_generation() {
        let service = AuthService::new("test_secret".to_string());

        let (key, hash) = service.generate_api_key().unwrap();
        assert!(key.starts_with("jil_live_"));
        assert_eq!(key.len(), 9 + 32); // "jil_live_" + 32 chars

        // Verify the hash
        assert!(service.verify_password(&key, &hash).unwrap());

        // Test prefix extraction
        let prefix = AuthService::get_key_prefix(&key).unwrap();
        assert!(prefix.starts_with("jil_live_"));
        assert_eq!(prefix.len(), 9 + 8);
    }
}
