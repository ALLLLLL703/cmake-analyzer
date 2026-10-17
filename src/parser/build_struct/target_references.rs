//! Bind literal target definitions and schema-identified uses without executing CMake.
//! Local extraction and workspace aggregation share one directory-aware collector.
// Contains AI-generated documentation additions.
mod collect;
mod model;
mod resolve;
pub(super) use collect::bind_local;
pub(crate) use model::{TargetCollector, TargetDirectoryId};
