use crate::drivers::error::DriverError;

/// Possible driver errors
pub mod error;
/// GPS driver
pub mod gps;

/// Result type for convenience
pub type DriverResult<T> = Result<T, DriverError>;