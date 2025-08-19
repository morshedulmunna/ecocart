//! Axum rate limit middleware built on `pkg::rate_limit`

use std::sync::Arc;

use axum::{body::Body, extract::State, http::Request, middleware::Next, response::Response};
use http::HeaderValue;

use crate::pkg::logger::warn;
use crate::pkg::rate_limit::{InMemorySlidingWindowRateLimiter, RateLimitConfig};
use crate::pkg::response::ApiErrorResponse;

/// Shared limiter state for the middleware
#[derive(Clone)]
pub struct RateLimitState {
    limiter: Arc<InMemorySlidingWindowRateLimiter>,
}

impl RateLimitState {
    pub fn new(limit: u32, window_secs: u64) -> Self {
        let limiter = InMemorySlidingWindowRateLimiter::new(RateLimitConfig::new(
            limit,
            std::time::Duration::from_secs(window_secs),
        ));
        Self {
            limiter: Arc::new(limiter),
        }
    }
}

/// Middleware function that enforces sliding-window rate limit per ip+path.
pub async fn rate_limit(
    State(state): State<RateLimitState>,
    req: Request<Body>,
    next: Next,
) -> Response {
    // Skip rate limiting for CORS preflight and HEAD requests to avoid
    // intermittent CORS failures due to throttled preflights.
    // This ensures OPTIONS requests are always answered promptly by outer layers
    // such as the CORS middleware.
    let method = req.method().clone();
    if method == http::Method::OPTIONS || method == http::Method::HEAD {
        return next.run(req).await;
    }
    // Prefer X-Forwarded-For, then X-Real-IP, else unknown
    let headers = req.headers();
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .or_else(|| headers.get("x-real-ip").and_then(|h| h.to_str().ok()))
        .unwrap_or("unknown");
    let path = req.uri().path();
    let key = InMemorySlidingWindowRateLimiter::compose_key(&["ip", ip, "path", path]);
    let decision = state.limiter.check_and_consume(&key);

    // We attach rate-limit headers on the response path below

    if !decision.allowed {
        let retry_secs = decision.retry_after.map(|d| d.as_secs()).unwrap_or(1);
        warn(format!(
            "Rate limited ip={} path={} retry_after_secs={}",
            ip, path, retry_secs
        ));

        let response = axum::http::Response::builder()
            .status(http::StatusCode::TOO_MANY_REQUESTS)
            .header(http::header::RETRY_AFTER, retry_secs.to_string())
            .header(
                http::HeaderName::from_static("x-rate-limit-reset"),
                decision.reset_after.as_secs().to_string(),
            )
            .header(
                http::HeaderName::from_static("content-type"),
                "application/json",
            )
            .body(
                serde_json::to_vec(&ApiErrorResponse::new(
                    "TOO_MANY_REQUESTS",
                    "Too many requests. Please retry later.",
                ))
                .unwrap_or_default(),
            )
            .unwrap();

        return response.map(axum::body::Body::from);
    }

    let mut response = next.run(req).await;

    // Attach standard rate-limit headers to successful responses
    response.headers_mut().insert(
        http::HeaderName::from_static("x-rate-limit-limit"),
        HeaderValue::from_str(&decision.limit.to_string()).unwrap_or(HeaderValue::from_static("0")),
    );
    response.headers_mut().insert(
        http::HeaderName::from_static("x-rate-limit-remaining"),
        HeaderValue::from_str(&decision.remaining.to_string())
            .unwrap_or(HeaderValue::from_static("0")),
    );
    response.headers_mut().insert(
        http::HeaderName::from_static("x-rate-limit-reset"),
        HeaderValue::from_str(&decision.reset_after.as_secs().to_string())
            .unwrap_or(HeaderValue::from_static("0")),
    );

    response
}
