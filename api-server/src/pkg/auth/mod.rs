use std::sync::Arc;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::configs::app_context::AppContext;
use crate::pkg::error::AppError;
use crate::pkg::security::verify_jwt;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Try to get AppContext from request extensions (set by Extension layer)
        let ctx = parts
            .extensions
            .get::<Arc<AppContext>>()
            .cloned()
            .ok_or_else(|| AppError::Internal("Missing application context".into()))?;

        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid Authorization scheme".into()))?;

        let claims: crate::pkg::security::Claims = verify_jwt(token, &ctx.auth.jwt_secret)
            .map_err(|_| AppError::Unauthorized("Invalid token".into()))?;

        Ok(AuthUser {
            user_id: claims.sub,
            role: claims.role,
        })
    }
}

impl AuthUser {
    pub fn require_admin(&self) -> Result<(), AppError> {
        if self.role != "admin" {
            return Err(AppError::Forbidden("Admin access required".into()));
        }
        Ok(())
    }
}
