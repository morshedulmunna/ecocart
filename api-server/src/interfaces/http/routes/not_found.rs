use crate::pkg::response::ApiErrorResponse;
use axum::{Json, http::StatusCode};

/// Default handler for unmatched routes (404 Not Found)
///
/// Returns a standardized JSON error response so clients consistently
/// receive structured error information for unknown paths.
pub async fn handler() -> (StatusCode, Json<ApiErrorResponse>) {
    let body = ApiErrorResponse::new("NOT_FOUND", "The requested resource was not found");
    (StatusCode::NOT_FOUND, Json(body))
}
