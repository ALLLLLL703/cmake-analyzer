// AI-authored partial configuration models.
use serde::Deserialize;
use std::{collections::BTreeMap, num::NonZeroUsize, path::PathBuf};

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct DiagnosticsPatch {
    pub enable: Option<bool>,
    pub rules: DiagnosticRulesPatch,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct DiagnosticRulesPatch {
    pub unknown_function: Option<DiagnosticLevelPatch>,
    pub unknown_variable: Option<DiagnosticLevelPatch>,
}
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum DiagnosticLevelPatch {
    Error,
    Warn,
    Info,
    Hint,
    None,
}
impl From<DiagnosticLevelPatch> for super::DiagnosticLevel {
    fn from(value: DiagnosticLevelPatch) -> Self {
        match value {
            DiagnosticLevelPatch::Error => Self::Error,
            DiagnosticLevelPatch::Warn => Self::Warn,
            DiagnosticLevelPatch::Info => Self::Info,
            DiagnosticLevelPatch::Hint => Self::Hint,
            DiagnosticLevelPatch::None => Self::None,
        }
    }
}
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum ExternalIndexPatch {
    Full,
    Functions,
    None,
}
impl From<ExternalIndexPatch> for crate::workspace::model::ExternalIndex {
    fn from(value: ExternalIndexPatch) -> Self {
        match value {
            ExternalIndexPatch::Full => Self::Full,
            ExternalIndexPatch::Functions => Self::Functions,
            ExternalIndexPatch::None => Self::None,
        }
    }
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct DocumentSymbolsPatch {
    pub enable: Option<bool>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct FormatPatch {
    pub on_save: Option<bool>,
    pub insert_spaces: Option<bool>,
    pub indent_width: Option<std::num::NonZeroU8>,
    pub line_width: Option<NonZeroUsize>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct CompletionPatch {
    pub reserve_arg: Option<bool>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct ModulesPatch {
    pub external_include: Option<ExternalIndexPatch>,
    pub external_find_package: Option<ExternalIndexPatch>,
    pub package_search_paths: Option<Vec<PathBuf>>,
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
    pub format: FormatPatch,
    pub completion: CompletionPatch,
    pub limits: Option<LimitsPatch>,
    pub diagnostics: DiagnosticsPatch,
    pub document_symbols: DocumentSymbolsPatch,
    pub modules: ModulesPatch,
    pub runtime: RuntimePatch,
    pub interpreter_commands: BTreeMap<String, String>,
}
