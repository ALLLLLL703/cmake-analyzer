// AI-authored configuration entrypoint.
mod defaults;
mod loader;
mod model;
mod patch_model;
mod runtime;
pub use model::{
    ConfigSources, ConfigStore, Diagnostics, DocumentSymbols, Limits, Modules, PROJECT_CONFIG,
    Runtime, Settings, USER_CONFIG,
};
pub(crate) use runtime::validate_script_arguments;
