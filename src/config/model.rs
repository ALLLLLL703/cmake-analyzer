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
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub max_open_documents: usize,
    pub max_workspaces: usize,
}
static DEFAULT_LIMITS: Limits = Limits {
    max_open_documents: 256,
    max_workspaces: 32,
};
impl Default for Limits {
    fn default() -> Self {
        DEFAULT_LIMITS
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
pub struct Format {
    pub on_save: bool,
    pub insert_spaces: bool,
    pub indent_width: usize,
    pub line_width: usize,
}
impl Default for Format {
    fn default() -> Self {
        let options = crate::format::Options::default();
        Self {
            on_save: true,
            insert_spaces: options.insert_spaces,
            indent_width: options.indent_width,
            line_width: options.line_width,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Completion {
    pub reserve_arg: bool,
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
            search_paths: super::defaults::PROJECT_SEARCH_PATHS
                .iter()
                .map(PathBuf::from)
                .collect(),
            builtin_directory: None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuntimeMode {
    Action,
    OnSave,
}
#[derive(Clone, Debug)]
pub struct Runtime {
    pub mode: RuntimeMode,
    pub enable: bool,
    pub executable: PathBuf,
    pub build_directory: Option<PathBuf>,
    pub arguments: Vec<String>,
    pub timeout_seconds: u64,
    pub max_output_bytes: usize,
    pub max_view_bytes: usize,
}
static DEFAULT_RUNTIME_TIMEOUT_SECONDS: u64 = 120;
static DEFAULT_RUNTIME_OUTPUT_BYTES: usize = 64 * 1024 * 1024;
static DEFAULT_RUNTIME_VIEW_BYTES: usize = 64 * 1024;
impl Default for Runtime {
    fn default() -> Self {
        Self {
            enable: true,
            mode: RuntimeMode::Action,
            executable: PathBuf::from("cmake"),
            build_directory: None,
            arguments: Vec::new(),
            timeout_seconds: DEFAULT_RUNTIME_TIMEOUT_SECONDS,
            max_output_bytes: DEFAULT_RUNTIME_OUTPUT_BYTES,
            max_view_bytes: DEFAULT_RUNTIME_VIEW_BYTES,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Settings {
    pub format: Format,
    pub completion: Completion,
    pub diagnostics: Diagnostics,
    pub document_symbols: DocumentSymbols,
    pub modules: Modules,
    pub runtime: Runtime,
    pub interpreter_commands: BTreeMap<PathBuf, InterpreterServices>,
}
#[derive(Default)]
pub(super) struct Layer {
    pub patch: SettingsPatch,
    pub commands: BTreeMap<PathBuf, InterpreterServices>,
}
pub struct ConfigStore {
    pub sources: ConfigSources,
    pub limits: Limits,
    pub(super) user: Layer,
    pub(super) explicit: Layer,
    pub(super) projects: BTreeMap<PathBuf, Arc<Settings>>,
    pub(super) builtin_directory: tokio::sync::OnceCell<Option<PathBuf>>,
}
