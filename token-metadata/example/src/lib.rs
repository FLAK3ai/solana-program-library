//! Crate defining an example program for storing SPL token metadata

#![allow(clippy::integer_arithmetic)]
#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod processor;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
