use thiserror::Error;

#[derive(Error, Debug)]
pub enum DriverError {
    #[error(transparent)]
    Capacity(#[from] heapless::CapacityError),
    #[error(transparent)]
    Utf8(#[from] core::str::Utf8Error),
    #[error(transparent)]
    Serial(#[from] embassy_stm32::usart::Error),
    #[error(transparent)]
    Spawn(#[from] embassy_executor::SpawnError)
}