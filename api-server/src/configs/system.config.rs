//! System/application configuration loader.
//!
//! Centralized, environment-driven configuration for the application runtime
//! (HTTP APIs, gRPC, logging, and graceful shutdowns). Values are sourced from
//! environment variables with sensible defaults for local development.

use std::env;

/// System-level configuration derived from environment variables.
///
/// Prefer using `SystemConfig::load_from_env()` to initialize this struct.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemConfig {
    /// Application environment (e.g. "development", "staging", "production").
    pub app_env: String,
    /// Application log level (e.g. "error", "warn", "info", "debug", "trace").
    pub log_level: String,
    /// HTTP API bind host (e.g. "0.0.0.0").
    pub api_host: String,
    /// HTTP API bind port.
    pub api_port: u16,
    /// gRPC bind host.
    pub grpc_host: String,
    /// gRPC bind port.
    pub grpc_port: u16,
    /// Graceful shutdown timeout in seconds.
    pub shutdown_grace_seconds: u64,
}

impl SystemConfig {
    /// Load system configuration from environment variables.
    ///
    /// Supported variables (with defaults):
    /// - `APP_ENV` → default: "development"
    /// - `LOG_LEVEL` → default: "info"
    /// - `API_HOST` → default: "127.0.0.1"
    /// - `API_PORT` → default: 8080
    /// - `GRPC_HOST` → default: "127.0.0.1"
    /// - `GRPC_PORT` → default: 50051
    /// - `SHUTDOWN_GRACE_SECONDS` → default: 10
    pub fn load_from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let app_env = env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        let api_host = env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let api_port = parse_port_with_default("API_PORT", 8080)?;
        let grpc_host = env::var("GRPC_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let grpc_port = parse_port_with_default("GRPC_PORT", 50051)?;
        let shutdown_grace_seconds = parse_u64_with_default("SHUTDOWN_GRACE_SECONDS", 10)?;

        Ok(Self {
            app_env,
            log_level,
            api_host,
            api_port,
            grpc_host,
            grpc_port,
            shutdown_grace_seconds,
        })
    }
}

/// Parse a `u16` port from an environment variable, applying a default when missing.
fn parse_port_with_default(
    var: &str,
    default_value: u16,
) -> Result<u16, Box<dyn std::error::Error>> {
    match env::var(var) {
        Ok(val) => {
            let parsed: u16 = val
                .parse()
                .map_err(|_| format!("{} must be a valid u16 port", var))?;
            Ok(parsed)
        }
        Err(_) => Ok(default_value),
    }
}

/// Parse a `u64` from an environment variable, applying a default when missing.
fn parse_u64_with_default(
    var: &str,
    default_value: u64,
) -> Result<u64, Box<dyn std::error::Error>> {
    match env::var(var) {
        Ok(val) => {
            let parsed: u64 = val
                .parse()
                .map_err(|_| format!("{} must be a valid u64", var))?;
            Ok(parsed)
        }
        Err(_) => Ok(default_value),
    }
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
    fn loads_defaults_when_unset() {
        clear_vars(&[
            "APP_ENV",
            "LOG_LEVEL",
            "API_HOST",
            "API_PORT",
            "GRPC_HOST",
            "GRPC_PORT",
            "SHUTDOWN_GRACE_SECONDS",
        ]);

        let cfg = SystemConfig::load_from_env().expect("should load defaults");
        assert_eq!(cfg.app_env, "development");
        assert_eq!(cfg.log_level, "info");
        assert_eq!(cfg.api_host, "127.0.0.1");
        assert_eq!(cfg.api_port, 8080);
        assert_eq!(cfg.grpc_host, "127.0.0.1");
        assert_eq!(cfg.grpc_port, 50051);
        assert_eq!(cfg.shutdown_grace_seconds, 10);
    }

    #[test]
    #[serial]
    fn loads_custom_values() {
        clear_vars(&[
            "APP_ENV",
            "LOG_LEVEL",
            "API_HOST",
            "API_PORT",
            "GRPC_HOST",
            "GRPC_PORT",
            "SHUTDOWN_GRACE_SECONDS",
        ]);

        unsafe {
            env::set_var("APP_ENV", "production");
            env::set_var("LOG_LEVEL", "warn");
            env::set_var("API_HOST", "0.0.0.0");
            env::set_var("API_PORT", "9000");
            env::set_var("GRPC_HOST", "0.0.0.0");
            env::set_var("GRPC_PORT", "6000");
            env::set_var("SHUTDOWN_GRACE_SECONDS", "30");
        }

        let cfg = SystemConfig::load_from_env().expect("should load custom values");
        assert_eq!(cfg.app_env, "production");
        assert_eq!(cfg.log_level, "warn");
        assert_eq!(cfg.api_host, "0.0.0.0");
        assert_eq!(cfg.api_port, 9000);
        assert_eq!(cfg.grpc_host, "0.0.0.0");
        assert_eq!(cfg.grpc_port, 6000);
        assert_eq!(cfg.shutdown_grace_seconds, 30);
    }

    #[test]
    #[serial]
    fn invalid_ports_error() {
        clear_vars(&["API_PORT", "GRPC_PORT"]);
        unsafe {
            env::set_var("API_PORT", "not-a-number");
        }
        let err = SystemConfig::load_from_env().unwrap_err();
        assert!(format!("{}", err).contains("API_PORT"));

        clear_vars(&["API_PORT", "GRPC_PORT"]);
        unsafe {
            env::set_var("GRPC_PORT", "-1");
        }
        let err = SystemConfig::load_from_env().unwrap_err();
        assert!(format!("{}", err).contains("GRPC_PORT"));
    }
}
