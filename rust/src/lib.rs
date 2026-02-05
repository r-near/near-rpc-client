//! Generated Rust types and client for NEAR Protocol JSON-RPC API.
//!
//! This crate provides:
//! - `types` module: All RPC request/response types generated from the OpenRPC schema
//! - `client` module (with `client` feature): A simple async RPC client
//!
//! # Example
//!
//! ```no_run
//! use near_rpc_client::{NearRpcClient, types::*};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = NearRpcClient::mainnet();
//!     let status = client.status().await.unwrap();
//!     println!("Chain ID: {:?}", status);
//! }
//! ```

pub mod types;

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "client")]
pub use client::NearRpcClient;

pub use types::*;
