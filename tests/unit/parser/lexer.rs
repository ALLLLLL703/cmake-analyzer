//! Lexer test modules for construction, argument delimiters, comments, errors, token positions and
//! borrowed payload text. Tests scan in-memory strings only.

// Contains AI-generated documentation edits.
// Contains AI-generated test scaffolding edits.

#[cfg(test)]
pub mod advance;
#[cfg(test)]
mod bracket;
mod comments;
#[cfg(test)]
mod errors;
#[cfg(test)]
mod parsing;
#[cfg(test)]
mod spans;
#[cfg(test)]
mod text;
