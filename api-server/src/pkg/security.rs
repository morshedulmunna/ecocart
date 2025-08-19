use anyhow::Result;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

/// Hash a plaintext password with Argon2id algorithm
pub fn hash_password(plain: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(plain.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("failed to hash password: {}", e))?
        .to_string();
    Ok(hash)
}

/// Verify a plaintext password against a stored Argon2 hash
pub fn verify_password(plain: &str, hashed: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(hashed)
        .map_err(|e| anyhow::anyhow!("invalid password hash format: {}", e))?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(plain.as_bytes(), &parsed_hash)
        .is_ok())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

/// Create a signed JWT with HS256
pub fn sign_jwt(claims: &Claims, secret: &str) -> Result<String> {
    let token = encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| anyhow::anyhow!("failed to sign jwt: {}", e))?;
    Ok(token)
}

/// Verify a JWT and deserialize claims
pub fn verify_jwt<T: for<'de> Deserialize<'de>>(token: &str, secret: &str) -> Result<T> {
    let data = decode::<T>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| anyhow::anyhow!("invalid jwt: {}", e))?;
    Ok(data.claims)
}

/// Helper to build standard access token claims
pub fn build_access_claims(user_id: &str, role: &str, ttl_seconds: i64) -> Claims {
    let exp = (Utc::now() + Duration::seconds(ttl_seconds)).timestamp() as usize;
    Claims {
        sub: user_id.to_string(),
        role: role.to_string(),
        exp,
    }
}
