// AI-authored partial configuration models.
use serde::Deserialize;
use std::{collections::BTreeMap, num::NonZeroUsize, path::PathBuf};

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
pub(super) struct LimitsPatch {
    pub max_open_documents: Option<NonZeroUsize>,
    pub max_workspaces: Option<NonZeroUsize>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct RuntimePatch {
    pub mode: Option<RuntimeModePatch>,
    pub enable: Option<bool>,
    pub executable: Option<PathBuf>,
    pub build_directory: Option<PathBuf>,
    pub arguments: Option<Vec<String>>,
    pub timeout_seconds: Option<std::num::NonZeroU64>,
    pub max_output_bytes: Option<NonZeroUsize>,
    pub max_view_bytes: Option<NonZeroUsize>,
}
#[derive(Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub(super) enum RuntimeModePatch {
    Action,
    OnSave,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct SettingsPatch {
    pub limits: Option<LimitsPatch>,
    pub diagnostics: DiagnosticsPatch,
    pub document_symbols: DocumentSymbolsPatch,
    pub modules: ModulesPatch,
    pub runtime: RuntimePatch,
    pub interpreter_commands: BTreeMap<String, String>,
}
