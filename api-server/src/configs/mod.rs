// Re-export the database config module from a file with a dot in its name.
// The file is named `db.config.rs`, so we use a path attribute to include it
// under a conventional Rust module name.

#[path = "db.config.rs"]
pub mod db_config;

// Re-export the system config module from a file with a dot in its name.
#[path = "system.config.rs"]
pub mod system_config;

// Re-export the redis config module from a file with a dot in its name.
#[path = "redis.config.rs"]
pub mod redis_config;

// App context module that aggregates configs and connections
pub mod app_context;
#[path = "auth.config.rs"]
pub mod auth_config;
