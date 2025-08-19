use crate::pkg::response::ApiResponse;
use axum::{Json, http::StatusCode};

/// Root route handler returning a standardized success payload
pub async fn handler() -> (StatusCode, Json<ApiResponse<serde_json::Value>>) {
    let body = ApiResponse::<serde_json::Value>::with_message(
        "ecocart API Server",
        StatusCode::OK.as_u16(),
    );
    (StatusCode::OK, Json(body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_root() {
        let (status, _body) = handler().await;
        assert_eq!(status, StatusCode::OK);
    }
}
