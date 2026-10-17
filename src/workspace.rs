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
