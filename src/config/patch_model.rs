// AI-authored partial configuration models.
use serde::Deserialize;
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct DiagnosticsPatch {
    pub enable: Option<bool>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct DocumentSymbolsPatch {
    pub enable: Option<bool>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct ModulesPatch {
    pub enable: Option<bool>,
    pub search_paths: Option<Vec<PathBuf>>,
    pub builtin_directory: Option<PathBuf>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct SettingsPatch {
    pub diagnostics: DiagnosticsPatch,
    pub document_symbols: DocumentSymbolsPatch,
    pub modules: ModulesPatch,
    pub interpreter_commands: BTreeMap<String, String>,
}
