//! Static CMake analysis, snapshot formatting, configuration and explicit runtime execution.
//! [`parser`] and [`workspace`] analyze source; [`mod@format`] renders in memory.
//! [`cli`] assembles transports or disk formatting; [`runtime_cmake`] alone exposes project execution.
//! Importing the library starts no process and performs no I/O.
//!
//! Contains AI-generated documentation additions.
pub mod model;
pub mod parser;
// AI-generated implementation.
pub mod cli;
pub mod config;
pub mod format;
pub mod runtime_cmake;
pub mod server;
pub mod workspace;
