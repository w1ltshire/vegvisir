#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![warn(missing_docs)]
#![no_std]
extern crate alloc;

/// Module implementing async TCP transport for targets with `std` support
#[cfg(feature = "tcp")]
pub mod tcp;

use core::error::Error;

/// Transport trait for sending data to a peer
#[allow(async_fn_in_trait)]
pub trait Transport: Send + Sync {
    /// The error type for this transport
    type Error: Error;

    /// Send data to the peer
    async fn send(&mut self, data: &[u8]) -> Result<(), Self::Error>;

    /// Receive data from the peer
    async fn recv(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
}

/// Listener trait for accepting incoming connections (typically GCS side)
#[allow(async_fn_in_trait)]
pub trait Listener: Send + Sync {
    /// The error type for this listener
    type Error: Error;

    /// Waits for the next incoming connection and returns a boxed `Transport`.
    async fn accept(&self) -> Result<impl Transport, Self::Error>;
    /// Close the listener
    async fn close(self) -> Result<(), Self::Error>;
}