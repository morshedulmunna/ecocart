//! Axum request logging middleware
//!
//! Logs method, path, status, latency, client ip and user agent using the
//! application's structured logger.

use std::time::Instant;

use axum::{
    body::Body,
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};

use crate::pkg::logger::{LogLevel, get_logger};

/// Middleware that logs each HTTP request/response with basic metadata.
pub async fn request_logger(req: Request<Body>, next: Next) -> Response {
    let start = Instant::now();

    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let headers: &HeaderMap = req.headers();

    // Prefer X-Forwarded-For, then X-Real-IP, otherwise unknown.
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .or_else(|| headers.get("x-real-ip").and_then(|h| h.to_str().ok()))
        .unwrap_or("unknown")
        .to_string();

    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("")
        .to_string();

    let response = next.run(req).await;

    let status = response.status().as_u16() as i64;
    let latency_ms = start.elapsed().as_millis() as i64;

    let mut fields: std::collections::HashMap<String, serde_json::Value> =
        std::collections::HashMap::new();
    fields.insert("method".to_string(), method.into());
    fields.insert("path".to_string(), path.into());
    fields.insert("status".to_string(), status.into());
    fields.insert("latency_ms".to_string(), latency_ms.into());
    fields.insert("ip".to_string(), ip.into());
    if !user_agent.is_empty() {
        fields.insert("user_agent".to_string(), user_agent.into());
    }

    let logger = get_logger();
    logger.log_with_fields(LogLevel::Info, "http_request", None, fields);

    response
}
