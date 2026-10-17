//! Syntax and analysis stages for CMake source. No stage here executes project code.
//! [`lexer`] records tokens/comments, [`nparser`] groups command calls, and
//! [`semanticer`] interprets supported structure before [`build_struct`] extracts facts.
//! [`interpreter_commands`] recognizes supported CLI forms without executing them.
//!
//! Contains AI-generated documentation additions.

pub mod lexer;
pub mod nparser;
pub mod semanticer;
// AI-generated implementation.
pub mod build_struct;
pub mod interpreter_commands;
