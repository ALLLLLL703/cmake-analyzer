// AI-authored configuration entrypoint.
mod defaults;
mod loader;
mod model;
mod patch_model;
pub use model::{
    ConfigSources, ConfigStore, Diagnostics, DocumentSymbols, Limits, Modules, PROJECT_CONFIG,
    Settings, USER_CONFIG,
};
