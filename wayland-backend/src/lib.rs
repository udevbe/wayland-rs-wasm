//! Backend API for wayland crates
//!
//! This crate provide low-level APIs for interacting with the Wayland protocol,
//! both client-side and server-side.
//!
//! Two possible backends are provided by this crate: the system backend ([`sys`] module)
//! which relies on the system-provided wayland libraries, and the rust backend ([`rs`] module)
//! which is an alternative rust implementation of the protocol. The rust backend is always
//! available, and the system backend is controlled by the `client_system` and `server_system`
//! cargo features. The `dlopen` cargo feature ensures that the system wayland
//! libraries are loaded dynamically at runtime, so that your executable does not link them and
//! can gracefully handle their absence (for example by falling back to X11).
//!
//! Additionally the default backends are reexported as toplevel `client` and `server` modules
//! in this crate. For both client and server, the default backend is the system one if the
//! associated cargo feature is enabled, and the rust one otherwise.
//!
//! Using these reexports is the recommended way to use the crate:
//! - If you don't need the `*_system` features, an other crate enabling them will not impact your code in
//!   any way as both backends have the same API (the system backend only has more methods)
//! - If your code needs to do FFI, you just need to directly depend on `wayland-backend` with the
//!   appropriate `*_system` feature enabled, and all the other crates in your dependency tree will
//!   automatically use the `sys` backend.
//!
//! Both the `wayland-client` and `wayland-server` crates follow this principle, so everything will "Just
//! Work" when using them.
//!
//! ## Logging
//!
//! This crate can generate some runtime error message (notably when a protocol error occurs). By default
//! those messages are printed to stderr. If you activate the `log` cargo feature, they will instead be
//! piped through the `log` crate.
//!
//! ## raw-window-handle integration
//!
//! This crate can implement [`HasRawWindowHandle`](raw_window_handle::HasRawWindowHandle) for the client
//! module [`Backend`](client::Backend) type if you activate the `raw-window-handle` feature.
//!
//! Note that the `client_system` feature must also be enabled for the implementation to be activated.

#![forbid(improper_ctypes)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(missing_docs, missing_debug_implementations)]
// The api modules are imported two times each, this is not accidental
#![allow(clippy::duplicate_mod)]
#![cfg_attr(coverage, feature(no_coverage))]
// Doc feature labels can be tested locally by running RUSTDOCFLAGS="--cfg=docsrs" cargo +nightly doc -p <crate>
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub extern crate io_lifetimes;
pub extern crate smallvec;

/// Helper macro for quickly making a [`Message`](crate::protocol::Message)
#[macro_export]
macro_rules! message {
    ($sender_id: expr, $opcode: expr, [$($args: expr),* $(,)?] $(,)?) => {
        $crate::protocol::Message {
            sender_id: $sender_id,
            opcode: $opcode,
            args: $crate::smallvec::smallvec![$($args),*],
        }
    }
}

// internal imports for dispatching logging depending on the `log` feature
#[cfg(feature = "log")]
#[allow(unused_imports)]
use log::{debug as log_debug, error as log_error, info as log_info, warn as log_warn};
#[cfg(not(feature = "log"))]
#[allow(unused_imports)]
use std::{
    eprintln as log_error, eprintln as log_warn, eprintln as log_info, eprintln as log_debug,
};

pub mod rs;

pub use rs::client;

#[cfg(test)]
mod test;

mod core_interfaces;
pub mod protocol;
mod types;
