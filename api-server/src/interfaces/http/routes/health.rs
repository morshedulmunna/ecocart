use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

pub async fn handler() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "healthy",
            "service": "ecocart"
        })),
    )
}
