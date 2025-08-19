use axum::{
    Extension, Json, Router,
    extract::{Path, Query},
    routing::{get, post},
};
use axum_extra::extract::Multipart;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, QueryBuilder, Row};
use std::fs;

use crate::configs::app_context::AppContext;
use crate::pkg::error::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDto {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub category_id: Option<i32>,
    pub image_url: Option<String>,
    pub stock: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct ProductQuery {
    pub q: Option<String>,
    pub category_id: Option<i32>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    /// 1-based page index (defaults to 1)
    pub page: Option<i64>,
    /// Page size (defaults to 12). Accepts camelCase alias from web client
    #[serde(alias = "pageSize")]
    pub page_size: Option<i64>,
}

pub fn router() -> Router {
    Router::new()
        .route("/api/products", get(list_products).post(create_product))
        .route("/api/products/upload", post(upload_image))
        .route(
            "/api/products/:id",
            get(get_product).put(update_product).delete(delete_product),
        )
}

#[derive(Debug, Serialize)]
struct Paginated<T>
where
    T: Serialize,
{
    items: Vec<T>,
    page: i64,
    page_size: i64,
    total: i64,
    total_pages: i64,
}

async fn list_products(
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Query(q): Query<ProductQuery>,
) -> AppResult<Json<Paginated<ProductDto>>> {
    // Resolve pagination params with sane defaults and caps
    let mut page = q.page.unwrap_or(1);
    if page < 1 {
        page = 1;
    }
    let mut page_size = q.page_size.unwrap_or(12);
    if page_size < 1 {
        page_size = 12;
    }
    if page_size > 100 {
        page_size = 100;
    }
    let offset = (page - 1) * page_size;

    // Construct base filter using QueryBuilder to safely bind values
    let mut count_builder: QueryBuilder<Postgres> =
        QueryBuilder::new("SELECT COUNT(*) AS total FROM products WHERE 1=1");
    let mut list_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT id, name, description, price, category_id, image_url, stock, created_at FROM products WHERE 1=1",
    );

    // Helper closure to append the same filters to both builders
    let append_filters = |builder: &mut QueryBuilder<Postgres>| {
        if let Some(ref s) = q.q {
            builder
                .push(" AND (name ILIKE ")
                .push_bind(format!("%{}%", s))
                .push(" OR description ILIKE ")
                .push_bind(format!("%{}%", s))
                .push(")");
        }
        if let Some(cid) = q.category_id {
            builder.push(" AND category_id = ").push_bind(cid);
        }
        if let Some(min) = q.min_price {
            builder.push(" AND price >= ").push_bind(min);
        }
        if let Some(max) = q.max_price {
            builder.push(" AND price <= ").push_bind(max);
        }
    };

    append_filters(&mut count_builder);
    append_filters(&mut list_builder);

    // Execute count query
    let total_row = count_builder
        .build()
        .fetch_one(&ctx.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let total: i64 = total_row.get("total");

    // Complete list query with ordering and pagination
    list_builder
        .push(" ORDER BY id DESC LIMIT ")
        .push_bind(page_size)
        .push(" OFFSET ")
        .push_bind(offset);

    let rows = list_builder
        .build()
        .fetch_all(&ctx.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let items: Vec<ProductDto> = rows
        .into_iter()
        .map(|r| ProductDto {
            id: Some(r.get("id")),
            name: r.get("name"),
            description: r.get("description"),
            price: r.get("price"),
            category_id: r.get("category_id"),
            image_url: r.get("image_url"),
            stock: r.get("stock"),
            created_at: r.get("created_at"),
        })
        .collect();

    let total_pages = if total == 0 {
        0
    } else {
        (total + page_size - 1) / page_size
    };

    Ok(Json(Paginated {
        items,
        page,
        page_size,
        total,
        total_pages,
    }))
}

#[derive(Debug, Deserialize)]
struct CreateProduct {
    name: String,
    description: Option<String>,
    price: f64,
    category_id: Option<i32>,
    stock: i32,
    image_url: Option<String>,
}

async fn create_product(
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Json(req): Json<CreateProduct>,
) -> AppResult<Json<ProductDto>> {
    let row = sqlx::query("INSERT INTO products (name, description, price, category_id, image_url, stock) VALUES ($1,$2,$3,$4,$5,$6) RETURNING id, name, description, price, category_id, image_url, stock, created_at")
        .bind(&req.name).bind(&req.description).bind(req.price).bind(&req.category_id).bind(&req.image_url).bind(req.stock)
        .fetch_one(&ctx.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(ProductDto {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        price: row.get("price"),
        category_id: row.get("category_id"),
        image_url: row.get("image_url"),
        stock: row.get("stock"),
        created_at: row.get("created_at"),
    }))
}

async fn get_product(
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Path(id): Path<i32>,
) -> AppResult<Json<ProductDto>> {
    let row = sqlx::query("SELECT id, name, description, price, category_id, image_url, stock, created_at FROM products WHERE id = $1")
        .bind(id).fetch_optional(&ctx.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    let row = match row {
        Some(r) => r,
        None => return Err(AppError::NotFound("Product not found".into())),
    };
    Ok(Json(ProductDto {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        price: row.get("price"),
        category_id: row.get("category_id"),
        image_url: row.get("image_url"),
        stock: row.get("stock"),
        created_at: row.get("created_at"),
    }))
}

#[derive(Debug, Deserialize)]
struct UpdateProduct {
    name: Option<String>,
    description: Option<String>,
    price: Option<f64>,
    category_id: Option<i32>,
    stock: Option<i32>,
    image_url: Option<String>,
}

async fn update_product(
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateProduct>,
) -> AppResult<Json<ProductDto>> {
    let row = sqlx::query("UPDATE products SET name = COALESCE($2, name), description = COALESCE($3, description), price = COALESCE($4, price), category_id = COALESCE($5, category_id), stock = COALESCE($6, stock), image_url = COALESCE($7, image_url) WHERE id = $1 RETURNING id, name, description, price, category_id, image_url, stock, created_at")
        .bind(id).bind(&req.name).bind(&req.description).bind(&req.price).bind(&req.category_id).bind(&req.stock).bind(&req.image_url)
        .fetch_one(&ctx.db_pool).await.map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(Json(ProductDto {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        price: row.get("price"),
        category_id: row.get("category_id"),
        image_url: row.get("image_url"),
        stock: row.get("stock"),
        created_at: row.get("created_at"),
    }))
}

async fn delete_product(
    Extension(ctx): Extension<std::sync::Arc<AppContext>>,
    Path(id): Path<i32>,
) -> AppResult<Json<serde_json::Value>> {
    let res = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(id)
        .execute(&ctx.db_pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound("Product not found".into()));
    }
    Ok(Json(serde_json::json!({"deleted": true})))
}

async fn upload_image(
    Extension(_ctx): Extension<std::sync::Arc<AppContext>>,
    mut multipart: Multipart,
) -> AppResult<Json<serde_json::Value>> {
    let upload_dir = std::path::Path::new("./uploads");
    if !upload_dir.exists() {
        fs::create_dir_all(upload_dir).map_err(|e| AppError::Internal(e.to_string()))?;
    }

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?
    {
        let file_name = field
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let file_path = upload_dir.join(&file_name);
        let data = field
            .bytes()
            .await
            .map_err(|e| AppError::BadRequest(e.to_string()))?;
        tokio::fs::write(&file_path, &data)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
        let url = format!("/uploads/{}", file_name);
        return Ok(Json(serde_json::json!({"url": url})));
    }
    Err(AppError::BadRequest("No file provided".into()))
}
