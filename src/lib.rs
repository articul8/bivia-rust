//! # Bivia SDK
//!
//! Rust client for the [Bivia](https://api.articul8.ai/bivia) calibrated
//! decision engine. Provides typed, async methods for every endpoint.

pub mod client;
pub mod error;
pub mod resources;
pub mod types;

pub use client::Client;
pub use error::BiviaError;
