//! Centralized API error types and conversions for Axum
//!
//! This module defines a single `AppError` enum that represents all
//! application-level errors the HTTP layer can produce. It implements
//! `IntoResponse` so handlers can return `Result<T, AppError>` and get
//! consistent JSON error responses.

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::pkg::logger::error_with_details;
use crate::pkg::response::ApiErrorResponse;

/// Standard application error type.
#[derive(Debug)]
pub enum AppError {
    /// Request is invalid (missing/invalid params, parse errors)
    BadRequest(String),
    /// Authentication failed or missing
    Unauthorized(String),
    /// Authenticated but not allowed
    Forbidden(String),
    /// Resource not found
    NotFound(String),
    /// Conflict with current resource state
    Conflict(String),

    // Rate limit conflict
    RateLimitConflict(String),

    /// Validation failed with field-level details
    Validation {
        message: String,
        details: serde_json::Value,
    },
    /// Upstream dependency/service failure
    ServiceUnavailable(String),
    /// Request timed out
    Timeout(String),
    /// Internal, unexpected error
    Internal(String),
}

impl AppError {
    /// Short stable machine code for each error variant
    fn code(&self) -> &'static str {
        match self {
            AppError::BadRequest(_) => "BAD_REQUEST",
            AppError::Unauthorized(_) => "UNAUTHORIZED",
            AppError::Forbidden(_) => "FORBIDDEN",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::RateLimitConflict(_) => "RATE_LIMIT_CONFLICT",
            AppError::Validation { .. } => "VALIDATION_ERROR",
            AppError::ServiceUnavailable(_) => "SERVICE_UNAVAILABLE",
            AppError::Timeout(_) => "TIMEOUT",
            AppError::Internal(_) => "INTERNAL_ERROR",
        }
    }

    /// Human message associated with the error
    fn message(&self) -> &str {
        match self {
            AppError::BadRequest(m)
            | AppError::Unauthorized(m)
            | AppError::Forbidden(m)
            | AppError::NotFound(m)
            | AppError::Conflict(m)
            | AppError::ServiceUnavailable(m)
            | AppError::Timeout(m)
            | AppError::Internal(m)
            | AppError::RateLimitConflict(m) => m,
            AppError::Validation { message, .. } => message,
        }
    }

    /// HTTP status code mapping for each error variant
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::BadRequest(_)
            | AppError::Validation { .. }
            | AppError::RateLimitConflict(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Timeout(_) => StatusCode::GATEWAY_TIMEOUT,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Build the JSON body for this error
    fn to_body(&self) -> ApiErrorResponse {
        match self {
            AppError::RateLimitConflict(message) => {
                ApiErrorResponse::new(self.code(), message).with_details(serde_json::json!({}))
            }
            AppError::Validation { message, details } => {
                ApiErrorResponse::new(self.code(), message).with_details(details.clone())
            }
            _ => ApiErrorResponse::new(self.code(), self.message()),
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error_with_details(
            format!("{}: {}", self.code(), self.message()),
            format!("{:?}", self),
        );
        let status = self.status_code();
        let body = self.to_body();
        (status, Json(body)).into_response()
    }
}

// Conversions from common error types into AppError

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::BadRequest(err.to_string())
    }
}

/// Convenient result alias for handlers and services
pub type AppResult<T> = Result<T, AppError>;

/// Helper to create a validation error with structured details
#[derive(Debug, Serialize)]
pub struct ValidationIssue {
    pub field: String,
    pub message: String,
}

impl ValidationIssue {
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// Build a validation error from a list of issues
pub fn validation_error(message: impl Into<String>, issues: Vec<ValidationIssue>) -> AppError {
    let details = serde_json::json!({
        "issues": issues,
    });
    AppError::Validation {
        message: message.into(),
        details,
    }
}
