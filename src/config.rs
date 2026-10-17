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
