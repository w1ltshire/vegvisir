use thiserror::Error;

/// Possible driver errors
#[derive(Error, Debug)]
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
