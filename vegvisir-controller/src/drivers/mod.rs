use crate::drivers::error::DriverError;

/// GPS driver
pub mod gps;
/// Possible driver errors
pub mod error;

/// Result type for convenience
pub type DriverResult<T> = Result<T, DriverError>;