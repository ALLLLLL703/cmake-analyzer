//! Effective settings and source/cache ownership for [`ConfigStore`]. Constructors
//! only allocate data/read environment; asynchronous methods in the loader own file reads.
//! Execution settings describe permission and budgets, not a running process.
//!
//! Contains AI-generated documentation additions.
// AI-authored layered configuration models.
use super::patch_model::SettingsPatch;
use crate::parser::interpreter_commands::InterpreterServices;
use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

/// Human-readable default user configuration location, not a tilde-expanding loader input. Example: `assert!(USER_CONFIG.starts_with("~/"))`.
pub const USER_CONFIG: &str = "~/.config/cmake-analyzer/cmake-analyzer.toml";
/// Entry-directory configuration basename. Example: `let path = directory.join(PROJECT_CONFIG)`.
pub const PROJECT_CONFIG: &str = ".cmake-analyzer.toml";

#[derive(Clone, Debug)]
/// Files used by [`ConfigStore::load`]. User and explicit paths resolve against `cwd`;
/// project files are discovered later by [`ConfigStore::for_directory`]. Construction starts no I/O.
///
/// # Examples
/// ```
/// # use cmake_analyzer::config::{ConfigSources};
/// let sources = ConfigSources::new("/work".into());
/// ```
pub struct ConfigSources {
    /// Startup directory used for relative source paths and interpreter mappings. Example: `sources.cwd.display()`.
    pub cwd: PathBuf,
    /// Optional user TOML; a missing file is allowed. Example: `sources.user_file = None`.
    pub user_file: Option<PathBuf>,
    /// Highest-priority TOML; when specified it must be readable. Example: `sources.explicit_file = Some("override.toml".into())`.
    pub explicit_file: Option<PathBuf>,
}
impl ConfigSources {
    /// Choose default source paths from startup cwd and HOME. Reads HOME only; no file existence
    /// checks, path normalization, configuration reads or process launches.
    ///
    /// # Arguments
    /// - `cwd` `[in]`: startup directory retained as supplied; load later makes it absolute.
    ///
    /// # Returns
    /// A source descriptor with an optional HOME-based user file and no explicit override.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::config::ConfigSources;
    /// let sources = ConfigSources::new("/work".into());
    /// assert!(sources.explicit_file.is_none());
    /// ```
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
/// Process-wide admission counts used by the server. [`ConfigStore::load`] merges only
/// user/explicit values; construction has no resource allocation or enforcement effects.
///
/// # Examples
/// ```
/// # use cmake_analyzer::config::{Limits};
/// let limits = Limits::default();
/// assert_eq!(limits.max_workspaces, 32);
/// ```
pub struct Limits {
    /// Maximum admitted open documents across the process; default 256. Example: `limits.max_open_documents = 128`.
    pub max_open_documents: usize,
    /// Maximum Workspaces and shared runtime-job budget; default 32. Example: `limits.max_workspaces = 16`.
    pub max_workspaces: usize,
}
static DEFAULT_LIMITS: Limits = Limits {
    max_open_documents: 256,
    max_workspaces: 32,
};
impl Default for Limits {
    /// Return the named process count defaults (256 documents, 32 Workspaces). No I/O or process effects.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// A fresh `Limits` value; no configuration files are consulted.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::config::Limits;
    /// let value = Limits::default();
    /// ```
    fn default() -> Self {
        DEFAULT_LIMITS
    }
}
pub use super::diagnostics::Diagnostics;
#[derive(Clone, Debug)]
/// Document/workspace symbol participation settings consumed by the server.
/// Stored in [`Settings`]; changing this value alone publishes nothing.
///
/// # Examples
/// ```
/// # use cmake_analyzer::config::{DocumentSymbols};
/// let symbols = DocumentSymbols::default();
/// assert!(symbols.enable);
/// ```
pub struct DocumentSymbols {
    /// Whether this owner contributes symbol results; defaults true. Example: `symbols.enable = false`.
    pub enable: bool,
}
impl Default for DocumentSymbols {
    /// Enable symbol participation by default. No I/O or process effects.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// A fresh `DocumentSymbols` value; no configuration files are consulted.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::config::DocumentSymbols;
    /// let value = DocumentSymbols::default();
    /// ```
    fn default() -> Self {
        Self { enable: true }
    }
}
#[derive(Clone, Debug)]
/// Configured save/CLI formatting style. [`crate::format::Options`] is the pure renderer
/// input; [`Settings`] holds this configuration. Construction does not format or write files.
///
/// # Examples
/// ```
/// # use cmake_analyzer::config::{Format};
/// let style = Format::default();
/// assert_eq!(style.indent_width, 2);
/// ```
pub struct Format {
    /// Enable pre-save edits; default true. Does not control CLI or explicit formatting. Example: `style.on_save = false`.
    pub on_save: bool,
    /// Use spaces rather than one tab per indentation level; default true. Example: `style.insert_spaces = false`.
    pub insert_spaces: bool,
    /// Indent/tab-stop width in columns; loaded values are 1..=255, default 2. Example: `style.indent_width = 4`.
    pub indent_width: usize,
    /// Positive Unicode-scalar column target, default 100; literals are never split. Example: `style.line_width = 80`.
    pub line_width: usize,
}
impl Default for Format {
    /// Use renderer defaults (two spaces, 100 columns) with pre-save formatting enabled. No I/O or process effects.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// A fresh `Format` value; no configuration files are consulted.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::config::Format;
    /// let value = Format::default();
    /// ```
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
/// Command-call snippet preference used by the server. [`Settings`] stores it;
/// construction does not inspect clients or produce edits.
///
/// # Examples
/// ```
/// # use cmake_analyzer::config::{Completion};
/// let completion = Completion::default();
/// assert!(!completion.reserve_arg);
/// ```
pub struct Completion {
    /// Offer required argument placeholders only when client/shared-owner policy also permits; default false. Example: `completion.reserve_arg = true`.
    pub reserve_arg: bool,
}
#[derive(Clone, Debug)]
/// Static module/package discovery policy for Workspace assembly. [`Settings`] stores
/// these values; constructing defaults does not scan directories or query CMake.
///
/// # Examples
/// ```
/// # use cmake_analyzer::config::{Modules};
/// let modules = Modules::default();
/// assert!(modules.enable);
/// ```
pub struct Modules {
    /// Indexing depth for includes outside the entry directory; defaults Full. Example: `modules.external_include = cmake_analyzer::workspace::model::ExternalIndex::None`.
    pub external_include: crate::workspace::model::ExternalIndex,
    /// Independent external package indexing policy; defaults Full. Example: `modules.external_find_package = cmake_analyzer::workspace::model::ExternalIndex::Functions`.
    pub external_find_package: crate::workspace::model::ExternalIndex,
    /// Package prefixes/config directories; Unix defaults /usr/local and /usr, empty elsewhere. Example: `modules.package_search_paths.clear()`.
    pub package_search_paths: Vec<PathBuf>,
    /// Master module/package lookup switch; literal paths/subdirectories remain analyzable. Example: `modules.enable = false`.
    pub enable: bool,
    /// Include module roots; defaults resolve entry-relative, configured paths defining-file-relative. Example: `modules.search_paths.push("cmake".into())`.
    pub search_paths: Vec<PathBuf>,
    /// Explicit or discovered CMake Modules directory; None permits lazy discovery when enabled. Example: `modules.builtin_directory = Some("/opt/cmake/Modules".into())`.
    pub builtin_directory: Option<PathBuf>,
}
impl Default for Modules {
    /// Enable full external indexing and conventional module/package roots without inspecting disk. No I/O or process effects.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// A fresh `Modules` value; no configuration files are consulted.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::config::Modules;
    /// let value = Modules::default();
    /// ```
    fn default() -> Self {
        Self {
            enable: true,
            external_include: crate::workspace::model::ExternalIndex::Full,
            external_find_package: crate::workspace::model::ExternalIndex::Full,
            package_search_paths: if cfg!(unix) {
                vec!["/usr/local".into(), "/usr".into()]
            } else {
                Vec::new()
            },
            search_paths: super::defaults::PROJECT_SEARCH_PATHS
                .iter()
                .map(PathBuf::from)
                .collect(),
            builtin_directory: None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// When runtime execution may be requested. [`Runtime`] carries this policy;
/// [`ConfigStore::for_directory`] enforces global authorization. Choosing a mode has no side effects.
///
/// # Examples
/// ```
/// # use cmake_analyzer::config::RuntimeMode;
/// let mode = RuntimeMode::Action;
/// ```
pub enum RuntimeMode {
    /// Manual actions only; default. Example: `let mode = RuntimeMode::Action`.
    Action,
    /// Allow saved related inputs to schedule execution only with user/explicit authorization. Example: `let mode = RuntimeMode::OnSave`.
    OnSave,
}
#[derive(Clone, Debug)]
/// Execution permission, command options and inspection budgets. [`ConfigStore`] layers
/// these settings; [`crate::runtime_cmake::model::ConfigureOptions`] describes one actual run.
/// Constructing these settings never starts a process or creates a build directory.
///
/// # Examples
/// ```
/// # use cmake_analyzer::config::{Runtime, RuntimeMode};
/// let runtime = Runtime::default();
/// assert_eq!(runtime.mode, RuntimeMode::Action);
/// ```
pub struct Runtime {
    /// Manual action by default; OnSave needs global authorization. Example: `runtime.mode = RuntimeMode::Action`.
    pub mode: RuntimeMode,
    /// Allow runtime actions; true by default, not automatic execution authority. Example: `runtime.enable = false`.
    pub enable: bool,
    /// Bare executable name uses PATH; configured multi-component paths use defining-file directory. Example: `runtime.executable = "/opt/cmake/bin/cmake".into()`.
    pub executable: PathBuf,
    /// Optional configure output directory; omission selects project/.cmake-analyzer/build. Ignored for scripts. Example: `runtime.build_directory = Some("build-analysis".into())`.
    pub build_directory: Option<PathBuf>,
    /// Extra allow-listed arguments passed directly, without shell interpretation; defaults empty. Example: `runtime.arguments.push("-DFLAG=ON".into())`.
    pub arguments: Vec<String>,
    /// Positive per-process timeout in seconds; default 120. Example: `runtime.timeout_seconds = 30`.
    pub timeout_seconds: u64,
    /// Positive per-run captured stderr/trace limit; default 64 MiB. Example: `runtime.max_output_bytes = 1024 * 1024`.
    pub max_output_bytes: usize,
    /// Positive view/hover/failure-detail byte budget; default 64 KiB. Example: `runtime.max_view_bytes = 4096`.
    pub max_view_bytes: usize,
}
static DEFAULT_RUNTIME_TIMEOUT_SECONDS: u64 = 120;
static DEFAULT_RUNTIME_OUTPUT_BYTES: usize = 64 * 1024 * 1024;
static DEFAULT_RUNTIME_VIEW_BYTES: usize = 64 * 1024;
impl Default for Runtime {
    /// Select enabled manual actions with PATH CMake and named time/output/view budgets. No I/O or process effects.
    ///
    /// # Arguments
    /// None.
    ///
    /// # Returns
    /// A fresh `Runtime` value; no configuration files are consulted.
    ///
    /// # Examples
    /// ```
    /// # use cmake_analyzer::config::Runtime;
    /// let value = Runtime::default();
    /// ```
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
/// Effective per-entry configuration shared through [`ConfigStore::for_directory`].
/// Fields describe policy; constructing a value performs no file or process effects.
///
/// # Examples
/// ```
/// # use cmake_analyzer::config::{Settings};
/// let settings = Settings::default();
/// assert!(settings.diagnostics.enable);
/// ```
pub struct Settings {
    /// Pre-save/CLI style and pre-save enable switch. Example: `settings.format.on_save = false`.
    pub format: Format,
    /// Command-call snippet policy. Example: `settings.completion.reserve_arg = true`.
    pub completion: Completion,
    /// Master report switch and per-rule severities. Example: `settings.diagnostics.enable = false`.
    pub diagnostics: Diagnostics,
    /// Symbol-query participation switch. Example: `settings.document_symbols.enable = false`.
    pub document_symbols: DocumentSymbols,
    /// Static dependency lookup and external indexing policy. Example: `settings.modules.search_paths.clear()`.
    pub modules: Modules,
    /// Execution options; not evidence that a run is active. Example: `settings.runtime.enable = false`.
    pub runtime: Runtime,
    /// Normalized file keys mapped to parsed project/script service requirements; layers merge by key. Example: `settings.interpreter_commands.clear()`.
    pub interpreter_commands: BTreeMap<PathBuf, InterpreterServices>,
}
#[derive(Default)]
/// One parsed source preserving omissions and normalized interpreter commands.
/// The loader applies it to [`Settings`]; creating an empty layer performs no I/O.
///
/// # Examples
/// ```text
/// let layer = Layer::default();
/// assert!(layer.commands.is_empty());
/// ```
pub(super) struct Layer {
    /// Partial settings from one TOML source. Example: `let patch = &layer.patch`.
    pub patch: SettingsPatch,
    /// Parsed interpreter mappings with startup-relative keys normalized to absolute paths. Example: `layer.commands.clear()`.
    pub commands: BTreeMap<PathBuf, InterpreterServices>,
}
/// Loaded global sources, admission limits and lazily cached per-entry settings.
/// CLI/server startup uses [`Self::load`]; Workspace assembly uses [`Self::for_directory`].
/// The store owns cached success/failure for builtin discovery; a fresh store retries it.
/// Construction by load reads global files; directory resolution may read local files and
/// query CMake version. No watcher, disk writes, configure or script execution.
///
/// # Flow
/// ```text
///  [user file]    [explicit file]
///       \            /
///        [load global layers] ---> [process limits]
///                 |
///          [ConfigStore owns]
///           /           \
///  [entry cache]   [builtin discovery cell]
///        ^                ^
///        |                |
///  [project TOML]    [PATH/version probe]
/// ```
///
/// # Examples
/// ```no_run
/// # use cmake_analyzer::config::{ConfigStore, ConfigSources};
/// # async fn example() -> std::io::Result<()> {
/// // Requires readable configuration sources; may query installed CMake version.
/// let mut store = ConfigStore::load(ConfigSources::new(std::env::current_dir()?)).await?;
/// let settings = store.for_directory(std::path::Path::new("/work/project")).await?;
/// # Ok(())
/// # }
/// ```
pub struct ConfigStore {
    /// Normalized source descriptor retained for reload/entry resolution. Example: `store.sources.cwd.display()`.
    pub sources: ConfigSources,
    /// User/explicit-only process limits computed at load time. Example: `store.limits.max_workspaces`.
    pub limits: Limits,
    /// Parsed optional user layer. Example: `let user = &store.user`.
    pub(super) user: Layer,
    /// Parsed required override layer, or empty when omitted. Example: `let explicit = &store.explicit`.
    pub(super) explicit: Layer,
    /// Effective settings cached by normalized entry directory until store replacement. Example: `store.projects.len()`.
    pub(super) projects: BTreeMap<PathBuf, Arc<Settings>>,
    /// One lazy result, including None for failed discovery; never retries inside this store. Example: `store.builtin_directory.get()`.
    pub(super) builtin_directory: tokio::sync::OnceCell<Option<PathBuf>>,
}
