//! Group lexical tokens into command calls without evaluating CMake.
//! [`model::NParser::parse`] forwards comments and lexical errors, then records
//! closed or partial commands. [`model::IncrementalNParser`] reuses unchanged
//! closed calls. Source spans are UTF-8 bytes, not LSP coordinates.
//!
//! Contains AI-generated documentation additions.

pub mod display;
pub mod model;
pub mod parsing;
mod text;
// AI-authored incremental implementation.
mod incremental;
