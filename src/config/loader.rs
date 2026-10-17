//! Read TOML asynchronously, parse on blocking workers and merge defaults/user/project/explicit
//! settings. No writes or watchers. Cached directory settings and cached builtin discovery
//! are replaced by constructing a fresh store during the server's transactional reload.
//!
//! Contains AI-generated documentation additions.
// AI-authored asynchronous file loading and field-wise configuration layering.
// review over
use super::{model::*, patch_model::SettingsPatch};
use crate::parser::interpreter_commands::{parse, resolve_path};
use std::{collections::BTreeMap, io, path::Path, sync::Arc};

impl ConfigStore {
    /// Read global configuration and construct a fresh settings cache. Makes source paths absolute,
    /// reads optional user/required explicit TOML, and parses on workers. No project settings,
    /// builtin probe, disk writes or project execution occur yet.
    ///
    /// # Arguments
    /// - `sources` `[in]`: consumed source choices; cwd and file paths are normalized in the returned store.
    ///
    /// # Returns
    /// `Ok(store)` contains loaded global layers, process-wide limits and empty caches; `Err` leaves no live store changed.
    ///
    /// # Errors
    /// Path normalization, file read/UTF-8, TOML validation, interpreter mapping or worker join failures. Missing optional user file is allowed.
    ///
    /// # Algorithm
    /// Normalize startup-relative sources, read/parse global layers, apply only their limits over defaults and allocate empty directory/discovery caches.
    ///
    /// # Flow
    /// ```text
    ///  [sources + cwd]
    ///         |
    ///  [absolute source paths]
    ///       /      \
    ///  [user read] [explicit read]
    ///       \      /
    ///      [parsed layers] ---> [global limits]
    ///             |
    ///       [fresh cache store]
    /// ```
    ///
    /// # Examples
    /// ```no_run
    /// # use cmake_analyzer::config::{ConfigStore, ConfigSources};
    /// # async fn example() -> std::io::Result<()> {
    /// // Requires readable source files if selected.
    /// let mut sources = ConfigSources::new(std::env::current_dir()?);
    /// sources.user_file = None;
    /// let store = ConfigStore::load(sources).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load(mut sources: ConfigSources) -> io::Result<Self> {
        sources.cwd = crate::workspace::absolute(&sources.cwd)?;
        sources.user_file = sources
            .user_file
            .as_deref()
            .map(|path| resolve_path(&sources.cwd, path))
            .transpose()?;
        sources.explicit_file = sources
            .explicit_file
            .as_deref()
            .map(|path| resolve_path(&sources.cwd, path))
            .transpose()?;
        let user = read_layer(sources.user_file.as_deref(), false, &sources.cwd).await?;
        let explicit = read_layer(sources.explicit_file.as_deref(), true, &sources.cwd).await?;
        let mut limits = Limits::default();
        for layer in [&user, &explicit] {
            if let Some(patch) = &layer.patch.limits {
                if let Some(value) = patch.max_open_documents {
                    limits.max_open_documents = value.get();
                }
                if let Some(value) = patch.max_workspaces {
                    limits.max_workspaces = value.get();
                }
            }
        }
        Ok(Self {
            sources,
            limits,
            user,
            explicit,
            projects: BTreeMap::new(),
            builtin_directory: tokio::sync::OnceCell::new(),
        })
    }
    /// Process-wide inspection budgets must not inherit project-local execution settings.
    /// Merge default/user/explicit runtime settings for native help inspection. Ignores project layers
    /// and performs no reads, process launches or cache changes.
    ///
    /// # Arguments
    /// - `self` `[in]`: loaded global layers, borrowed without mutation.
    ///
    /// # Returns
    /// Owned effective Runtime from global layers only; executable/enable/mode do not authorize help execution at this helper.
    ///
    /// # Examples
    /// ```text
    /// let budgets = store.global_runtime();
    /// let timeout = budgets.timeout_seconds;
    /// ```
    pub(crate) fn global_runtime(&self) -> Runtime {
        let mut settings = Settings::default();
        apply(&mut settings, &self.user);
        apply(&mut settings, &self.explicit);
        settings.runtime
    }
    /// Omitted fields inherit; arrays replace and interpreter mappings merge by normalized file path.
    /// Resolve and cache effective settings for one analysis-entry directory. Reads its optional TOML
    /// and may lazily query PATH CMake version for builtin modules. Does not configure or execute
    /// project scripts, write files or install a watcher.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: directory settings are inserted on success; builtin discovery success/failure is initialized at most once.
    /// - `directory` `[in]`: entry directory, resolved against stored startup cwd without requiring it to exist.
    ///
    /// # Returns
    /// `Ok(Arc<Settings>)` shares a cached or newly merged result; `Err` does not insert settings for this directory.
    ///
    /// # Errors
    /// Path/read/parse errors; project-local limits or on_save authority without global-source status.
    /// Builtin discovery failure is nonfatal and caches None.
    ///
    /// # Algorithm
    /// Return a cache hit first. Read local overrides, reject project-only authority, resolve default
    /// module paths against the entry, then apply user/local/explicit layers. Force manual mode unless
    /// global settings authorize saves. Discover a missing builtin directory only when lookup is enabled.
    ///
    /// # Flow
    /// ```text
    ///  [normalize directory] ---> <cached?> ---yes---> [shared settings]
    ///                               | no
    ///                         [read local TOML]
    ///                               |
    ///                      [validate global-only policy] --bad--> [error]
    ///                               |
    ///  [defaults] ---> [user / local / explicit merge]
    ///                               |
    ///                     [save-authority gate]
    ///                               |
    ///                     <builtin needed?>
    ///                        /         \
    ///                  [cached probe] [skip]
    ///                        \         /
    ///                         [insert + share]
    /// ```
    ///
    /// # Examples
    /// ```no_run
    /// # use cmake_analyzer::config::{ConfigStore, ConfigSources};
    /// # async fn example(mut store: ConfigStore) -> std::io::Result<()> {
    /// // Reads /work/project/.cmake-analyzer.toml if present; may query installed CMake.
    /// let settings = store.for_directory(std::path::Path::new("/work/project")).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn for_directory(&mut self, directory: &Path) -> io::Result<Arc<Settings>> {
        let directory = resolve_path(&self.sources.cwd, directory)?;
        if let Some(settings) = self.projects.get(&directory) {
            return Ok(settings.clone());
        }
        let project_file = directory.join(PROJECT_CONFIG);
        let local = read_layer(Some(&project_file), false, &self.sources.cwd).await?;
        let global_source = self.sources.explicit_file.as_ref() == Some(&project_file)
            || self.sources.user_file.as_ref() == Some(&project_file);
        if local.patch.limits.is_some() && !global_source {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "{}: limits are process-wide; use user configuration or an explicit override",
                    directory.join(PROJECT_CONFIG).display()
                ),
            ));
        }
        if !global_source
            && local.patch.runtime.mode == Some(super::patch_model::RuntimeModePatch::OnSave)
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "runtime.mode = on_save requires user configuration or an explicit override; project settings may only select action or disable execution",
            ));
        }
        let mut authority = Settings::default();
        apply(&mut authority, &self.user);
        apply(&mut authority, &self.explicit);
        let mut settings = Settings::default();
        for path in &mut settings.modules.search_paths {
            *path = resolve_path(&directory, path)?;
        }
        for layer in [&self.user, &local, &self.explicit] {
            apply(&mut settings, layer);
        }
        if !authority.runtime.enable || authority.runtime.mode != super::model::RuntimeMode::OnSave
        {
            settings.runtime.mode = super::model::RuntimeMode::Action;
        }
        if settings.modules.enable && settings.modules.builtin_directory.is_none() {
            settings.modules.builtin_directory = self
                .builtin_directory
                .get_or_init(|| super::defaults::builtin_directory(&self.sources.cwd))
                .await
                .clone();
        }
        let settings = Arc::new(settings);
        self.projects.insert(directory, settings.clone());
        Ok(settings)
    }
}
/// Read one UTF-8 TOML file asynchronously and join a blocking parse job. No writes or subprocesses.
///
/// # Arguments
/// - `path` `[in]`: optional source; absence yields an empty layer.
/// - `required` `[in]`: reject a missing file when true; false tolerates NotFound only.
/// - `cwd` `[in]`: startup base for interpreter mappings passed to the parser.
///
/// # Returns
/// `Ok(layer)` for parsed input or an absent optional file; `Err` for other reads/validation/join failures.
///
/// # Errors
/// I/O/UTF-8 errors retain the source path; parse/normalization errors and worker join errors propagate.
///
/// # Examples
/// ```text
/// let layer = read_layer(None, false, std::path::Path::new("/work")).await?;
/// ```
async fn read_layer(path: Option<&Path>, required: bool, cwd: &Path) -> io::Result<Layer> {
    let Some(path) = path else {
        return Ok(Layer::default());
    };
    let text = match tokio::fs::read_to_string(path).await {
        Ok(text) => text,
        Err(error) if !required && error.kind() == io::ErrorKind::NotFound => {
            return Ok(Layer::default());
        }
        Err(error) => {
            return Err(io::Error::new(
                error.kind(),
                format!("{}: {error}", path.display()),
            ));
        }
    };
    let path = path.to_owned();
    let cwd = cwd.to_owned();
    tokio::task::spawn_blocking(move || parse_layer(&text, &path, &cwd))
        .await
        .map_err(io::Error::other)?
}
/// Decode TOML, normalize configured paths and tokenize interpreter mappings in memory.
/// Does not check existence, read files or execute commands.
///
/// # Arguments
/// - `text` `[in]`: complete TOML source.
/// - `path` `[in]`: defining file; its parent anchors module/build/multi-component executable paths.
/// - `cwd` `[in]`: startup base for interpreter keys and command paths, independent of defining file.
///
/// # Returns
/// `Ok(layer)` contains validated partial settings and normalized mappings; `Err` rejects the entire source.
///
/// # Errors
/// Invalid TOML, unknown fields, invalid positive counts/width, unsupported runtime arguments, invalid interpreter commands,
/// path normalization failures or duplicate interpreter keys after normalization.
///
/// # Algorithm
/// Decode omission-preserving fields; anchor file-relative paths, validate arguments, then consume the raw mapping
/// into normalized keys and parsed service requirements. Reject key collisions before returning.
///
/// # Flow
/// ```text
///  [TOML] ---> [SettingsPatch]
///                    |
///           [normalize file-relative paths]
///                    |
///           [validate runtime arguments]
///                    |
///  [startup cwd] --> [normalize + parse mappings]
///                    |             |
///                 [Layer]   [collision/error]
/// ```
///
/// # Examples
/// ```text
/// let layer = parse_layer("[format]\nindent_width = 4", std::path::Path::new("/work/style.toml"), std::path::Path::new("/work"))?;
/// ```
fn parse_layer(text: &str, path: &Path, cwd: &Path) -> io::Result<Layer> {
    let invalid = |error: String| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{}: {error}", path.display()),
        )
    };
    let mut patch: SettingsPatch =
        toml::from_str(text).map_err(|error| invalid(error.to_string()))?;
    let base = path.parent().unwrap_or(Path::new("/"));
    if let Some(paths) = &mut patch.modules.package_search_paths {
        for value in paths {
            *value = resolve_path(base, value)?;
        }
    }
    if let Some(paths) = &mut patch.modules.search_paths {
        for value in paths {
            *value = resolve_path(base, value)?;
        }
    }
    if let Some(value) = &mut patch.modules.builtin_directory {
        *value = resolve_path(base, value)?;
    }
    if let Some(path) = &mut patch.runtime.build_directory {
        *path = resolve_path(base, path)?;
    }
    if let Some(path) = &mut patch.runtime.executable {
        if path.components().count() > 1 {
            *path = resolve_path(base, path)?;
        }
    }
    if let Some(arguments) = &patch.runtime.arguments {
        super::runtime::validate_arguments(arguments)
            .map_err(|error| invalid(error.to_string()))?;
    }
    let mut commands = BTreeMap::new();
    for (file, command) in std::mem::take(&mut patch.interpreter_commands) {
        let file = resolve_path(cwd, Path::new(&file))?;
        let services = parse(&command, cwd).map_err(|error| {
            invalid(format!("interpreter_commands[{}]: {error}", file.display()))
        })?;
        if commands.insert(file.clone(), services).is_some() {
            return Err(invalid(format!(
                "Duplicate interpreter file {}",
                file.display()
            )));
        }
    }
    Ok(Layer { patch, commands })
}
/// Apply only present fields to effective settings in memory. No I/O, process launches or authorization checks.
/// Arrays replace lower values; interpreter mappings merge by key and later keys overwrite.
///
/// # Arguments
/// - `settings` `[in,out]`: present scalar/vector fields are overwritten; unrelated fields remain; mappings extend/overwrite matching keys.
/// - `layer` `[in]`: parsed partial source and normalized mappings; not changed.
///
/// # Returns
/// `()`; all changes are made directly to settings. Process limits are handled separately at load.
///
/// # Algorithm
/// For each present scalar overwrite its effective field; replace supplied vectors
/// with clones. Convert enum patches to runtime enums. Extend interpreter mappings,
/// letting this layer overwrite matching keys. Do not apply process limits here.
///
/// # Flow
/// ```text
///  [effective settings]    [one partial layer]
///           \                    /
///            [each configurable field]
///                      |
///                  <present?>
///                  /       \
///                no        yes
///                 |         |
///             [retain] [overwrite/replace]
///                 \         /
///                  [merge command map]
///                          |
///                   [updated settings]
/// ```
///
/// # Examples
/// ```text
/// let mut settings = Settings::default();
/// apply(&mut settings, &Layer::default());
/// ```
fn apply(settings: &mut Settings, layer: &Layer) {
    if let Some(value) = layer.patch.format.on_save {
        settings.format.on_save = value;
    }
    if let Some(value) = layer.patch.format.insert_spaces {
        settings.format.insert_spaces = value;
    }
    if let Some(value) = layer.patch.format.indent_width {
        settings.format.indent_width = usize::from(value.get());
    }
    if let Some(value) = layer.patch.format.line_width {
        settings.format.line_width = value.get();
    }
    if let Some(value) = layer.patch.completion.reserve_arg {
        settings.completion.reserve_arg = value;
    }
    if let Some(level) = layer.patch.diagnostics.rules.unknown_function {
        settings.diagnostics.rules.unknown_function = level.into();
    }
    if let Some(level) = layer.patch.diagnostics.rules.unknown_variable {
        settings.diagnostics.rules.unknown_variable = level.into();
    }
    if let Some(mode) = layer.patch.modules.external_include {
        settings.modules.external_include = mode.into();
    }
    if let Some(mode) = layer.patch.modules.external_find_package {
        settings.modules.external_find_package = mode.into();
    }
    if let Some(paths) = &layer.patch.modules.package_search_paths {
        settings.modules.package_search_paths = paths.clone();
    }
    if let Some(enable) = layer.patch.diagnostics.enable {
        settings.diagnostics.enable = enable;
    }
    if let Some(enable) = layer.patch.document_symbols.enable {
        settings.document_symbols.enable = enable;
    }
    if let Some(enable) = layer.patch.modules.enable {
        settings.modules.enable = enable;
    }
    if let Some(paths) = &layer.patch.modules.search_paths {
        settings.modules.search_paths = paths.clone();
    }
    if let Some(path) = &layer.patch.modules.builtin_directory {
        settings.modules.builtin_directory = Some(path.clone());
    }
    if let Some(mode) = layer.patch.runtime.mode {
        settings.runtime.mode = match mode {
            super::patch_model::RuntimeModePatch::Action => super::model::RuntimeMode::Action,
            super::patch_model::RuntimeModePatch::OnSave => super::model::RuntimeMode::OnSave,
        };
    }
    if let Some(value) = layer.patch.runtime.enable {
        settings.runtime.enable = value;
    }
    if let Some(value) = &layer.patch.runtime.executable {
        settings.runtime.executable = value.clone();
    }
    if let Some(value) = &layer.patch.runtime.build_directory {
        settings.runtime.build_directory = Some(value.clone());
    }
    if let Some(value) = &layer.patch.runtime.arguments {
        settings.runtime.arguments = value.clone();
    }
    if let Some(value) = layer.patch.runtime.timeout_seconds {
        settings.runtime.timeout_seconds = value.get();
    }
    if let Some(value) = layer.patch.runtime.max_output_bytes {
        settings.runtime.max_output_bytes = value.get();
    }
    if let Some(value) = layer.patch.runtime.max_view_bytes {
        settings.runtime.max_view_bytes = value.get();
    }
    settings.interpreter_commands.extend(layer.commands.clone());
}
