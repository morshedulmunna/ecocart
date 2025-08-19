use axum::{Extension, Json, Router, extract::Path, routing::get};
use serde::Serialize;
use sqlx::Row;
use utoipa::ToSchema;

use crate::configs::app_context::AppContext;
use crate::pkg::error::{AppError, AppResult};

#[derive(Debug, Serialize, ToSchema)]
pub struct UserProfile {
    id: uuid::Uuid,
    username: String,
    email: String,
    role: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

pub fn router() -> Router {
    Router::new().route("/api/users/:id", get(get_user))
}

/// Get a user profile by id
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(("id" = String, Path, description = "User id (UUID)")),
    responses(
        (status = 200, description = "User profile", body = UserProfile),
        (status = 404, description = "Not found", body = crate::pkg::response::ApiErrorResponse)
    ),
    tag = "Users"
)]
async fn get_user(
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Path(id): Path<uuid::Uuid>,
) -> AppResult<Json<UserProfile>> {
    let row = sqlx::query("SELECT id, username, email, role, created_at FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(&ctx.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = match row {
        Some(r) => r,
        None => return Err(AppError::NotFound("User not found".into())),
    };

    Ok(Json(UserProfile {
        id: row.get("id"),
        username: row.get("username"),
        email: row.get("email"),
        role: row.get("role"),
        created_at: row.get("created_at"),
    }))
}
