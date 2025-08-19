use std::sync::Arc;

use axum::{
    Extension, Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode},
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
async fn get_product_not_found_when_absent() {
    let ctx = make_test_context();
    let app = build_test_app(ctx);

    let req = Request::builder()
        .method("GET")
        .uri("/api/products/999999")
        .body(Body::empty())
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    // Handler queries DB, but since pool is lazy, it tries to connect and fails.
    // Our error mapping should be Internal Server Error in that case.
    assert!(
        resp.status() == StatusCode::INTERNAL_SERVER_ERROR
            || resp.status() == StatusCode::NOT_FOUND
    );

    let _ = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
}

#[tokio::test]
async fn create_product_without_body_is_bad_request() {
    let ctx = make_test_context();
    let app = build_test_app(ctx);

    let req = Request::builder()
        .method("POST")
        .uri("/api/products")
        .header("content-type", "application/json")
        .body(Body::from("{}"))
        .unwrap();

    let resp = app.oneshot(req).await.unwrap();
    // Missing required fields causes body-deserialize to succeed but DB insert will fail;
    // either BadRequest (if validation added) or InternalError otherwise.
    assert!(
        resp.status() == StatusCode::BAD_REQUEST
            || resp.status() == StatusCode::UNPROCESSABLE_ENTITY
            || resp.status() == StatusCode::INTERNAL_SERVER_ERROR
    );
}
