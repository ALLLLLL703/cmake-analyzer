//! Owned build facts, snapshot-qualified source locations, and incremental extraction state.
//! Construction has no I/O; facts describe possible structure, not configured build results.
// Contains AI-generated documentation additions.
// AI-generated implementation.
use crate::{
    model::{Spanned, TextSpan},
    parser::semanticer::model::{BlockId, CommandId, ScopeId, SemanticError, SemanticedBuffer},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Qualifies IDs that are valid only within one semantic snapshot.
/// Identity of one file revision used to qualify otherwise reusable command IDs. No side effects.
///
/// [`SourceRef`] attaches this key to facts; [`BuildStruct`] groups one revision.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::{AnalysisKey};
/// let key = AnalysisKey { file: 7, revision: 2 };
/// ```
pub struct AnalysisKey {
    /// Workspace-local file ID, not a global path identity Example: `key.file == 7`.
    pub file: u32,
    /// Revision of that file; command IDs must not cross revisions Example: `key.revision == 2`.
    pub revision: u64,
}
#[derive(Clone, Debug)]
/// Origin and enclosing execution context of a build fact. No side effects.
///
/// [`BuildFact`] retains this origin; [`AnalysisKey`] qualifies its local IDs.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::*;
/// fn inspect(source: &SourceRef) {
///     let _ = source.analysis.revision;
/// }
/// ```
pub struct SourceRef {
    /// Owning file revision Example: `source.analysis.revision`.
    pub analysis: AnalysisKey,
    /// Index of the originating semantic command Example: `source.command.id`.
    pub command: CommandId,
    /// Full command range in UTF-8 bytes and source coordinates Example: `source.span.start_byte`.
    pub span: TextSpan,
    /// Immediate enclosing block, or no block at file level Example: `source.block.is_none()`.
    pub block: Option<BlockId>,
    /// File-local variable scope, or caller/file scope Example: `source.scope.is_none()`.
    pub scope: Option<ScopeId>,
    /// Inside an if or loop; does not prove the command ran Example: `source.conditional`.
    pub conditional: bool,
    /// Inside a function or macro body; not executed by extraction Example: `source.deferred`.
    pub deferred: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// Retained operand text plus whether it is a known static operand. Creating it performs no I/O
/// or expansion; traversal can later replace an expression with its conservatively expanded text.
///
/// [`Dependency`] and [`TargetDeclaration`] retain these operands, including unresolved
/// expressions.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::{Argument};
/// let arg = Argument { text: "core".into(), literal: true };
/// ```
pub struct Argument {
    /// Semantic content, not raw source delimiters Example: `arg.text.as_str()`.
    pub text: String,
    /// True when text is usable as a known static operand. Set by initial semantic classification
    /// or later successful conservative expansion. Example: `arg.literal`.
    pub literal: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Dependency lookup category and scope rule. Choosing it has no side effects.
///
/// [`Dependency`] carries operands; [`crate::workspace::model::DependencyEdge`] records lookup
/// results.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::DependencyKind;
/// let kind = DependencyKind::Include;
/// ```
pub enum DependencyKind {
    /// Search Find modules or package config entrypoints, sharing caller scope Example:
    /// `DependencyKind::Package` .
    Package,
    /// Load a file or bare module while keeping caller directory and scope Example:
    /// `DependencyKind::Include` .
    Include,
    /// Load child CMakeLists.txt in a copied child scope Example: `DependencyKind::Subdirectory`.
    Subdirectory,
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// Static include, package or child-directory request, not a loaded dependency. No side effects.
///
/// [`DependencyKind`] selects lookup; [`crate::workspace::Workspace`] resolves and visits it.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::*;
/// fn inspect(dependency: &Dependency) {
///     let _ = dependency.arguments.len();
/// }
/// ```
pub struct Dependency {
    /// Remaining original operands, preserving package search flags Example:
    /// `dependency.arguments.len()` .
    pub arguments: Vec<Argument>,
    /// Lookup and scope-sharing category Example: `dependency.kind == DependencyKind::Include`.
    pub kind: DependencyKind,
    /// First operand: file/module, package name or source directory Example:
    /// `dependency.path.text.as_str()` .
    pub path: Argument,
    /// Optional second subdirectory operand; unknown expressions remain unresolved Example:
    /// `dependency.binary.as_ref().map(|arg| &arg.text)` .
    pub binary: Option<Argument>,
    /// Missing file is permitted; packages are optional unless REQUIRED Example:
    /// `dependency.optional` .
    pub optional: bool,
    /// Literal NO_POLICY_SCOPE flag; recorded, not interpreted Example:
    /// `dependency.no_policy_scope` .
    pub no_policy_scope: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
/// Project metadata extracted without evaluating CMake. No side effects.
///
/// [`BuildEffect::Project`] carries this data; [`Argument`] retains unresolved metadata.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::*;
/// fn inspect(project: &ProjectDeclaration) {
///     let _ = project.name.text.as_str();
/// }
/// ```
pub struct ProjectDeclaration {
    /// First project operand Example: `project.name.text.as_str()`.
    pub name: Argument,
    /// VERSION operand, if supplied Example: `project.version.as_ref()`.
    pub version: Option<Argument>,
    /// DESCRIPTION operand, if supplied Example: `project.description.as_ref()`.
    pub description: Option<Argument>,
    /// HOMEPAGE_URL operand, if supplied Example: `project.homepage.as_ref()`.
    pub homepage: Option<Argument>,
    /// Explicit languages, empty for NONE, or default C and CXX Example: `project.languages.len()`.
    pub languages: Vec<Argument>,
    /// All operands after the name, including unmodeled metadata Example: `project.arguments.len()`
    /// .
    pub arguments: Vec<Argument>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Structural constructor class used to filter typed target uses. No side effects.
///
/// [`TargetDeclaration::kind`] stores it; aliases retain a separate original operand.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::TargetKind;
/// let kind = TargetKind::StaticLibrary;
/// ```
pub enum TargetKind {
    /// Executable constructor Example: `TargetKind::Executable`.
    Executable,
    /// Library with no explicit subtype Example: `TargetKind::Library`.
    Library,
    /// STATIC library Example: `TargetKind::StaticLibrary`.
    StaticLibrary,
    /// SHARED library Example: `TargetKind::SharedLibrary`.
    SharedLibrary,
    /// MODULE library Example: `TargetKind::ModuleLibrary`.
    ModuleLibrary,
    /// OBJECT library Example: `TargetKind::ObjectLibrary`.
    ObjectLibrary,
    /// INTERFACE library Example: `TargetKind::InterfaceLibrary`.
    InterfaceLibrary,
    /// UNKNOWN library Example: `TargetKind::UnknownLibrary`.
    UnknownLibrary,
    /// Custom target, with files only from SOURCES Example: `TargetKind::Custom`.
    Custom,
    /// Alias declaration with a separate original-target operand Example: `TargetKind::Alias`.
    Alias,
}
/// Snapshot-qualified target definition/read. The argument span retains delimiters.
#[derive(Clone, Debug)]
/// Snapshot-qualified definition or read of a literal target name. No side effects.
///
/// [`TargetDeclaration::references`] owns these links; [`SourceRef`] identifies the command.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::*;
/// fn inspect(reference: &TargetReference) {
///     let _ = reference.source.analysis.revision;
/// }
/// ```
pub struct TargetReference {
    /// Originating command and file revision Example: `reference.source.analysis.revision`.
    pub source: SourceRef,
    /// Raw target argument range, retaining quote/bracket delimiters Example:
    /// `reference.span.start_byte` .
    pub span: TextSpan,
    /// Definition or Read role used by navigation Example:
    /// `reference.kind == crate::parser::semanticer::model::ReferenceKind::Read` .
    pub kind: crate::parser::semanticer::model::ReferenceKind,
}
#[derive(Clone, Debug)]
/// Structural target metadata used by build inspection and target-name navigation. No side effects.
///
/// [`BuildStruct`] stores local references; [`crate::workspace::model::RootGraph::targets`] stores
/// owner-qualified copies.
///
/// # Flow
/// ```text
/// [shared arguments]       [shared sources]
///           |                    |
///           +----[declaration]----+
///                       |
///                [clone_unbound]
///                  /         \
///         [local binding] [workspace binding]
///                  |         |
///               [separate reference lists]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::*;
/// fn inspect(target: &TargetDeclaration) {
///     let _ = target.references.len();
/// }
/// ```
pub struct TargetDeclaration {
    /// File-local in BuildStruct; owner-qualified in RootGraph.targets.
    /// Definition/read locations bound in the current analysis context Example:
    /// `target.references.len()` .
    pub references: Vec<TargetReference>,
    /// Declared name; only literal nonempty names can bind Example: `target.name.text.as_str()`.
    pub name: Argument,
    /// Constructor and library subtype, or Alias Example: `target.kind == TargetKind::Alias`.
    pub kind: TargetKind,
    /// Literal IMPORTED flag; does not create an actual target Example: `target.imported`.
    pub imported: bool,
    /// Literal GLOBAL flag; makes imported visibility project-wide Example: `target.global`.
    pub global: bool,
    /// Original target operand when ALIAS was present Example:
    /// `target.alias.as_ref().map(|arg| &arg.text)` .
    pub alias: Option<Argument>,
    // Owner-qualified declarations share immutable source/argument payloads.
    /// Shared immutable source operands; empty for imports and aliases Example:
    /// `target.sources.len()` .
    pub sources: std::sync::Arc<[Argument]>,
    /// Shared immutable operands after the name, flags included Example: `target.arguments.len()`.
    pub arguments: std::sync::Arc<[Argument]>,
}
impl TargetDeclaration {
    /// Copy metadata without references tied to a previous analysis context.
    /// Copy target metadata but discard context-specific references. Shared source/argument arrays
    /// remain shared; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: Declaration whose metadata is borrowed, not changed.
    ///
    /// # Returns
    /// New declaration with an empty references list.
    ///
    /// # Examples
    /// ```text
    /// let rebound = target.clone_unbound();
    /// ```
    pub(crate) fn clone_unbound(&self) -> Self {
        Self {
            references: Vec::new(),
            name: self.name.clone(),
            kind: self.kind,
            imported: self.imported,
            global: self.global,
            alias: self.alias.clone(),
            sources: self.sources.clone(),
            arguments: self.arguments.clone(),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Source usage category recorded from literal target_sources keywords. No side effects.
///
/// [`SourceSection::visibility`] stores the optional category; no propagation is evaluated.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::Visibility;
/// let visibility = Visibility::Public;
/// ```
pub enum Visibility {
    /// Sources for the target itself Example: `Visibility::Private`.
    Private,
    /// Sources for the target and its consumers Example: `Visibility::Public`.
    Public,
    /// Sources for consumers Example: `Visibility::Interface`.
    Interface,
}
#[derive(Clone, Debug)]
/// Named target_sources file set with optional kind and grouped paths. No filesystem access.
///
/// [`SourceSection`] holds these sets; [`Argument`] preserves dynamic paths.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::*;
/// fn inspect(set: &FileSet) {
///     let _ = set.name.text.as_str();
/// }
/// ```
pub struct FileSet {
    /// FILE_SET name operand Example: `set.name.text.as_str()`.
    pub name: Argument,
    /// TYPE operand; built-in HEADERS also implies this kind Example: `set.kind.as_ref()`.
    pub kind: Option<Argument>,
    /// Operands routed by BASE_DIRS Example: `set.base_dirs.len()`.
    pub base_dirs: Vec<Argument>,
    /// File operands routed by FILES or the initial file mode Example: `set.files.len()`.
    pub files: Vec<Argument>,
}
#[derive(Clone, Debug)]
/// One visibility section of target_sources, containing plain paths and file sets. No side effects.
///
/// [`SourceDeclaration`] preserves section order; [`FileSet`] models named groups.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::*;
/// fn inspect(section: &SourceSection) {
///     let _ = section.visibility == Some(Visibility::Public);
/// }
/// ```
pub struct SourceSection {
    /// Explicit visibility, or None before any visibility keyword Example:
    /// `section.visibility == Some(Visibility::Public)` .
    pub visibility: Option<Visibility>,
    /// Plain source operands outside FILE_SET groups Example: `section.files.len()`.
    pub files: Vec<Argument>,
    /// Ordered named file-set declarations Example: `section.file_sets.len()`.
    pub file_sets: Vec<FileSet>,
}
#[derive(Clone, Debug)]
/// Ordered target_sources sections for one target operand. No I/O or target mutation.
///
/// [`SourceSection`] groups visibility; [`BuildEffect::Sources`] retains this declaration.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::*;
/// fn inspect(sources: &SourceDeclaration) {
///     let _ = sources.target.text.as_str();
/// }
/// ```
pub struct SourceDeclaration {
    /// Target operand, possibly unresolved Example: `sources.target.text.as_str()`.
    pub target: Argument,
    /// Nonempty or explicitly marked visibility sections in source order Example:
    /// `sources.sections.len()` .
    pub sections: Vec<SourceSection>,
    /// Original operands following the target name Example: `sources.arguments.len()`.
    pub arguments: Vec<Argument>,
}
#[derive(Clone, Debug)]
/// Possible build action extracted from one semantic command, without executing it. No side
/// effects.
///
/// [`BuildFact`] supplies its origin; [`crate::workspace::model::RootGraph`] orders visited facts.
///
/// # Flow
/// ```text
/// [semantic command]
///          |
///     [effect kind]
///      /       \
/// [structure] [variable writes]
///      |       |
///      +---[BuildFact]
///              |
///       [workspace replay]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::BuildEffect;
/// let effect = BuildEffect::Minimum { arguments: vec![] };
/// ```
pub enum BuildEffect {
    /// Possible output names; nonlocal destinations remain unmodeled Example:
    /// `BuildEffect::VariableWrites { names: vec!["OUT".into()], local: true }` .
    VariableWrites {
        /// Literal output names in argument order. Example:
        /// `BuildEffect::VariableWrites { names: vec!["OUT".into()], local: true }` .
        names: Vec<String>,
        // False retains nonlocal writes without guessing their destination.
        /// False for cache/parent writes whose destination is not modeled. Example:
        /// `BuildEffect::VariableWrites { names: vec![], local: false }` .
        local: bool,
    },
    /// Project metadata and language defaults Example:
    /// `if let BuildEffect::Project(project) = effect { let _ = project.name; }` .
    Project(/** Project metadata. Example: `BuildEffect::Project(p)`. */ ProjectDeclaration),
    /// Raw cmake_minimum_required operands Example: `BuildEffect::Minimum { arguments: vec![] }`.
    Minimum {
        /// Retained minimum-version operands. Example: `BuildEffect::Minimum { arguments: vec![] }`
        /// .
        arguments: Vec<Argument>,
    },
    /// Raw enable_language operands Example: `BuildEffect::Languages { arguments: vec![] }`.
    Languages {
        /// Retained language operands. Example: `BuildEffect::Languages { arguments: vec![] }`.
        arguments: Vec<Argument>,
    },
    /// Structural target declaration Example:
    /// `if let BuildEffect::Target(target) = effect { let _ = target.kind; }` .
    Target(/** Target metadata. Example: `BuildEffect::Target(t)`. */ TargetDeclaration),
    /// Target source sections Example:
    /// `if let BuildEffect::Sources(sources) = effect { let _ = sources.sections; }` .
    Sources(/** Source groups. Example: `BuildEffect::Sources(s)`. */ SourceDeclaration),
    /// Target operand and raw link operands, not resolved libraries Example:
    /// `BuildEffect::Links { target: arg, arguments: vec![] }` .
    Links {
        /// Target operand, not an evaluated target. Example:
        /// `BuildEffect::Links { target: arg, arguments: vec![] }` .
        target: Argument,
        /// Raw link items and visibility/configuration keywords. Example:
        /// `if let BuildEffect::Links { arguments, .. } = effect { let _ = arguments.len(); }` .
        arguments: Vec<Argument>,
    },
    /// Unresolved dependency request for workspace lookup Example:
    /// `if let BuildEffect::Dependency(dependency) = effect { let _ = dependency.path; }` .
    Dependency(/** Lookup request. Example: `BuildEffect::Dependency(d)`. */ Dependency),
}
#[derive(Clone, Debug)]
/// One ordered structural effect and its command origin. Construction has no side effects.
///
/// [`BuildStruct::facts`] owns these; [`BuildEffect`] describes the retained effect.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::*;
/// fn inspect(fact: &BuildFact) {
///     let _ = fact.source.analysis.revision;
/// }
/// ```
pub struct BuildFact {
    /// Revision-qualified command location and control context Example:
    /// `fact.source.analysis.revision` .
    pub source: SourceRef,
    /// Structural payload; not proof that CMake executed it Example:
    /// `matches!(fact.effect, BuildEffect::Target(_))` .
    pub effect: BuildEffect,
}
#[derive(Clone, Debug)]
/// An extraction problem, currently a missing required first operand. No side effects.
///
/// [`BuildStruct::diagnostics`] stores these separately from semantic errors; [`SourceRef`] locates
/// them.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::*;
/// fn inspect(diagnostic: &BuildDiagnostic) {
///     let _ = diagnostic.source.command.id;
/// }
/// ```
pub struct BuildDiagnostic {
    /// Command that could not supply its build fact Example: `diagnostic.source.command.id`.
    pub source: SourceRef,
    /// Human-readable extraction failure Example: `diagnostic.message.as_str()`.
    pub message: String,
}
#[derive(Clone, Debug)]
/// File-local facts; child files belong to the Workspace graph.
/// File-local ordered facts and extraction diagnostics, owning all retained payloads. No I/O.
///
/// [`BuildStructAnalyzer::analyze`] creates this output; [`crate::workspace::model::RootGraph`]
/// owns child visits.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::{BuildStruct, AnalysisKey};
/// let build = BuildStruct { analysis: AnalysisKey { file: 0, revision: 1 }, facts: vec![], diagnostics: vec![] };
/// ```
pub struct BuildStruct {
    /// File revision shared by all fact origins Example: `build.analysis.revision`.
    pub analysis: AnalysisKey,
    /// Ordered effects; one command can emit a build effect and variable writes Example:
    /// `build.facts.len()` .
    pub facts: Vec<BuildFact>,
    /// Extraction errors, separate from semantic diagnostics Example: `build.diagnostics.len()`.
    pub diagnostics: Vec<BuildDiagnostic>,
}
// AI-generated display model.
/// Borrowed streaming view of build facts and optional semantic diagnostics. Creating it does not
/// write output.
///
/// [`BuildStruct::display`] constructs the adapter; [`Self::with_semantic_diagnostics`] adds
/// borrowed errors.
///
/// # Flow
/// ```text
/// [borrowed facts]         [borrowed errors/warnings]
///         |                            |
///         +----> [Display formatter] <-+
///                          |
///                  [escape + optional style]
///                          |
///                   [caller output sink]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::BuildStruct;
/// fn view(build: &BuildStruct) -> String { build.display(false).to_string() }
/// ```
pub struct BuildDisplay<'a> {
    /// Build facts borrowed for rendering Example: `view.buffer.facts.len()`.
    pub(super) buffer: &'a BuildStruct,
    /// Whether the formatter emits styled text Example: `view.colored`.
    pub(super) colored: bool,
    /// Borrowed earlier-pass errors, initially empty Example: `view.semantic_diagnostics.len()`.
    pub(super) semantic_diagnostics: &'a [Spanned<SemanticError>],
    /// Borrowed earlier-pass warnings, initially empty Example: `view.semantic_warnings.len()`.
    pub(super) semantic_warnings: &'a [Spanned<crate::parser::semanticer::model::SemanticWarn>],
}

// AI-authored per-command effect cache. Ranges point into the previous owned output.
/// Per-command effect cache that replaces shared build output on update. No I/O.
///
/// [`Self::update`] reuses unaffected effects; [`BuildStructAnalyzer::analyze`] is the
/// full-analysis counterpart.
///
/// # Flow
/// ```text
/// [old units]       [new commands]
///      |                  |
///      +------> [compare spans/schema]
///                      /       \
///                   reuse     extract
///                      \       /
///                    [rebind references]
///                            |
///                    [replace shared buffer]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::*;
/// fn inspect(cache: &IncrementalBuildStruct) {
///     let _ = cache.buffer.facts.len();
/// }
/// ```
pub struct IncrementalBuildStruct {
    /// Current immutable owned result; previous Arc readers remain valid Example:
    /// `cache.buffer.facts.len()` .
    pub buffer: std::sync::Arc<BuildStruct>,
    /// Counts of command units recomputed and reused by the last operation Example:
    /// `cache.work.reused` .
    pub work: crate::model::incremental::IncrementalWork,
    /// Previous command spans, schema identities and result ranges Example: `cache.units.len()`.
    pub(super) units: Vec<BuildUnit>,
}
/// Cached output ranges for a single semantic command. No side effects.
///
/// [`IncrementalBuildStruct`] uses schema identity and mapped spans to decide reuse.
///
/// # Examples
/// ```text
/// let fact_range = unit.facts.clone();
/// ```
pub(super) struct BuildUnit {
    /// Previous full command range Example: `unit.span.start_byte`.
    pub span: TextSpan,
    /// Shared schema identity required for effect reuse Example: `unit.schema.is_some()`.
    pub schema: Option<std::sync::Arc<crate::parser::semanticer::schema::CommandSchema>>,
    /// Half-open slice range in the previous build facts Example: `unit.facts.clone()`.
    pub facts: std::ops::Range<usize>,
    /// Half-open slice range in previous extraction diagnostics Example: `unit.diagnostics.clone()`
    /// .
    pub diagnostics: std::ops::Range<usize>,
}

/// Temporary borrow; the returned BuildStruct owns its payloads.
/// Temporary semantic-buffer borrow for extracting owned structural facts. No I/O.
///
/// [`Self::new`] sets the origin; [`Self::analyze`] extracts without executing dependencies.
///
/// # Flow
/// ```text
/// [semantic commands]      [origin key]
///          |                    |
///          v                    |
///  [native effects / Out writes]+
///          |
///          v
///  [ordered facts] ----> [local target binding]
///                               |
///                          [owned output]
/// ```
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::build_struct::model::{AnalysisKey, BuildStruct, BuildStructAnalyzer};
/// use cmake_analyzer::parser::semanticer::model::SemanticedBuffer;
/// fn extract(buffer: &SemanticedBuffer) -> BuildStruct {
///     BuildStructAnalyzer::new(buffer, AnalysisKey { file: 0, revision: 1 }).analyze()
/// }
/// ```
pub struct BuildStructAnalyzer<'a> {
    /// Semantic commands and block ancestry read during extraction Example:
    /// `analyzer.buffer.commands.len()` .
    pub buffer: &'a SemanticedBuffer,
    /// File revision assigned to every emitted source reference Example: `analyzer.origin.revision`
    /// .
    pub origin: AnalysisKey,
}
impl<'a> BuildStructAnalyzer<'a> {
    /// Borrow semantic input and assign the origin for later extraction. No I/O or analysis occurs.
    ///
    /// # Arguments
    /// - `buffer` `[in]`: Semantic commands and block ancestry; not modified.
    /// - `origin` `[in]`: File/revision identity to stamp onto emitted facts.
    ///
    /// # Returns
    /// Borrowed analyzer; its later output owns all payloads.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::build_struct::model::{AnalysisKey, BuildStructAnalyzer};
    /// use cmake_analyzer::parser::semanticer::model::SemanticedBuffer;
    /// let semantic = SemanticedBuffer::default();
    /// let analyzer = BuildStructAnalyzer::new(&semantic, AnalysisKey { file: 0, revision: 1 });
    /// ```
    pub fn new(buffer: &'a SemanticedBuffer, origin: AnalysisKey) -> Self {
        Self { buffer, origin }
    }
}
