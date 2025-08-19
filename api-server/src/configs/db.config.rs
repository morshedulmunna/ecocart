//! Database configuration loader.
//!
//! Loads database settings from environment variables, pulling from a `.env`
//! file when present. Prefer `DATABASE_URL` directly; if absent, construct a
//! Postgres URL from individual `DB_*` variables.

use std::env;

/// Database configuration derived from environment variables.
///
/// Prefer using `DatabaseConfig::load_from_env()` to initialize this struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseConfig {
    /// Full connection string to the Postgres database.
    pub database_url: String,
}

impl DatabaseConfig {
    /// Load database configuration from environment variables.
    ///
    /// Order of precedence:
    /// - If `DATABASE_URL` is set, use it as-is.
    /// - Otherwise, try to construct it from `DB_USER`, `DB_PASSWORD`, `DB_HOST`,
    ///   `DB_PORT` (default 5432), `DB_NAME`, and optional `DB_SSLMODE` (default "disable").
    ///
    /// Returns an error if neither `DATABASE_URL` is set nor the required `DB_*`
    /// variables are sufficient to construct a valid URL.
    pub fn load_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        if let Ok(url) = env::var("DATABASE_URL") {
            if url.trim().is_empty() {
                return Err("DATABASE_URL is set but empty".into());
            }
            return Ok(Self { database_url: url });
        }

        let host =
            env::var("DB_HOST").map_err(|_| "DB_HOST must be set or provide DATABASE_URL")?;
        let port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
        let user =
            env::var("DB_USER").map_err(|_| "DB_USER must be set or provide DATABASE_URL")?;
        let password = env::var("DB_PASSWORD").unwrap_or_default();
        let name =
            env::var("DB_NAME").map_err(|_| "DB_NAME must be set or provide DATABASE_URL")?;
        let sslmode = env::var("DB_SSLMODE").unwrap_or_else(|_| "disable".to_string());

        let url = build_postgres_url(&host, &port, &user, &password, &name, &sslmode);
        Ok(Self { database_url: url })
    }
}

/// Construct a Postgres connection URL from individual pieces.
///
/// This function keeps things simple and does not perform percent-encoding of
/// credentials. Ensure your credentials are URL-safe.
fn build_postgres_url(
    host: &str,
    port: &str,
    user: &str,
    password: &str,
    database: &str,
    sslmode: &str,
) -> String {
    let auth = if password.is_empty() {
        format!("{}", user)
    } else {
        format!("{}:{}", user, password)
    };

    // Include sslmode for predictable behavior across environments
    format!(
        "postgres://{}@{}:{}/{}?sslmode={}",
        auth, host, port, database, sslmode
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    fn clear_vars(keys: &[&str]) {
        for &k in keys {
            unsafe {
                env::remove_var(k);
            }
        }
    }

    #[test]
    #[serial]
    fn loads_direct_database_url() {
        clear_vars(&[
            "DATABASE_URL",
            "DB_HOST",
            "DB_PORT",
            "DB_USER",
            "DB_PASSWORD",
            "DB_NAME",
            "DB_SSLMODE",
        ]);
        unsafe {
            env::set_var(
                "DATABASE_URL",
                "postgres://u:p@localhost:5432/db?sslmode=disable",
            );
        }

        let cfg = DatabaseConfig::load_from_env().expect("should load DATABASE_URL");
        assert_eq!(
            cfg.database_url,
            "postgres://u:p@localhost:5432/db?sslmode=disable"
        );
    }

    #[test]
    #[serial]
    fn builds_url_from_parts() {
        clear_vars(&[
            "DATABASE_URL",
            "DB_HOST",
            "DB_PORT",
            "DB_USER",
            "DB_PASSWORD",
            "DB_NAME",
            "DB_SSLMODE",
        ]);
        unsafe {
            env::set_var("DB_HOST", "127.0.0.1");
            env::set_var("DB_PORT", "5433");
            env::set_var("DB_USER", "alice");
            env::set_var("DB_PASSWORD", "secret");
            env::set_var("DB_NAME", "appdb");
            env::set_var("DB_SSLMODE", "disable");
        }

        let cfg = DatabaseConfig::load_from_env().expect("should build from parts");
        assert_eq!(
            cfg.database_url,
            "postgres://alice:secret@127.0.0.1:5433/appdb?sslmode=disable"
        );
    }

    #[test]
    #[serial]
    fn error_when_missing_required_parts() {
        clear_vars(&[
            "DATABASE_URL",
            "DB_HOST",
            "DB_PORT",
            "DB_USER",
            "DB_PASSWORD",
            "DB_NAME",
            "DB_SSLMODE",
        ]);
        unsafe {
            env::set_var("DB_USER", "bob");
            env::set_var("DB_NAME", "appdb");
        }
        // DB_HOST missing
        let err = DatabaseConfig::load_from_env().unwrap_err();
        assert!(format!("{}", err).contains("DB_HOST"));
    }
}
