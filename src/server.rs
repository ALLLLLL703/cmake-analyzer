//! Connect CMake analysis to the language-server transport.
//! [`CmakeAnalyzer`] exposes the protocol implementation; [`ServerOptions`] supplies
//! startup inputs. A single service owns mutable projects and editor overlays.
//! Initialization reads configuration and discovers roots; only explicitly authorized
//! runtime actions execute project CMake code. Replies, diagnostics and logs go through
//! the client handle, never direct protocol writes to stdout.
//!
//! ```text
//!  [Transport handlers]           [Client capabilities]
//!           |                             |
//!           +------------+----------------+
//!                        v
//!               [Bounded service queue]
//!                        |
//!                        v
//!               [One project owner] ------> [Client replies/reports]
//!                  |           |
//!                  v           v
//!           [Editor overlays] [Disk snapshots]
//! ```
// AI-authored analyzer LSP integration.
mod action;
pub mod completion;
mod configuration;
mod diagnostics;
mod formatting;
mod handlers;
pub mod hovers;
pub mod model;
mod navigation;
mod paths;
pub mod roots;
mod service;
mod symbols;
mod text;
mod uri;
pub use model::{CmakeAnalyzer, ServerOptions};
// AI-authored test wiring; adapter tests stay outside production source.
/// Private adapter regression tests loaded from the external test file.
/// Tests exercise coordinate/URI/root behavior without exposing production internals.
#[cfg(test)]
#[path = "../tests/unit/server.rs"]
mod tests;
