use crate::interfaces::http;

/// Start the HTTP APIs server.
///
/// Loads configuration, establishes core connections, and serves until
/// a shutdown signal is received.
pub async fn apis_command() -> Result<(), Box<dyn std::error::Error>> {
    http::server::run().await?;

    Ok(())
}
