// AI-generated implementation.
//! Keeps file results and project links. See src/workspace/README.md for the flow.
mod display;
mod documents;
mod graph;
pub mod model;
mod parser;
pub use model::{DependencyGraphDisplay, Workspace, WorkspaceDisplay, WorkspaceLimits};
