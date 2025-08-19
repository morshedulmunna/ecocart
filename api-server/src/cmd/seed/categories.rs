use sqlx::{Row, postgres::PgPoolOptions};

use crate::configs::db_config::DatabaseConfig;

/// Seed default product categories into the `categories` table.
///
/// Idempotent: uses ON CONFLICT(name) to avoid duplicates and returns IDs for
/// both newly inserted and existing rows.
pub async fn seed_categories() -> Result<Vec<(i32, String)>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let db = DatabaseConfig::load_from_env()?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db.database_url)
        .await?;

    let seed_items: Vec<(&str, Option<&str>)> = vec![
        ("Eco-Friendly Home", Some("Sustainable household products")),
        ("Sustainable Fashion", Some("Clothing and accessories")),
        ("Zero Waste", Some("Reusable and refillable essentials")),
        ("Organic Groceries", Some("Certified organic food items")),
        (
            "Renewable Tech",
            Some("Solar, wind and power-saving gadgets"),
        ),
    ];

    let mut results: Vec<(i32, String)> = Vec::with_capacity(seed_items.len());
    for (name, description) in seed_items {
        let row = sqlx::query(
            "INSERT INTO categories (name, description) VALUES ($1, $2)
             ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
             RETURNING id, name",
        )
        .bind(name)
        .bind(description)
        .fetch_one(&pool)
        .await?;

        let id: i32 = row.get("id");
        let name: String = row.get("name");
        results.push((id, name));
    }

    println!("Seed: categories ready ({} total)", results.len());
    Ok(results)
}
