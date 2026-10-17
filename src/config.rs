//! Layered analyzer configuration. [`ConfigStore::load`] reads user/explicit sources;
//! [`ConfigStore::for_directory`] merges project overrides and caches effective [`Settings`].
//! Loading never configures a project or runs a script; optional builtin discovery queries only `cmake --version`.
//!
//! Contains AI-generated documentation additions.
// AI-authored configuration entrypoint.
mod defaults;
mod diagnostics;
pub use diagnostics::{DiagnosticLevel, DiagnosticRules};
mod loader;
mod model;
mod patch_model;
mod runtime;
pub(crate) use defaults::cmake_executable;
pub use model::{
    Completion, ConfigSources, ConfigStore, Diagnostics, DocumentSymbols, Format, Limits, Modules,
    PROJECT_CONFIG, Runtime, RuntimeMode, Settings, USER_CONFIG,
};
pub(crate) use runtime::validate_script_arguments;
