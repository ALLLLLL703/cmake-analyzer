// AI-authored analyzer LSP integration.
mod action;
mod configuration;
mod diagnostics;
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
#[cfg(test)]
#[path = "../tests/unit/server.rs"]
mod tests;
