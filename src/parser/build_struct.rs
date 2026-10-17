//! Extract ordered, file-local build facts without executing CMake or reading dependencies.
//! [`model::BuildStructAnalyzer`] consumes semantic commands; [`crate::workspace::Workspace`]
//! interprets dependency visits and owns cross-file reference indexes.
// Contains AI-generated documentation additions.
// AI-generated implementation.
mod analyze;
pub(crate) mod dependencies;
mod display;
mod incremental;
pub mod model;
mod project;
mod sources;
pub(crate) mod target_references;
mod targets;
