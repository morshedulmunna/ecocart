use crate::pkg::logger::Logger;
use once_cell::sync::Lazy;
use std::sync::Arc;

// Global logger instance
static GLOBAL_LOGGER: Lazy<Arc<Logger>> = Lazy::new(|| Arc::new(Logger::new("ecocart")));

/// Get the global logger instance
pub fn get_logger() -> Arc<Logger> {
    Arc::clone(&GLOBAL_LOGGER)
}

/// Initialize the global logger with a custom service name
pub fn init_logger(_service_name: &str) {
    // This will replace the global logger instance
    // Note: This is a simplified approach. In a more complex scenario,
    // you might want to use a Mutex or RwLock for thread-safe replacement
    let _ = Lazy::force(&GLOBAL_LOGGER);
}

// Simple and flexible logging functions - you can pass anything that implements Display
#[track_caller]
pub fn debug<T: std::fmt::Display>(message: T) {
    GLOBAL_LOGGER.debug(&message.to_string());
}

#[track_caller]
pub fn info<T: std::fmt::Display>(message: T) {
    GLOBAL_LOGGER.info(&message.to_string());
}

#[track_caller]
pub fn warn<T: std::fmt::Display>(message: T) {
    GLOBAL_LOGGER.warn(&message.to_string());
}

#[track_caller]
pub fn error<T: std::fmt::Display>(message: T) {
    GLOBAL_LOGGER.error(&message.to_string());
}

#[track_caller]
pub fn error_with_details<T: std::fmt::Display, E: std::fmt::Display>(message: T, error: E) {
    GLOBAL_LOGGER.error_with_details(&message.to_string(), &error.to_string());
}

// Flexible macros that can format anything
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        {
            #[track_caller]
            fn __log_debug_forward(msg: String) {
                $crate::pkg::logger::global::debug(msg);
            }
            __log_debug_forward(format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        {
            #[track_caller]
            fn __log_info_forward(msg: String) {
                $crate::pkg::logger::global::info(msg);
            }
            __log_info_forward(format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        {
            #[track_caller]
            fn __log_warn_forward(msg: String) {
                $crate::pkg::logger::global::warn(msg);
            }
            __log_warn_forward(format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        {
            #[track_caller]
            fn __log_error_forward(msg: String) {
                $crate::pkg::logger::global::error(msg);
            }
            __log_error_forward(format!($($arg)*))
        }
    };
}

#[macro_export]
macro_rules! log_error_with_details {
    ($message:expr, $error:expr) => {{
        #[track_caller]
        fn __log_error_with_details_forward<M: std::fmt::Display, E: std::fmt::Display>(
            m: M,
            e: E,
        ) {
            $crate::pkg::logger::global::error_with_details(m, e);
        }
        __log_error_with_details_forward($message, $error)
    }};
}
