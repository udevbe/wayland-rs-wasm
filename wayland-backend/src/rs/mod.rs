//! Rust implementations of the Wayland backends

mod client_impl;

mod debug;
mod map;
pub(crate) mod socket;
mod wire;

/// Client-side rust implementation of a Wayland protocol backend
///
/// The main entrypoint is the [`Backend::connect`](client::Backend::connect) method.
#[path = "../client_api.rs"]
pub mod client;
