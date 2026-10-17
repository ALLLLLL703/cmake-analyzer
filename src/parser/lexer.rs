//! UTF-8 byte scanning and source-backed token views; no I/O.
//! [`model::Lexer::parse`] is the full scanner; [`model::IncrementalLexer`]
//! resumes the same scanner at whole-token/comment checkpoints. Quoted and bracket
//! payloads never contribute parentheses or comments to outer scan state.
//!
//! Contains AI-generated documentation additions.

mod bracket;
// AI-authored incremental implementation.
pub mod display;
mod incremental;
pub mod model;
pub mod parsing;
mod text;
