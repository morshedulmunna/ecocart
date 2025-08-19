use std::sync::Arc;

use axum::{
    Extension, Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
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

fn build_test_app(ctx: Arc<AppContext>) -> Router {
    routes::router().layer(Extension(ctx))
}

fn make_test_context() -> Arc<AppContext> {
    let db_pool = PgPoolOptions::new()
        .connect_lazy("postgres://ignored/ignored")
        .expect("lazy pool");

    let redis_cfg = RedisConfig {
        redis_url: "redis://127.0.0.1:6379/0".to_string(),
    };
    let redis_client = Arc::new(RedisClient::open(redis_cfg.redis_url.clone()).unwrap());

    let system = SystemConfig::load_from_env().unwrap();
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

fn bearer(token: &str) -> String {
    format!("Bearer {}", token)
}

#[tokio::test]
async fn create_category_requires_admin_forbidden_for_user() {
    let ctx = make_test_context();
    let app = build_test_app(ctx.clone());

    let claims = build_access_claims("u1", "user", ctx.auth.access_ttl_seconds);
    let token = sign_jwt(&claims, &ctx.auth.jwt_secret).unwrap();

    let body = json!({
        "name": "Electronics",
        "description": "Gadgets",
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/categories")
        .header("content-type", "application/json")
        .header("authorization", bearer(&token))
        .body(Body::from(body.to_string()))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    let _ = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
}

#[tokio::test]
async fn create_category_missing_auth_unauthorized() {
    let ctx = make_test_context();
    let app = build_test_app(ctx);

    let body = json!({
        "name": "Electronics",
        "description": "Gadgets",
    });
    let req = Request::builder()
        .method("POST")
        .uri("/api/categories")
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn update_category_requires_admin_forbidden_for_user() {
    let ctx = make_test_context();
    let app = build_test_app(ctx.clone());

    let claims = build_access_claims("u1", "user", ctx.auth.access_ttl_seconds);
    let token = sign_jwt(&claims, &ctx.auth.jwt_secret).unwrap();

    let body = json!({
        "name": "Updated",
    });
    let req = Request::builder()
        .method("PUT")
        .uri("/api/categories/1")
        .header("content-type", "application/json")
        .header("authorization", bearer(&token))
        .body(Body::from(body.to_string()))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn delete_category_requires_admin_forbidden_for_user() {
    let ctx = make_test_context();
    let app = build_test_app(ctx.clone());

    let claims = build_access_claims("u1", "user", ctx.auth.access_ttl_seconds);
    let token = sign_jwt(&claims, &ctx.auth.jwt_secret).unwrap();

    let req = Request::builder()
        .method("DELETE")
        .uri("/api/categories/1")
        .header("authorization", bearer(&token))
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}
