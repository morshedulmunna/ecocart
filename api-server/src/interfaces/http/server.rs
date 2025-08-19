use std::net::SocketAddr;
use std::sync::Arc;

use axum::{Extension, Router, middleware};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, services::ServeDir, trace::TraceLayer,
};

use super::middlewares::axum_error_handler::error_handler as error_handler_mw;
use super::middlewares::axum_rate_limit::{RateLimitState, rate_limit as rate_limit_mw};
use super::middlewares::axum_request_logger::request_logger as request_logger_mw;
use crate::configs::app_context::AppContext;
use crate::pkg::logger::info;

use super::routes;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Build the Axum application router with middlewares and routes
fn build_app(ctx: Arc<AppContext>) -> Router {
    let cors = CorsLayer::permissive();
    let trace = TraceLayer::new_for_http();
    let compression = CompressionLayer::new();

    let api = routes::router();
    let rate_limit_state = RateLimitState::new(100, 60);

    let app = Router::new()
        .merge(api)
        .nest_service("/uploads", ServeDir::new("./uploads"))
        .fallback(super::routes::not_found::handler)
        .layer(
            ServiceBuilder::new()
                .layer(trace)
                .layer(cors)
                .layer(compression),
        )
        .layer(middleware::from_fn(request_logger_mw))
        .layer(middleware::from_fn(error_handler_mw))
        .route_layer(middleware::from_fn_with_state(
            rate_limit_state.clone(),
            rate_limit_mw,
        ))
        .layer(Extension(ctx));

    // Build OpenAPI doc and mount Swagger UI at /docs
    let openapi = crate::interfaces::http::swagger::ApiDoc::openapi();
    app.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", openapi))
}

/// Start the HTTP server using Axum
pub async fn start_server(
    host: &str,
    config: &AppContext,
) -> Result<(), Box<dyn std::error::Error>> {
    let ctx = Arc::new(config.clone());
    let app = build_app(ctx);

    let addr: SocketAddr = format!("{}:{}", host, config.system.api_port).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

/// Start the server with default configuration
pub async fn start_default_server() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = AppContext::get()
        .await
        .expect("Failed to initialize AppContext");

    info(format!(
        "Server will be available at http://{}:{}",
        ctx.system.api_host, ctx.system.api_port
    ));

    start_server(&ctx.system.api_host, ctx.as_ref()).await
}

/// Entry called by `apis_command` to run the HTTP server.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    start_default_server().await?;
    Ok(())
}
