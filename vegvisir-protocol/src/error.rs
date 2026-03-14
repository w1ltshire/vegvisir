use thiserror::Error;

/// Possible errors
#[derive(Error, Debug)]
pub enum Error {
	/// [`postcard`] serialization error
	#[error(transparent)]
	Serialization(#[from] postcard::Error),
	/// Buffer capacity error
	#[error(transparent)]
	Capacity(#[from] heapless::CapacityError)
}

/// Result type used by this crate
pub type ProtocolResult<T> = Result<T, Error>;