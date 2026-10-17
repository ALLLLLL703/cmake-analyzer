// Contains AI-authored documentation edits.
//! Stream-driven Language Server Protocol server with protocol-independent business models.
//!
//! Implement [`ILanguageServer`], configure [`Limits`], then run [`Wrapper::run`].
//! [`ClientHandle`] makes outgoing calls; [`Context`] carries cancellation and coordinates.
//! Running a wrapper performs I/O; importing this module has no side effects.
pub mod model;
pub mod wrapper;
// AI-generated public server API.
pub use model::{
    ClientHandle, Context, ExitStatus, ILanguageServer, Limits, ServerError, ServerResult, Wrapper,
};
