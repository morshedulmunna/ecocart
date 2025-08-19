use sqlx::{Row, postgres::PgPoolOptions};

use crate::configs::db_config::DatabaseConfig;

/// Seed sample products mapped to existing categories.
///
/// The function accepts a list of `(id, name)` category tuples, typically
/// returned by `seed_categories`, and inserts several products associated with
/// those categories. It is idempotent by product name.
pub async fn seed_products(categories: &[(i32, String)]) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let db = DatabaseConfig::load_from_env()?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db.database_url)
        .await?;

    if categories.is_empty() {
        println!("Seed: no categories available; skipping products");
        return Ok(());
    }

    // Define a small catalog of eco-friendly products
    let catalog: Vec<(&str, Option<&str>, f64, i32)> = vec![
        (
            "Bamboo Toothbrush",
            Some("Biodegradable handle, soft bristles"),
            3.99,
            250,
        ),
        (
            "Reusable Metal Straw Set",
            Some("Includes brush and pouch"),
            5.49,
            300,
        ),
        (
            "Organic Cotton Tote",
            Some("Durable, machine washable"),
            9.99,
            150,
        ),
        (
            "Solar Power Bank",
            Some("10000mAh portable charger"),
            29.9,
            80,
        ),
        (
            "Compostable Trash Bags",
            Some("Strong and leak-resistant"),
            12.5,
            120,
        ),
        (
            "Refillable Glass Soap Dispenser",
            Some("Amber glass, pump included"),
            15.0,
            60,
        ),
        ("Beeswax Food Wraps", Some("Set of 3 sizes"), 11.25, 140),
        ("LED Energy-Saving Bulb", Some("Warm white, 9W"), 4.75, 400),
    ];

    for (idx, (name, description, price, stock)) in catalog.into_iter().enumerate() {
        // Round-robin assign category by index for deterministic behavior
        let (category_id, _category_name) = categories[idx % categories.len()].clone();

        // Check if product by name exists
        let existing = sqlx::query("SELECT id FROM products WHERE name = $1")
            .bind(name)
            .fetch_optional(&pool)
            .await?;

        if let Some(row) = existing {
            let pid: i32 = row.get("id");
            // Update existing product
            sqlx::query(
                "UPDATE products SET description = $2, price = $3, category_id = $4, stock = $5 WHERE id = $1",
            )
            .bind(pid)
            .bind(description)
            .bind(price)
            .bind(category_id)
            .bind(stock)
            .execute(&pool)
            .await?;
        } else {
            // Insert new product
            sqlx::query(
                "INSERT INTO products (name, description, price, category_id, stock) VALUES ($1, $2, $3, $4, $5)",
            )
            .bind(name)
            .bind(description)
            .bind(price)
            .bind(category_id)
            .bind(stock)
            .execute(&pool)
            .await?;
        }
    }

    println!("Seed: products ready");
    Ok(())
}
