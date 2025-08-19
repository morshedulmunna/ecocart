//! Shared API response helpers and types
//!
//! Currently provides a small `ApiErrorResponse` used by the error layer to
//! produce consistent error JSON bodies.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Standard error response payload returned by the API
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    /// Stable machine-readable error code
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Timestamp at which the error was produced
    pub timestamp: DateTime<Utc>,
    /// Optional request/trace correlation id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// Optional structured details for clients (e.g., validation issues)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

impl ApiErrorResponse {
    /// Create a minimal error body
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            timestamp: Utc::now(),
            trace_id: Some(uuid::Uuid::new_v4().to_string()),
            details: None,
        }
    }

    /// Attach structured details (e.g., validation issues)
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }
}

/// Standard success response payload returned by the API
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    /// Human-readable message accompanying the response
    pub message: String,
    /// Timestamp at which the response was produced
    pub timestamp: DateTime<Utc>,
    pub status_code: u16,
    /// Optional success payload
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    /// Create a response with a message and no data
    pub fn with_message(message: impl Into<String>, status_code: u16) -> Self {
        Self {
            message: message.into(),
            timestamp: Utc::now(),
            status_code,
            data: None,
        }
    }

    /// Create a response with a message and data
    pub fn with_data(message: impl Into<String>, data: T, status_code: u16) -> Self {
        Self {
            message: message.into(),
            timestamp: Utc::now(),
            status_code,
            data: Some(data),
        }
    }
}
