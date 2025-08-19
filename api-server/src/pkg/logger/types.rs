use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRecord {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub caller: String,
    pub message: String,
    pub service: String,
    pub log_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(flatten)]
    pub extra_fields: HashMap<String, serde_json::Value>,
}

impl LogRecord {
    pub fn new(
        level: LogLevel,
        caller: &str,
        message: &str,
        service: &str,
        error: Option<&str>,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            caller: caller.to_string(),
            message: message.to_string(),
            service: service.to_string(),
            log_id: uuid::Uuid::new_v4().to_string(),
            error: error.map(|e| e.to_string()),
            extra_fields: HashMap::new(),
        }
    }

    pub fn with_field(mut self, key: &str, value: serde_json::Value) -> Self {
        self.extra_fields.insert(key.to_string(), value);
        self
    }

    pub fn with_fields(mut self, fields: HashMap<String, serde_json::Value>) -> Self {
        self.extra_fields.extend(fields);
        self
    }
}
