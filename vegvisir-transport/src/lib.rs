#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![warn(missing_docs)]
#![no_std]

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