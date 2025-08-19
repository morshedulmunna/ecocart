use std::env;

use sqlx::{Row, postgres::PgPoolOptions};

use crate::configs::db_config::DatabaseConfig;
use crate::pkg::security::hash_password;

/// Seed an admin user into the `users` table.
///
/// Behavior:
/// - If `ADMIN_EMAIL` exists in DB:
///   - If role != "admin", promote to admin and update password hash
///   - If already admin, no-op (idempotent)
/// - If not exists, insert a new admin with provided username/email/password
///   - If the desired username is taken, append a timestamp suffix to make it unique
pub async fn seed_admin() -> Result<(), Box<dyn std::error::Error>> {
    // Load env (supports local runs) and build a short-lived pool
    dotenv::dotenv().ok();
    let db = DatabaseConfig::load_from_env()?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db.database_url)
        .await?;

    let admin_email = env::var("ADMIN_EMAIL").unwrap_or_else(|_| "admin@ecocart.local".to_string());
    let admin_username = env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string());
    let admin_password = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".to_string());

    let password_hash = hash_password(&admin_password).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::Other, format!("hash error: {}", e))
    })?;

    // Check if a user with the admin email already exists
    if let Some(row) = sqlx::query("SELECT id, role FROM users WHERE email = $1")
        .bind(&admin_email)
        .fetch_optional(&pool)
        .await?
    {
        let current_role: String = row.get("role");
        if current_role != "admin" {
            // Promote to admin and rotate password
            sqlx::query("UPDATE users SET role = 'admin', password_hash = $1 WHERE email = $2")
                .bind(&password_hash)
                .bind(&admin_email)
                .execute(&pool)
                .await?;
            println!(
                "Seed: existing user with email {} promoted to admin",
                admin_email
            );
        } else {
            println!("Seed: admin user already present; nothing to do");
        }
        return Ok(());
    }

    // Ensure username is unique; if taken, append a short suffix
    let desired_username = if sqlx::query("SELECT 1 FROM users WHERE username = $1")
        .bind(&admin_username)
        .fetch_optional(&pool)
        .await?
        .is_some()
    {
        format!("{}{}", admin_username, chrono::Utc::now().timestamp())
    } else {
        admin_username
    };

    let rec = sqlx::query(
		"INSERT INTO users (username, email, password_hash, role) VALUES ($1, $2, $3, 'admin') RETURNING id",
	)
	.bind(&desired_username)
	.bind(&admin_email)
	.bind(&password_hash)
	.fetch_one(&pool)
	.await?;
    let _id: uuid::Uuid = rec.get("id");
    println!(
        "Seed: created admin user '{}' <{}>",
        desired_username, admin_email
    );

    Ok(())
}
