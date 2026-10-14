// AI-authored configuration entrypoint.
mod loader;
mod model;
mod patch_model;
pub use model::{
    ConfigSources, ConfigStore, Diagnostics, DocumentSymbols, Modules, PROJECT_CONFIG, Settings,
    USER_CONFIG,
};
