pub mod global;
pub mod logger;
pub mod types;

pub use global::{debug, error, error_with_details, get_logger, info, init_logger, warn};
pub use logger::Logger;
pub use types::{LogLevel, LogRecord};
