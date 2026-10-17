//! Static CMake command, scope, reference and limited value analysis.
//!
//! [`model`] exposes snapshot-local records; [`parsing`] fills them without evaluating
//! conditions or calling project functions. Version discovery may run `cmake --version`,
//! not configure or script mode. Workspace callers replay resolved dependency visits
//! through the internal reference collector; replay means ordered analysis, not execution.
// Contains AI-generated documentation additions.
pub mod model;
mod reference_model;
pub(crate) mod references;
mod value_model;
pub(crate) mod values;
// AI-authored incremental implementation.
mod documentation;
mod incremental;
pub mod parsing;
mod path_model;
pub mod schema;
pub mod variables;
// AI-generated display wiring.
mod display;
