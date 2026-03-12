use embassy_executor::Spawner;
use embassy_stm32::Peripherals;
use crate::drivers::error::DriverError;

/// Possible driver errors
pub mod error;
/// GPS driver
pub mod gps;

/// Result type for convenience
pub type DriverResult<T> = Result<T, DriverError>;

/// Trait that all drivers must implement
pub trait Driver {
	/// Initialize the driver state and return `Self`
	fn init(peripherals: Peripherals) -> Self;

	/// Spawn whatever tasks this driver has
	fn spawn(self, spawner: &Spawner) -> DriverResult<()>;
}