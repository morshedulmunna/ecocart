//! Redis configuration loader.
//!
//! Loads Redis settings from environment variables, pulling from a `.env`
//! file when present. Prefer `REDIS_URL` directly; if absent, construct a
//! URL from individual `REDIS_*` variables.

use std::env;

/// Redis configuration derived from environment variables.
///
/// Prefer using `RedisConfig::load_from_env()` to initialize this struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedisConfig {
    /// Full connection string to the Redis instance.
    pub redis_url: String,
}

impl RedisConfig {
    /// Load Redis configuration from environment variables.
    ///
    /// Order of precedence:
    /// - If `REDIS_URL` is set, use it as-is.
    /// - Otherwise, construct it from: `REDIS_HOST` (default "127.0.0.1"),
    ///   `REDIS_PORT` (default 6379), optional `REDIS_USERNAME`, optional
    ///   `REDIS_PASSWORD`, optional `REDIS_DB` (default 0), and optional
    ///   `REDIS_TLS` (boolean, default false) to switch scheme to `rediss://`.
    pub fn load_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        if let Ok(url) = env::var("REDIS_URL") {
            if url.trim().is_empty() {
                return Err("REDIS_URL is set but empty".into());
            }
            return Ok(Self { redis_url: url });
        }

        let host = env::var("REDIS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = parse_u16_with_default("REDIS_PORT", 6379)?;
        let username = env::var("REDIS_USERNAME").ok();
        let password = env::var("REDIS_PASSWORD").ok();
        let db = parse_u32_with_default("REDIS_DB", 0)?;
        let use_tls = parse_bool_with_default("REDIS_TLS", false)?;

        let scheme = if use_tls { "rediss" } else { "redis" };
        let auth = match (username.as_deref(), password.as_deref()) {
            (Some(u), Some(p)) if !u.is_empty() || !p.is_empty() => format!("{}:{}@", u, p),
            (Some(u), None) if !u.is_empty() => format!("{}@", u),
            (None, Some(p)) if !p.is_empty() => format!(":{}@", p),
            _ => String::new(),
        };

        let url = format!("{}://{}{}:{}/{}", scheme, auth, host, port, db);
        Ok(Self { redis_url: url })
    }
}

fn parse_u16_with_default(
    var: &str,
    default_value: u16,
) -> Result<u16, Box<dyn std::error::Error>> {
    match env::var(var) {
        Ok(val) => {
            let parsed: u16 = val
                .parse()
                .map_err(|_| format!("{} must be a valid u16", var))?;
            Ok(parsed)
        }
        Err(_) => Ok(default_value),
    }
}

fn parse_u32_with_default(
    var: &str,
    default_value: u32,
) -> Result<u32, Box<dyn std::error::Error>> {
    match env::var(var) {
        Ok(val) => {
            let parsed: u32 = val
                .parse()
                .map_err(|_| format!("{} must be a valid u32", var))?;
            Ok(parsed)
        }
        Err(_) => Ok(default_value),
    }
}

fn parse_bool_with_default(
    var: &str,
    default_value: bool,
) -> Result<bool, Box<dyn std::error::Error>> {
    match env::var(var) {
        Ok(val) => match val.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Ok(true),
            "0" | "false" | "no" | "off" => Ok(false),
            _ => Err(format!("{} must be a boolean (true/false)", var).into()),
        },
        Err(_) => Ok(default_value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use serial_test::serial;

    fn clear_vars(keys: &[&str]) {
        for &k in keys {
            unsafe {
                env::remove_var(k);
            }
        }
    }

    #[test]
    #[serial]
    fn loads_direct_url() {
        clear_vars(&["REDIS_URL"]);
        unsafe {
            env::set_var("REDIS_URL", "redis://:p@localhost:6379/0");
        }
        let cfg = RedisConfig::load_from_env().expect("should load direct URL");
        assert_eq!(cfg.redis_url, "redis://:p@localhost:6379/0");
    }

    #[test]
    #[serial]
    fn builds_url_from_parts_defaults() {
        clear_vars(&[
            "REDIS_URL",
            "REDIS_HOST",
            "REDIS_PORT",
            "REDIS_USERNAME",
            "REDIS_PASSWORD",
            "REDIS_DB",
            "REDIS_TLS",
        ]);
        let cfg = RedisConfig::load_from_env().expect("should build from defaults");
        assert_eq!(cfg.redis_url, "redis://127.0.0.1:6379/0");
    }

    #[test]
    #[serial]
    fn builds_url_with_auth_tls_and_db() {
        clear_vars(&[
            "REDIS_URL",
            "REDIS_HOST",
            "REDIS_PORT",
            "REDIS_USERNAME",
            "REDIS_PASSWORD",
            "REDIS_DB",
            "REDIS_TLS",
        ]);
        unsafe {
            env::set_var("REDIS_HOST", "cache");
            env::set_var("REDIS_PORT", "6380");
            env::set_var("REDIS_USERNAME", "u");
            env::set_var("REDIS_PASSWORD", "p");
            env::set_var("REDIS_DB", "2");
            env::set_var("REDIS_TLS", "true");
        }
        let cfg = RedisConfig::load_from_env().expect("should build from parts");
        assert_eq!(cfg.redis_url, "rediss://u:p@cache:6380/2");
    }
}
