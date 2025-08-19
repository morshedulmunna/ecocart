use sqlx::{Row, postgres::PgPoolOptions};

use crate::configs::db_config::DatabaseConfig;

/// Seed sample products mapped to existing categories.
///
/// The function accepts a list of `(id, name)` category tuples, typically
/// returned by `seed_categories`, and inserts several products associated with
/// those categories. It is idempotent by product name. Also seeds a sample
/// `image_url` for each product pointing to a bundled placeholder image.
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

    // Define a small catalog of eco-friendly products with placeholder image URLs
    // Using a bundled placeholder that is available under `/uploads/` to avoid broken links in dev
    let placeholder_image: &str = "/uploads/Untitled-designeee-570x478.png";
    let catalog: Vec<(&str, Option<&str>, f64, i32, Option<&str>)> = vec![
        (
            "Bamboo Toothbrush",
            Some("Biodegradable handle, soft bristles"),
            3.99,
            250,
            Some(placeholder_image),
        ),
        (
            "Reusable Metal Straw Set",
            Some("Includes brush and pouch"),
            5.49,
            300,
            Some(placeholder_image),
        ),
        (
            "Organic Cotton Tote",
            Some("Durable, machine washable"),
            9.99,
            150,
            Some(placeholder_image),
        ),
        (
            "Solar Power Bank",
            Some("10000mAh portable charger"),
            29.9,
            80,
            Some(placeholder_image),
        ),
        (
            "Compostable Trash Bags",
            Some("Strong and leak-resistant"),
            12.5,
            120,
            Some(placeholder_image),
        ),
        (
            "Refillable Glass Soap Dispenser",
            Some("Amber glass, pump included"),
            15.0,
            60,
            Some(placeholder_image),
        ),
        (
            "Beeswax Food Wraps",
            Some("Set of 3 sizes"),
            11.25,
            140,
            Some(placeholder_image),
        ),
        (
            "LED Energy-Saving Bulb",
            Some("Warm white, 9W"),
            4.75,
            400,
            Some(placeholder_image),
        ),
    ];

    for (idx, (name, description, price, stock, image_url)) in catalog.into_iter().enumerate() {
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
                "UPDATE products SET description = $2, price = $3, category_id = $4, stock = $5, image_url = $6 WHERE id = $1",
            )
            .bind(pid)
            .bind(description)
            .bind(price)
            .bind(category_id)
            .bind(stock)
            .bind(image_url)
            .execute(&pool)
            .await?;
        } else {
            // Insert new product
            sqlx::query(
                "INSERT INTO products (name, description, price, category_id, stock, image_url) VALUES ($1, $2, $3, $4, $5, $6)",
            )
            .bind(name)
            .bind(description)
            .bind(price)
            .bind(category_id)
            .bind(stock)
            .bind(image_url)
            .execute(&pool)
            .await?;
        }
    }

    println!("Seed: products ready");
    Ok(())
}
