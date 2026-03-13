use core::fmt::Debug;
use defmt::{Formatter, write};
use thiserror::__private18::AsDisplay;
use thiserror::Error;

/// Possible driver errors
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DriverError {
    /// Vec/String is too small
    #[error(transparent)]
    Capacity(#[from] heapless::CapacityError),
    /// Failed to parse UTF-8
    #[error(transparent)]
    Utf8(#[from] core::str::Utf8Error),
    /// UART error
    #[error(transparent)]
    Serial(#[from] embassy_stm32::usart::Error),
    /// Failed to spawn a task
    #[error(transparent)]
    Spawn(#[from] embassy_executor::SpawnError),
}

impl defmt::Format for DriverError {
    fn format(&self, fmt: Formatter) {
        match self {
            DriverError::Capacity(_) => write!(fmt, "insufficient capacity"),
            DriverError::Utf8(_) => write!(fmt, "utf8 parsing error"),
            DriverError::Serial(e) => write!(fmt, "{}", e.as_display()),
            DriverError::Spawn(e) => write!(fmt, "{}", e.as_display()),
        }
    }
}