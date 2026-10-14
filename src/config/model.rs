// AI-authored layered configuration models.
use super::patch_model::SettingsPatch;
use crate::parser::interpreter_commands::InterpreterServices;
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

pub const USER_CONFIG: &str = "~/.config/cmake-analyzer/cmake-analyzer.toml";
pub const PROJECT_CONFIG: &str = ".cmake-analyzer.toml";

#[derive(Clone, Debug)]
pub struct ConfigSources {
    pub cwd: PathBuf,
    pub user_file: Option<PathBuf>,
    pub explicit_file: Option<PathBuf>,
}
impl ConfigSources {
    pub fn new(cwd: PathBuf) -> Self {
        Self {
            cwd,
            user_file: std::env::var_os("HOME")
                .map(|home| PathBuf::from(home).join(".config/cmake-analyzer/cmake-analyzer.toml")),
            explicit_file: None,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Diagnostics {
    pub enable: bool,
}
impl Default for Diagnostics {
    fn default() -> Self {
        Self { enable: true }
    }
}
#[derive(Clone, Debug)]
pub struct DocumentSymbols {
    pub enable: bool,
}
impl Default for DocumentSymbols {
    fn default() -> Self {
        Self { enable: true }
    }
}
#[derive(Clone, Debug)]
pub struct Modules {
    pub enable: bool,
    pub search_paths: Vec<PathBuf>,
    pub builtin_directory: Option<PathBuf>,
}
impl Default for Modules {
    fn default() -> Self {
        Self {
            enable: true,
            search_paths: Vec::new(),
            builtin_directory: None,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Settings {
    pub diagnostics: Diagnostics,
    pub document_symbols: DocumentSymbols,
    pub modules: Modules,
    pub interpreter_commands: BTreeMap<PathBuf, InterpreterServices>,
}
#[derive(Default)]
pub(super) struct Layer {
    pub patch: SettingsPatch,
    pub commands: BTreeMap<PathBuf, InterpreterServices>,
}
pub struct ConfigStore {
    pub sources: ConfigSources,
    pub(super) user: Layer,
    pub(super) explicit: Layer,
    pub(super) projects: BTreeMap<PathBuf, Arc<Settings>>,
}
