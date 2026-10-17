// AI-authored asynchronous file loading and field-wise configuration layering.
// review over
use super::{model::*, patch_model::SettingsPatch};
use crate::parser::interpreter_commands::{parse, resolve_path};
use std::{collections::BTreeMap, io, path::Path, sync::Arc};

impl ConfigStore {
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
    pub(crate) fn global_runtime(&self) -> Runtime {
        let mut settings = Settings::default();
        apply(&mut settings, &self.user);
        apply(&mut settings, &self.explicit);
        settings.runtime
    }
    /// Omitted fields inherit; arrays replace and interpreter mappings merge by normalized file path.
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
