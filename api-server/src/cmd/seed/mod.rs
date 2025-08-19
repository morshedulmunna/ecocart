//! Seed entrypoint aggregating multiple seed routines.
//!
//! Add additional seed modules (e.g., categories, products) and
//! call them from `seed_command` to seed more data.

pub mod admin;

/// Run all seed routines. Currently seeds the default admin user.
pub async fn seed_command() -> Result<(), Box<dyn std::error::Error>> {
    admin::seed_admin().await?;
    Ok(())
}
