use std::sync::Arc;

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Extension, Router,
};
use tower::util::ServiceExt; // oneshot

use ecocart::configs::app_context::AppContext;
use ecocart::configs::auth_config::AuthConfig;
use ecocart::configs::redis_config::RedisConfig;
use ecocart::configs::system_config::SystemConfig;
use ecocart::interfaces::http::routes;

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

#[tokio::test]
async fn get_user_not_found_or_internal_without_db() {
    let ctx = make_test_context();
    let app = build_test_app(ctx);

    let id = uuid::Uuid::new_v4();
    let req = Request::builder()
        .method("GET")
        .uri(format!("/api/users/{}", id))
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    // Depending on DB error vs not found, accept either Internal or NotFound
    assert!(
        resp.status() == StatusCode::INTERNAL_SERVER_ERROR
            || resp.status() == StatusCode::NOT_FOUND
    );
    let _ = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
}


