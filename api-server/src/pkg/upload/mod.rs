use std::path::Path;
use tokio::fs;

use crate::pkg::error::AppError;

/// Save bytes to a local uploads directory and return relative URL path
pub async fn save_bytes(bytes: &[u8], original_name: Option<&str>) -> Result<String, AppError> {
    let dir = Path::new("./uploads");
    if !dir.exists() {
        fs::create_dir_all(dir)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;
    }
    let filename = original_name
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{}", uuid::Uuid::new_v4()));
    let path = dir.join(&filename);
    fs::write(&path, bytes)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    Ok(format!("/uploads/{}", filename))
}
