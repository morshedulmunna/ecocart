use std::env;

/// Authentication/JWT configuration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_issuer: String,
    pub access_ttl_seconds: i64,
    pub refresh_ttl_seconds: i64,
}

impl AuthConfig {
    pub fn load_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let _ = dotenv::dotenv().ok();

        let jwt_secret =
            env::var("JWT_SECRET").unwrap_or_else(|_| "dev_secret_change_me".to_string());
        let jwt_issuer = env::var("JWT_ISSUER").unwrap_or_else(|_| "ecocart".to_string());
        let access_ttl_seconds = env::var("JWT_ACCESS_TTL_SECONDS")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(900);
        let refresh_ttl_seconds = env::var("JWT_REFRESH_TTL_SECONDS")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(60 * 60 * 24 * 30);

        Ok(Self {
            jwt_secret,
            jwt_issuer,
            access_ttl_seconds,
            refresh_ttl_seconds,
        })
    }
}
