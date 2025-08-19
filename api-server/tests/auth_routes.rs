use std::sync::Arc;

use axum::{
    Extension, Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde::Deserialize;
use serde_json::json;
use tower::util::ServiceExt; // for `oneshot`

// Crate modules
use ecocart::configs::app_context::AppContext;
use ecocart::configs::auth_config::AuthConfig;
use ecocart::configs::redis_config::RedisConfig;
use ecocart::configs::system_config::SystemConfig;
use ecocart::interfaces::http::routes;
use ecocart::pkg::security::{build_access_claims, sign_jwt};

use redis::Client as RedisClient;
use sqlx::postgres::PgPoolOptions;

/// Build a minimal router with the auth routes and an injected `AppContext`.
fn build_test_app(ctx: Arc<AppContext>) -> Router {
    routes::router().layer(Extension(ctx))
}

/// Create a lightweight `AppContext` suitable for exercising auth routes
/// without requiring live Postgres or Redis connections.
fn make_test_context() -> Arc<AppContext> {
    // Lazy Postgres pool that won't attempt a network connection unless used.
    let db_pool = PgPoolOptions::new()
        .connect_lazy("postgres://ignored/ignored")
        .expect("lazy pool should construct");

    let redis_cfg = RedisConfig {
        redis_url: "redis://127.0.0.1:6379/0".to_string(),
    };
    // `Client::open` does not connect until a connection is requested.
    let redis_client =
        Arc::new(RedisClient::open(redis_cfg.redis_url.clone()).expect("redis client open"));

    let system = SystemConfig::load_from_env().expect("system config");
    let auth = AuthConfig {
        jwt_secret: "test_secret_change_me".to_string(),
        jwt_issuer: "ecocart-tests".to_string(),
        access_ttl_seconds: 900,
        refresh_ttl_seconds: 3600,
    };

    Arc::new(AppContext {
        system,
        db_pool,
        redis_config: redis_cfg,
        redis_client,
        auth,
    })
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
}

#[tokio::test]
async fn refresh_returns_new_access_token() {
    let ctx = make_test_context();
    let app = build_test_app(ctx.clone());

    // Create a valid refresh token using the same secret and TTL as the app.
    let claims = build_access_claims("user-123", "user", ctx.auth.refresh_ttl_seconds);
    let refresh_token = sign_jwt(&claims, &ctx.auth.jwt_secret).expect("sign refresh");

    let req_body = json!({ "refresh_token": refresh_token });
    let request = Request::builder()
        .method("POST")
        .uri("/api/auth/refresh")
        .header("content-type", "application/json")
        .body(Body::from(req_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let payload: TokenResponse = serde_json::from_slice(&body).unwrap();
    assert_eq!(payload.token_type, "Bearer");
    assert_eq!(payload.expires_in, ctx.auth.access_ttl_seconds);
    assert!(!payload.access_token.is_empty());
}

#[tokio::test]
async fn refresh_with_invalid_token_is_unauthorized() {
    let ctx = make_test_context();
    let app = build_test_app(ctx);

    let req_body = json!({ "refresh_token": "invalid.token.value" });
    let request = Request::builder()
        .method("POST")
        .uri("/api/auth/refresh")
        .header("content-type", "application/json")
        .body(Body::from(req_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn logout_returns_ok_true() {
    let ctx = make_test_context();
    let app = build_test_app(ctx);

    let request = Request::builder()
        .method("POST")
        .uri("/api/auth/logout")
        .header("content-type", "application/json")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["ok"], true);
}
