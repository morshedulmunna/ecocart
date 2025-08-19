use axum::extract::Path;
use axum::{Json, Router, extract::Extension, routing::get};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::configs::app_context::AppContext;
use crate::pkg::auth::AuthUser;
use crate::pkg::error::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryDto {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/api/categories",
            get(list_categories).post(create_category),
        )
        .route(
            "/api/categories/:id",
            get(get_category)
                .put(update_category)
                .delete(delete_category),
        )
}

async fn list_categories(
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
) -> AppResult<Json<Vec<CategoryDto>>> {
    let rows = sqlx::query("SELECT id, name, description FROM categories ORDER BY id DESC")
        .fetch_all(&ctx.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let items = rows
        .into_iter()
        .map(|r| CategoryDto {
            id: Some(r.get("id")),
            name: r.get("name"),
            description: r.get("description"),
        })
        .collect();
    Ok(Json(items))
}

#[derive(Debug, Deserialize)]
struct CreateCategory {
    name: String,
    description: Option<String>,
}

async fn create_category(
    AuthUser { role, .. }: AuthUser,
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Json(req): Json<CreateCategory>,
) -> AppResult<Json<CategoryDto>> {
    if role != "admin" {
        return Err(AppError::Forbidden("Admin access required".into()));
    }
    let row = sqlx::query("INSERT INTO categories (name, description) VALUES ($1, $2) RETURNING id, name, description")
        .bind(&req.name)
        .bind(&req.description)
        .fetch_one(&ctx.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(CategoryDto {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
    }))
}

async fn get_category(
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Path(id): Path<i32>,
) -> AppResult<Json<CategoryDto>> {
    let row = sqlx::query("SELECT id, name, description FROM categories WHERE id = $1")
        .bind(id)
        .fetch_optional(&ctx.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let row = match row {
        Some(r) => r,
        None => return Err(AppError::NotFound("Category not found".into())),
    };
    Ok(Json(CategoryDto {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
    }))
}

#[derive(Debug, Deserialize)]
struct UpdateCategory {
    name: Option<String>,
    description: Option<String>,
}

async fn update_category(
    AuthUser { role, .. }: AuthUser,
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateCategory>,
) -> AppResult<Json<CategoryDto>> {
    if role != "admin" {
        return Err(AppError::Forbidden("Admin access required".into()));
    }
    let row = sqlx::query("UPDATE categories SET name = COALESCE($2, name), description = COALESCE($3, description) WHERE id = $1 RETURNING id, name, description")
        .bind(id).bind(&req.name).bind(&req.description)
        .fetch_one(&ctx.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(CategoryDto {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
    }))
}

async fn delete_category(
    AuthUser { role, .. }: AuthUser,
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Path(id): Path<i32>,
) -> AppResult<Json<serde_json::Value>> {
    if role != "admin" {
        return Err(AppError::Forbidden("Admin access required".into()));
    }
    let res = sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(&ctx.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound("Category not found".into()));
    }
    Ok(Json(serde_json::json!({"deleted": true})))
}
