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
        let mut settings = Settings::default();
        for path in &mut settings.modules.search_paths {
            *path = resolve_path(&directory, path)?;
        }
        for layer in [&self.user, &local, &self.explicit] {
            apply(&mut settings, layer);
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
    if let Some(paths) = &mut patch.modules.search_paths {
        for value in paths {
            *value = resolve_path(base, value)?;
        }
    }
    if let Some(value) = &mut patch.modules.builtin_directory {
        *value = resolve_path(base, value)?;
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
    settings.interpreter_commands.extend(layer.commands.clone());
}
