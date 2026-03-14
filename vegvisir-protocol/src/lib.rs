#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![warn(missing_docs)]
#![no_std]

/// Representing protocol packets
pub mod packet;
/// Possible errors
pub mod error;
/// Protocol messages
pub mod message;