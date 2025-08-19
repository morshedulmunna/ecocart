/// Run SQLx database migrations located in `./migrations` against `DATABASE_URL`.
///
/// This connects to the Postgres database and runs any pending migrations.
/// It expects the environment variable `DATABASE_URL` to be set.
pub async fn migrate_command() -> Result<(), Box<dyn std::error::Error>> {
    use dotenv::dotenv;
    use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
    use std::{env, path::PathBuf};

    // Load .env if present and read DATABASE_URL; fail fast with a helpful error
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment variable file!");

    // Create a small connection pool; migrations are short-lived
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Resolve migrations directory with sensible defaults for local dev and containers
    // Priority: MIGRATIONS_DIR env -> ./migrations (cwd) -> /app/migrations
    let migrations_dir: PathBuf = env::var("MIGRATIONS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            let rel = cwd.join("migrations");
            if rel.is_dir() {
                rel
            } else {
                PathBuf::from("/app/migrations")
            }
        });

    // Run filesystem migrations (not embedded) so they can be edited without rebuild
    let migrator = Migrator::new(migrations_dir.as_path()).await?;
    migrator.run(&pool).await?;

    println!("Migrations applied successfully");
    Ok(())
}
