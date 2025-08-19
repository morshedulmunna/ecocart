use ecocart::pkg::logger::{debug, error, info, warn};

// Log anything that implements Display
info("Hello world");
debug(42);
warn(3.14);
error(true);

// Log custom structs (if they implement Display)
info(my_custom_struct);
debug(my_enum);

// Log with formatting using macros
use ecocart::{log_info, log_debug, log_warn, log_error};

log_info!("User {} logged in", username);
log_debug!("Processing {} items", count);
log_warn!("Temperature is {}°C", temp);
log_error!("Failed to connect: {}", error_msg);
