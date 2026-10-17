//! Own one project's immutable file results, ordered dependency visits and contextual indexes.
//! [`Workspace`] queues edits until [`Workspace::flush`]; static analysis does not configure
//! or execute the project. Explicit runtime traces are installed separately through [`runtime`].
// Contains AI-generated documentation additions.
// AI-generated implementation.
// See src/workspace/README.md for the flow.
pub(crate) mod completion;
mod display;
mod documents;
pub(crate) use documents::absolute;
mod graph;
mod incremental;
pub mod model;
mod packages;
mod parser;
mod reference_values;
mod references;
mod refresh;
pub mod runtime;
mod targets;
pub use model::{DependencyGraphDisplay, Workspace, WorkspaceDisplay, WorkspaceLimits};
