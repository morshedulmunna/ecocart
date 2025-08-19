//! Axum global error handling middleware
//!
//! Catches unexpected errors from downstream services and converts them to a
//! standardized JSON error response using `AppError`.

use axum::response::IntoResponse;
use axum::{body::Body, middleware::Next, response::Response};
use http::Request;

use crate::pkg::error::AppError;
use crate::pkg::logger::error_with_details;

/// Middleware that converts unhandled errors into `AppError` JSON responses.
pub async fn error_handler(req: Request<Body>, next: Next) -> Response {
    match std::panic::AssertUnwindSafe(next.run(req))
        .catch_unwind()
        .await
    {
        Ok(res) => res,
        Err(panic) => {
            // Convert panic payload into a string for logging.
            let detail = if let Some(s) = panic.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic.downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown panic".to_string()
            };

            error_with_details("UNHANDLED_PANIC", detail.clone());
            let app_err = AppError::Internal(detail);
            app_err.into_response()
        }
    }
}

use futures::FutureExt;
