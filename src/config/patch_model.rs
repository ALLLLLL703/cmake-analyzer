//! TOML input records preserving omitted fields for field-wise layering.
//! Optional values inherit when absent, arrays replace when present, and unknown fields
//! are rejected by serde. Decoding is in-memory; the loader owns file reads/path resolution.
//!
//! Contains AI-generated documentation additions.
// AI-authored partial configuration models.
use serde::Deserialize;
use std::{collections::BTreeMap, num::NonZeroUsize, path::PathBuf};

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
/// Master report switch and per-rule omissions. Used by [`SettingsPatch`] and the loader to apply only supplied values.
/// Constructing/decoding this record has no filesystem or process effects.
///
/// # Examples
/// ```text
/// let patch = DiagnosticsPatch::default();
/// ```
pub(super) struct DiagnosticsPatch {
    /// Optional master switch; None inherits. Example: `DiagnosticsPatch { enable: Some(false), ..Default::default() }`.
    pub enable: Option<bool>,
    /// Rule overrides merged independently. Example: `DiagnosticsPatch { rules: DiagnosticRulesPatch::default(), ..Default::default() }`.
    pub rules: DiagnosticRulesPatch,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
/// Optional unresolved-name severities. Used by [`SettingsPatch`] and the loader to apply only supplied values.
/// Constructing/decoding this record has no filesystem or process effects.
///
/// # Examples
/// ```text
/// let patch = DiagnosticRulesPatch::default();
/// ```
pub(super) struct DiagnosticRulesPatch {
    /// Absent function rule inherits the previous level. Example: `DiagnosticRulesPatch { unknown_function: Some(DiagnosticLevelPatch::Warn), ..Default::default() }`.
    pub unknown_function: Option<DiagnosticLevelPatch>,
    /// Absent variable rule inherits the previous level. Example: `DiagnosticRulesPatch { unknown_variable: Some(DiagnosticLevelPatch::None), ..Default::default() }`.
    pub unknown_variable: Option<DiagnosticLevelPatch>,
}
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
/// Snake-case TOML severity decoded into [`super::DiagnosticLevel`]. Used by [`DiagnosticRulesPatch`]. Constructing it has no side effects.
///
/// # Examples
/// ```text
/// let value = DiagnosticLevelPatch::Error;
/// ```
pub(super) enum DiagnosticLevelPatch {
    /// Highest report severity. Example: `let value = DiagnosticLevelPatch::Error`.
    Error,
    /// Warning report severity. Example: `let value = DiagnosticLevelPatch::Warn`.
    Warn,
    /// Information report severity. Example: `let value = DiagnosticLevelPatch::Info`.
    Info,
    /// Hint report severity. Example: `let value = DiagnosticLevelPatch::Hint`.
    Hint,
    /// Disable this rule. Example: `let value = DiagnosticLevelPatch::None`.
    None,
}
impl From<DiagnosticLevelPatch> for super::DiagnosticLevel {
    /// Translate a decoded configuration choice to its effective model value. No I/O or mutation.
    ///
    /// # Arguments
    /// - `value` `[in]`: consumed TOML choice.
    ///
    /// # Returns
    /// The corresponding effective enum variant, with no fallible cases.
    ///
    /// # Examples
    /// ```text
    /// let effective = super::DiagnosticLevel::from(DiagnosticLevelPatch::None);
    /// ```
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
/// Snake-case external indexing choice converted to [`crate::workspace::model::ExternalIndex`]. Used by [`ModulesPatch`]. Constructing it has no side effects.
///
/// # Examples
/// ```text
/// let value = ExternalIndexPatch::Full;
/// ```
pub(super) enum ExternalIndexPatch {
    /// Normal conservative indexing. Example: `let value = ExternalIndexPatch::Full`.
    Full,
    /// Import unconditional function/macro declarations only. Example: `let value = ExternalIndexPatch::Functions`.
    Functions,
    /// Retain navigation edges without importing symbols. Example: `let value = ExternalIndexPatch::None`.
    None,
}
impl From<ExternalIndexPatch> for crate::workspace::model::ExternalIndex {
    /// Translate a decoded configuration choice to its effective model value. No I/O or mutation.
    ///
    /// # Arguments
    /// - `value` `[in]`: consumed TOML choice.
    ///
    /// # Returns
    /// The corresponding effective enum variant, with no fallible cases.
    ///
    /// # Examples
    /// ```text
    /// let effective = crate::workspace::model::ExternalIndex::from(ExternalIndexPatch::None);
    /// ```
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
/// Optional symbol participation override. Used by [`SettingsPatch`] and the loader to apply only supplied values.
/// Constructing/decoding this record has no filesystem or process effects.
///
/// # Examples
/// ```text
/// let patch = DocumentSymbolsPatch::default();
/// ```
pub(super) struct DocumentSymbolsPatch {
    /// None inherits symbol participation. Example: `DocumentSymbolsPatch { enable: Some(false), ..Default::default() }`.
    pub enable: Option<bool>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
/// Optional pre-save/CLI formatting overrides. Used by [`SettingsPatch`] and the loader to apply only supplied values.
/// Constructing/decoding this record has no filesystem or process effects.
///
/// # Examples
/// ```text
/// let patch = FormatPatch::default();
/// ```
pub(super) struct FormatPatch {
    /// None inherits save formatting policy. Example: `FormatPatch { on_save: Some(false), ..Default::default() }`.
    pub on_save: Option<bool>,
    /// None inherits tabs/spaces policy. Example: `FormatPatch { insert_spaces: Some(false), ..Default::default() }`.
    pub insert_spaces: Option<bool>,
    /// Optional 1..=255 column indent/tab width, enforced by nonzero u8. Example: `FormatPatch { indent_width: std::num::NonZeroU8::new(4), ..Default::default() }`.
    pub indent_width: Option<std::num::NonZeroU8>,
    /// Optional positive scalar-column target. Example: `FormatPatch { line_width: NonZeroUsize::new(80), ..Default::default() }`.
    pub line_width: Option<NonZeroUsize>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
/// Optional command-call snippet preference. Used by [`SettingsPatch`] and the loader to apply only supplied values.
/// Constructing/decoding this record has no filesystem or process effects.
///
/// # Examples
/// ```text
/// let patch = CompletionPatch::default();
/// ```
pub(super) struct CompletionPatch {
    /// None inherits, Some(false) explicitly disables. Example: `CompletionPatch { reserve_arg: Some(true), ..Default::default() }`.
    pub reserve_arg: Option<bool>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
/// Optional module lookup paths and external indexing policies. Used by [`SettingsPatch`] and the loader to apply only supplied values.
/// Constructing/decoding this record has no filesystem or process effects.
///
/// # Examples
/// ```text
/// let patch = ModulesPatch::default();
/// ```
pub(super) struct ModulesPatch {
    /// Optional policy for external include files. Example: `ModulesPatch { external_include: Some(ExternalIndexPatch::Functions), ..Default::default() }`.
    pub external_include: Option<ExternalIndexPatch>,
    /// Optional independent external package policy. Example: `ModulesPatch { external_find_package: Some(ExternalIndexPatch::None), ..Default::default() }`.
    pub external_find_package: Option<ExternalIndexPatch>,
    /// Present list replaces all lower prefixes; [] removes defaults. Example: `ModulesPatch { package_search_paths: Some(Vec::new()), ..Default::default() }`.
    pub package_search_paths: Option<Vec<PathBuf>>,
    /// Optional master module/package lookup switch. Example: `ModulesPatch { enable: Some(false), ..Default::default() }`.
    pub enable: Option<bool>,
    /// Present list replaces lower include roots; paths resolve against defining file. Example: `ModulesPatch { search_paths: Some(vec!["cmake".into()]), ..Default::default() }`.
    pub search_paths: Option<Vec<PathBuf>>,
    /// Optional override root; omission inherits or permits lazy discovery. Example: `ModulesPatch { builtin_directory: Some("/opt/cmake/Modules".into()), ..Default::default() }`.
    pub builtin_directory: Option<PathBuf>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
/// Optional positive process-wide counts, accepted only from global sources. Used by [`SettingsPatch`] and the loader to apply only supplied values.
/// Constructing/decoding this record has no filesystem or process effects.
///
/// # Examples
/// ```text
/// let patch = LimitsPatch::default();
/// ```
pub(super) struct LimitsPatch {
    /// Optional maximum open-document count across the process. Example: `LimitsPatch { max_open_documents: NonZeroUsize::new(128), ..Default::default() }`.
    pub max_open_documents: Option<NonZeroUsize>,
    /// Optional maximum Workspace/runtime-job count. Example: `LimitsPatch { max_workspaces: NonZeroUsize::new(16), ..Default::default() }`.
    pub max_workspaces: Option<NonZeroUsize>,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
/// Optional execution settings; loading does not start a run. Used by [`SettingsPatch`] and the loader to apply only supplied values.
/// Constructing/decoding this record has no filesystem or process effects.
///
/// # Examples
/// ```text
/// let patch = RuntimePatch::default();
/// ```
pub(super) struct RuntimePatch {
    /// Optional scheduling mode; project OnSave cannot grant global authority. Example: `RuntimePatch { mode: Some(RuntimeModePatch::Action), ..Default::default() }`.
    pub mode: Option<RuntimeModePatch>,
    /// Optional permission to execute project/script code. Example: `RuntimePatch { enable: Some(false), ..Default::default() }`.
    pub enable: Option<bool>,
    /// Optional executable name or defining-file-relative path. Example: `RuntimePatch { executable: Some("cmake".into()), ..Default::default() }`.
    pub executable: Option<PathBuf>,
    /// Optional defining-file-relative configure directory; scripts ignore it. Example: `RuntimePatch { build_directory: Some("analysis-build".into()), ..Default::default() }`.
    pub build_directory: Option<PathBuf>,
    /// Present argument vector replaces lower arguments and is allow-list validated. Example: `RuntimePatch { arguments: Some(vec!["-DFLAG=ON".into()]), ..Default::default() }`.
    pub arguments: Option<Vec<String>>,
    /// Optional positive timeout in seconds. Example: `RuntimePatch { timeout_seconds: std::num::NonZeroU64::new(30), ..Default::default() }`.
    pub timeout_seconds: Option<std::num::NonZeroU64>,
    /// Optional positive captured stderr/trace byte bound. Example: `RuntimePatch { max_output_bytes: NonZeroUsize::new(4096), ..Default::default() }`.
    pub max_output_bytes: Option<NonZeroUsize>,
    /// Optional positive view/hover/failure-detail byte bound. Example: `RuntimePatch { max_view_bytes: NonZeroUsize::new(1024), ..Default::default() }`.
    pub max_view_bytes: Option<NonZeroUsize>,
}
#[derive(Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
/// Snake-case scheduling choice used by [`RuntimePatch`]; the loader separately checks global save authority. Constructing it has no side effects.
///
/// # Examples
/// ```text
/// let value = RuntimeModePatch::Action;
/// ```
pub(super) enum RuntimeModePatch {
    /// Manual execution requests only. Example: `let value = RuntimeModePatch::Action`.
    Action,
    /// Request saved-input scheduling, subject to global authorization. Example: `let value = RuntimeModePatch::OnSave`.
    OnSave,
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
/// One omission-preserving TOML document, rejecting unknown fields. The loader applies
/// supplied values to [`super::model::Settings`]; nested patches preserve each section's omissions.
/// Constructing/decoding this record has no filesystem or process effects.
///
/// # Examples
/// ```text
/// let patch = SettingsPatch::default();
/// ```
pub(super) struct SettingsPatch {
    /// Formatting fields merged independently. Example: `SettingsPatch { format: FormatPatch::default(), ..Default::default() }`.
    pub format: FormatPatch,
    /// Snippet preference override. Example: `SettingsPatch { completion: CompletionPatch::default(), ..Default::default() }`.
    pub completion: CompletionPatch,
    /// Optional globally scoped admission limits. Example: `SettingsPatch { limits: Some(LimitsPatch::default()), ..Default::default() }`.
    pub limits: Option<LimitsPatch>,
    /// Report switch and rule overrides. Example: `SettingsPatch { diagnostics: DiagnosticsPatch::default(), ..Default::default() }`.
    pub diagnostics: DiagnosticsPatch,
    /// Symbol participation override. Example: `SettingsPatch { document_symbols: DocumentSymbolsPatch::default(), ..Default::default() }`.
    pub document_symbols: DocumentSymbolsPatch,
    /// Static lookup/indexing overrides. Example: `SettingsPatch { modules: ModulesPatch::default(), ..Default::default() }`.
    pub modules: ModulesPatch,
    /// Execution and budget overrides, not a process handle. Example: `SettingsPatch { runtime: RuntimePatch::default(), ..Default::default() }`.
    pub runtime: RuntimePatch,
    /// Raw file-key/command strings, later normalized and parsed by the loader. Example: `SettingsPatch { interpreter_commands: BTreeMap::from([("tool.cmake".into(), "cmake -P tool.cmake".into())]), ..Default::default() }`.
    pub interpreter_commands: BTreeMap<String, String>,
}
