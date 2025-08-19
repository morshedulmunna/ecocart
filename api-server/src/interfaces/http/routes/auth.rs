use axum::{Json, Router, extract::Extension, routing::post};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use utoipa::ToSchema;

use crate::configs::app_context::AppContext;
use crate::pkg::error::{AppError, AppResult};
use crate::pkg::security::{self, build_access_claims, hash_password, sign_jwt, verify_password};

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    id: uuid::Uuid,
    username: String,
    email: String,
    role: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    user: UserResponse,
    access_token: String,
    refresh_token: String,
    token_type: String,
    expires_in: i64,
}

pub fn router() -> Router {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/refresh", post(refresh))
        .route("/api/auth/logout", post(logout))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    id: uuid::Uuid,
}

/// Register a new user account
#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses((status = 200, description = "Registered", body = RegisterResponse)),
    tag = "Auth"
)]
async fn register(
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<RegisterResponse>> {
    let existing = sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(&req.email)
        .fetch_optional(&ctx.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if existing.is_some() {
        return Err(AppError::Conflict("Email already exists".into()));
    }

    let password_hash =
        hash_password(&req.password).map_err(|e| AppError::Internal(e.to_string()))?;
    let rec = sqlx::query(
        "INSERT INTO users (username, email, password_hash, role) VALUES ($1, $2, $3, $4) RETURNING id"
    )
    .bind(&req.username)
    .bind(&req.email)
    .bind(&password_hash)
    .bind("user")
    .fetch_one(&ctx.db_pool)
    .await
    .map_err(|e| AppError::Internal(e.to_string()))?;
    let id: uuid::Uuid = rec.get("id");

    Ok(Json(RegisterResponse { id }))
}

/// Log in and receive access/refresh tokens
#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses((status = 200, description = "Logged in", body = LoginResponse)),
    tag = "Auth"
)]
async fn login(
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    let row =
        sqlx::query("SELECT id, username, email, password_hash, role FROM users WHERE email = $1")
            .bind(&req.email)
            .fetch_optional(&ctx.db_pool)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

    let row = match row {
        Some(r) => r,
        None => return Err(AppError::Unauthorized("Invalid credentials".into())),
    };
    let hashed: String = row.get("password_hash");
    let user_id: uuid::Uuid = row.get("id");
    let username: String = row.get("username");
    let email: String = row.get("email");
    let role: String = row.get("role");

    let ok =
        verify_password(&req.password, &hashed).map_err(|e| AppError::Internal(e.to_string()))?;
    if !ok {
        return Err(AppError::Unauthorized("Invalid credentials".into()));
    }

    let access_claims =
        build_access_claims(&user_id.to_string(), &role, ctx.auth.access_ttl_seconds);
    let refresh_claims =
        build_access_claims(&user_id.to_string(), &role, ctx.auth.refresh_ttl_seconds);
    let access_token = sign_jwt(&access_claims, &ctx.auth.jwt_secret)
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let refresh_token = sign_jwt(&refresh_claims, &ctx.auth.jwt_secret)
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let user = UserResponse {
        id: user_id,
        username,
        email,
        role: role.clone(),
    };

    Ok(Json(LoginResponse {
        user,
        access_token,
        refresh_token,
        token_type: "Bearer".into(),
        expires_in: ctx.auth.access_ttl_seconds,
    }))
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    refresh_token: String,
}

/// Exchange refresh token for new access token
#[utoipa::path(
    post,
    path = "/api/auth/refresh",
    request_body = RefreshRequest,
    responses((status = 200, description = "New access token", body = TokenResponse)),
    tag = "Auth"
)]
async fn refresh(
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Json(req): Json<RefreshRequest>,
) -> AppResult<Json<TokenResponse>> {
    let claims: security::Claims =
        security::verify_jwt(&req.refresh_token, &ctx.auth.jwt_secret)
            .map_err(|_| AppError::Unauthorized("Invalid refresh token".into()))?;

    let access_claims = build_access_claims(&claims.sub, &claims.role, ctx.auth.access_ttl_seconds);
    let new_access = sign_jwt(&access_claims, &ctx.auth.jwt_secret)
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(TokenResponse {
        access_token: new_access,
        refresh_token: req.refresh_token,
        token_type: "Bearer".into(),
        expires_in: ctx.auth.access_ttl_seconds,
    }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OkResponse {
    ok: bool,
}

/// Logout (stateless)
#[utoipa::path(
    post,
    path = "/api/auth/logout",
    responses((status = 200, description = "Ok", body = OkResponse)),
    tag = "Auth"
)]
async fn logout() -> AppResult<Json<OkResponse>> {
    Ok(Json(OkResponse { ok: true }))
}
