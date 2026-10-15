// AI-generated implementation.
// See src/workspace/README.md for the flow.
mod display;
mod documents;
pub(crate) use documents::absolute;
mod graph;
mod incremental;
pub mod model;
mod parser;
mod references;
mod refresh;
pub use model::{DependencyGraphDisplay, Workspace, WorkspaceDisplay, WorkspaceLimits};
