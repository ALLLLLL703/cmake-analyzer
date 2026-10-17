//! Snapshot-local semantic records and analyzer state.
//!
//! Command, block and scope IDs index separate vectors in [`SemanticedBuffer`].
//! A byte span belongs to the source snapshot that produced it; never reuse an ID
//! or span against an unrelated or updated snapshot. Possible writes are not runtime facts.
// Contains AI-generated documentation additions.
// AI-generated import change.
pub use super::path_model::*;
pub use super::reference_model::*;
pub use super::value_model::*;
use std::{collections::HashMap, sync::Arc};

use crate::{
    model::{Spanned, TextSpan},
    parser::{
        nparser::model::{NParsedBuffer, NParsedError},
        semanticer::schema::CommandSchema,
    },
};

// AI-authored incremental analysis cache.
/// Per-file analysis cache that publishes immutable snapshots after edits. Workspace document analysis uses this to retain command payloads, not runtime state.
///
/// [`Self::update`] update remaps or recomputes commands.
///
/// # Flow
/// ```text
///  [cached source + analyzer]       [published Arc snapshot]
///              |                             |
///              v                             v
///          [update]                    [existing readers]
///           /    \
///  [reuse payloads] [rescan changed]
///           |    |
///           +--+-+
///              v
///  [rebuild dependent semantic facts]
///              |
///              v
///        [replace Arc] -----> [new readers]
/// ```
/// # Examples
/// ```
/// use cmake_analyzer::parser::{
///     lexer::model::IncrementalLexer,
///     nparser::model::IncrementalNParser,
///     semanticer::model::{FileId, IncrementalSemanticer},
/// };
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let source = "cmake_minimum_required(VERSION 3.31)\nset(x yes)";
/// let lexer = IncrementalLexer::new(source);
/// let parser = IncrementalNParser::new(source, &lexer.buffer);
/// let cache = IncrementalSemanticer::new(source, parser.buffer, FileId { id: 0 }).await;
/// assert_eq!(cache.buffer.commands.len(), 2);
/// # });
/// ```
pub struct IncrementalSemanticer {
    /// Published snapshot; old Arc clones remain valid after updates.
    /// Example: `cache.buffer.commands.len()`.
    pub buffer: Arc<SemanticedBuffer>,
    /// Counts of commands extracted again or reused in the last pass.
    /// Example: `cache.work.reused`.
    pub work: crate::model::incremental::IncrementalWork,
    /// Whether the last pass rebuilt blocks and scopes.
    /// Example: `cache.structure_rebuilt`.
    pub structure_rebuilt: bool,
    /// Whether the builtin/template variable seed was rebuilt.
    /// Example: `cache.templates_rebuilt`.
    pub templates_rebuilt: bool,
    /// Mutable extraction engine holding the current parser nodes and schemas.
    /// Example: `cache.analyzer.cursor.offset`.
    pub(super) analyzer: Semanticer,
    /// Source text matching the published spans.
    /// Example: `cache.source.len()`.
    pub(super) source: std::sync::Arc<str>,
    /// Unbound builtin/template records cloned before rebuilding writes and reads.
    /// Example: `cache.variable_seed.len()`.
    pub(super) variable_seed: Vec<SemanticVariable>,
}

/// Analyzer over parser nodes and a version-selected command catalog. Parsing mutates the engine and returns a separate semantic snapshot; construction performs no I/O.
///
/// [`Self::parse`] parse runs the ordered static passes.
///
/// # Flow
/// ```text
///  [parser nodes]        [injected schemas]
///        |                       |
///        v                       v
///  [version selection] ---> [active catalog]
///        |                       |
///        +-----------+-----------+
///                    v
///               [parse passes]
///                /         \
///        [cursor changed] [new snapshot]
/// ```
/// # Examples
/// ```
/// use cmake_analyzer::parser::{nparser::model::NParsedBuffer, semanticer::model::Semanticer};
/// let analyzer = Semanticer::new(NParsedBuffer::default(), Default::default());
/// ```
pub struct Semanticer {
    /// Parser snapshot whose ranges must match the supplied source.
    /// Example: `analyzer.buffer.nodes.len()`.
    pub buffer: NParsedBuffer,
    /// Inspection position and original node count.
    /// Example: `analyzer.cursor.offset`.
    pub cursor: SemanticCursor,
    // AI-generated change.
    /// Active lowercase command schemas, refreshed by parse.
    /// Example: `analyzer.schemas.get("set")`.
    pub schemas: HashMap<String, Arc<CommandSchema>>,
    // AI-generated additions.
    /// Declared or host-detected version for the active catalog; absent before parsing or on detection failure.
    /// Example: `analyzer.version.is_none()`.
    pub version: Option<CMakeVersion>,
    /// Caller-provided schemas that override the selected builtin catalog.
    /// Example: `analyzer.injected_schemas.len()`.
    pub(super) injected_schemas: HashMap<String, Arc<CommandSchema>>,
}

/// Node position used by semantic inspection. Construction has no side effects.
///
/// [`Semanticer::peek`] peek reads at this offset.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::SemanticCursor;
/// let cursor = SemanticCursor::new(2);
/// assert_eq!(cursor.offset, 0);
/// ```
#[derive(Default, Debug)]
pub struct SemanticCursor {
    /// Zero-based parser-node position; parse sets it to the node count.
    /// Example: `cursor.offset = 1`.
    pub offset: usize,
    /// Parser-node count recorded at construction, not a byte length.
    /// Example: `cursor.max_length`.
    pub max_length: usize,
}

/// Immutable-by-convention semantic snapshot for document queries, display and workspace replay. All local IDs and byte spans are meaningful only with this snapshot and its source. Constructing a buffer has no I/O effects.
///
/// [`Semanticer::parse`] parse populates these arenas.
///
/// # Flow
/// ```text
///  [source snapshot]       [file identity]
///          |                      |
///          +----------+-----------+
///                     v
///             [command arena]
///              /     |      \
///         [blocks] [reads]  [arguments]
///            |       |         |
///         [scopes] [bindings] [possible writes]
///              \     |      /
///               [query/display users]
/// ```
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::{FileId, SemanticedBuffer};
/// let buffer = SemanticedBuffer::new_with_file_id(FileId { id: 7 });
/// assert!(buffer.commands.is_empty());
/// ```
#[derive(Default, Clone, Debug)]
pub struct SemanticedBuffer {
    /// Workspace file identity used to qualify reference locations; not a snapshot version.
    /// Example: `buffer.id.id`.
    pub id: FileId,
    /// Source-ordered commands indexed by CommandId.
    /// Example: `buffer.commands.get(0)`.
    pub commands: Vec<SemanticCommandRepresent>,
    /// Builtin and discovered variable records, separated by optional file-local scope.
    /// Example: `buffer.variables.len()`.
    pub variables: Vec<SemanticVariable>,
    /// Bound user definitions and encountered builtin commands.
    /// Example: `buffer.functions.len()`.
    pub functions: Vec<SemanticFunction>,
    /// Sparse read/write observations from the local static value pass.
    /// Example: `buffer.values.len()`.
    pub values: Vec<VariableValueAt>,
    /// Source-positioned variable-state warnings.
    /// Example: `buffer.warnings.len()`.
    pub warnings: Vec<Spanned<SemanticWarn>>,
    /// Reads, writes or calls with no static binding.
    /// Example: `buffer.unresolved_references.len()`.
    pub unresolved_references: Vec<UnresolvedReference>,
    /// Parser, version, command and block errors retained for reporting.
    /// Example: `buffer.diagnostics.len()`.
    pub diagnostics: Vec<Spanned<SemanticError>>,
    // AI-generated addition.
    /// Version used for catalog selection, or None when discovery failed.
    /// Example: `buffer.version`.
    pub version: Option<CMakeVersion>,
    // AI-generated additions.
    /// Recovered nested block arena indexed by BlockId.
    /// Example: `buffer.blocks.get(0)`.
    pub blocks: Vec<SemanticBlock>,
    /// Function/variable-block scope arena indexed by ScopeId.
    /// Example: `buffer.scopes.get(0)`.
    pub scopes: Vec<SemanticScope>,
    /// Root-level display/navigation entries, not every nested command.
    /// Example: `buffer.symbols.len()`.
    pub symbols: Vec<SemanticSymbol>,
}

// AI-generated implementation.
/// Borrowed formatting policy for a semantic snapshot. Inspection CLI callers use it to include colors or unchanged variables; construction does not write output.
///
/// [`SemanticedBuffer::display`] display creates this view.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::SemanticedBuffer;
/// let buffer = SemanticedBuffer::default();
/// let text = buffer.display(false).to_string();
/// assert!(text.contains("CMAKE SEMANTICS"));
/// ```
pub struct SemanticDisplay<'a> {
    /// Snapshot borrowed until formatting completes.
    /// Example: `view.buffer.commands.len()`.
    pub(super) buffer: &'a SemanticedBuffer,
    /// Whether to emit ANSI styles rather than plain text.
    /// Example: `view.colored`.
    pub(super) colored: bool,
    /// Whether untouched builtin records are shown.
    /// Example: `view.all_variables`.
    pub(super) all_variables: bool,
}

/// Recoverable semantic-analysis error data for diagnostics. Creating an error has no I/O or reporting effects.
///
/// [`SemanticedBuffer::diagnostics`] collects source-spanned errors.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// use cmake_analyzer::parser::nparser::model::NParsedError;
/// let value = SemanticError::NParsedError(NParsedError::UnclosedLeftParentheses);
/// ```
#[derive(Clone, Debug)]
pub enum SemanticError {
    /// Forwarded parser error.
    /// Example: `SemanticError::NParsedError(NParsedError::UnclosedLeftParentheses)`.
    NParsedError(
        /// Parser failure payload; e.g. `SemanticError::NParsedError(NParsedError::UnclosedLeftParentheses)`.
        NParsedError,
    ),
    /// Command missing from the active schema catalog and unresolved by static function binding.
    /// Example: `SemanticError::CommandNotFound("mystery".into())`.
    CommandNotFound(
        /// Unknown source spelling; e.g. `SemanticError::CommandNotFound("mystery".into())`.
        String,
    ),
    // AI-generated additions.
    /// Version declaration or host-version discovery failed.
    /// Example: `SemanticError::Version("invalid version".into())`.
    Version(
        /// Failure message; e.g. `SemanticError::Version("invalid version".into())`.
        String,
    ),
    /// Numeric version has no supported catalog; parsing falls back to the baseline.
    /// Example: `SemanticError::UnsupportedVersion(CMakeVersion { major: 9, minor: 0, patch: 0 })`.
    UnsupportedVersion(
        /// Unsupported version; e.g. `SemanticError::UnsupportedVersion(CMakeVersion { major: 9, minor: 0, patch: 0 })`.
        CMakeVersion,
    ),
    // AI-generated additions.
    /// Block opener has no matched closer during recovery.
    /// Example: `SemanticError::UnclosedBlock(BlockKind::If)`.
    UnclosedBlock(
        /// Unclosed keyword family; e.g. `SemanticError::UnclosedBlock(BlockKind::If)`.
        BlockKind,
    ),
    /// Closing keyword has no compatible open block.
    /// Example: `SemanticError::UnexpectedBlockEnd("endif".into())`.
    UnexpectedBlockEnd(
        /// Closing source spelling; e.g. `SemanticError::UnexpectedBlockEnd("endif".into())`.
        String,
    ),
    /// Else/elseif occurs outside the innermost if block.
    /// Example: `SemanticError::UnexpectedBranch("else".into())`.
    UnexpectedBranch(
        /// Branch spelling; e.g. `SemanticError::UnexpectedBranch("else".into())`.
        String,
    ),
    /// Branch follows an else in the same if block.
    /// Example: `SemanticError::InvalidIfBranch("elseif".into())`.
    InvalidIfBranch(
        /// Invalid branch spelling; e.g. `SemanticError::InvalidIfBranch("elseif".into())`.
        String,
    ),
}

/// One possible variable name in a file-local scope, used by completion and navigation. A builtin record does not prove initialization; writes describe source possibilities only. No I/O occurs on construction.
///
/// [`VariableChanging`] records raw possible-write sites.
///
/// # Flow
/// ```text
///  [catalog name]       [possible output slot]
///         |                      |
///         +-----------+----------+
///                     v
///             [name + local scope]
///                 /         \
///        [possible writes] [bound references]
///                 |         |
///                 +----+----+
///                      v
///              [hover/navigation]
/// ```
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::SemanticVariable;
/// let variable = SemanticVariable { name: "answer".into(), builtin: false, first_write: None, scope: None, changing: vec![], references: vec![] };
/// ```
#[derive(Clone, Debug)]
pub struct SemanticVariable {
    // AI-generated addition.
    /// Case-sensitive variable spelling.
    /// Example: `variable.name.as_str()`.
    pub name: String,
    /// Whether seeded from a builtin catalog or specialized builtin template.
    /// Example: `variable.builtin`.
    pub builtin: bool,
    /// First possible raw output-name argument span, including delimiters; None for unwritten builtins. Not proof of runtime initialization.
    /// Example: `variable.first_write.is_none()`.
    pub first_write: Option<TextSpan>,
    // AI-generated change.
    /// File-local variable scope; None denotes the surrounding file scope.
    /// Example: `variable.scope.is_none()`.
    pub scope: Option<ScopeId>,
    /// Deduplicated possible output writes in source order.
    /// Example: `variable.changing.len()`.
    pub changing: Vec<VariableChanging>,
    /// Definition/read/write locations attached by reference binding.
    /// Example: `variable.references.len()`.
    pub references: Vec<ReferenceLocation>,
}

/// Comment ranges attached to a declaration in the same immutable source snapshot. Hover callers render only these ranges; storing them has no I/O effects.
///
/// [`Self::render`] render strips comment syntax with a byte budget.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::Documentation;
/// let documentation = Documentation { spans: vec![] };
/// assert_eq!(documentation.render("", 80), "");
/// ```
#[derive(Clone, Debug)]
pub struct Documentation {
    /// Ordered full comment ranges, including comment delimiters.
    /// Example: `documentation.spans.len()`.
    pub spans: Vec<TextSpan>,
}
/// A source command that may write a variable, used for declaration documentation and navigation. It is not an execution event; constructing it has no side effects.
///
/// [`SemanticVariable::changing`] owns the write list.
///
/// # Examples
/// ```
/// use cmake_analyzer::{model::TextSpan, parser::semanticer::model::{CommandId, VariableChanging}};
/// let change = VariableChanging { documentation: None, command: CommandId { id: 0 }, span: TextSpan::default() };
/// ```
#[derive(Clone, Debug)]
pub struct VariableChanging {
    /// Leading comments shared with the writing command when available.
    /// Example: `change.documentation.is_some()`.
    pub documentation: Option<Arc<Documentation>>,
    /// Command index in the owning semantic snapshot.
    /// Example: `change.command.id`.
    pub command: CommandId,
    /// Raw Out argument range, including quote/bracket delimiters.
    /// Example: `change.span.start_byte`.
    pub span: TextSpan,
}
/// A source command with schema classification and snapshot-local structural links. Parser passes fill it for completion, dependency discovery and references; it never represents proof of execution. Construction performs no I/O.
///
/// [`SemanticArg`] stores argument spelling and raw source ranges.
///
/// # Flow
/// ```text
///  [raw command span]       [shared schema]
///           |                       |
///           v                       v
///        [arguments] ----------> [role matching]
///          /      \                /       \
///  [read names] [literal text] [paths] [possible writes]
///          |          |           |        |
///          +----------+-----------+--------+
///                     v
///               [snapshot queries]
/// ```
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// use cmake_analyzer::model::TextSpan;
/// let command = SemanticCommandRepresent {
///     documentation: None, id: CommandId { id: 0 }, kind: CommandKind::Other("custom".into()),
///     span: TextSpan::default(), args: vec![], variable_references: vec![], paths: vec![],
///     name: "custom".into(), closed: true, schema: None, full_span: TextSpan::default(), block: None, scope: None,
/// };
/// assert!(command.schema.is_none());
/// ```
#[derive(Clone, Debug)]
pub struct SemanticCommandRepresent {
    /// Adjacent standalone leading comments for function, macro or set declarations.
    /// Example: `command.documentation.is_some()`.
    pub documentation: Option<Arc<Documentation>>,
    /// Dense index into the owning command arena.
    /// Example: `command.id.id`.
    pub id: CommandId,
    /// Schema-derived builtin classification or unresolved command spelling.
    /// Example: `matches!(command.kind, CommandKind::Other(_))`.
    pub kind: CommandKind,
    /// Command-name range only; use full_span for the complete command.
    /// Example: `command.span.start_byte`.
    pub span: TextSpan,
    /// Arguments in source-slot order; delimiter forms retain their own spans.
    /// Example: `command.args.len()`.
    pub args: Vec<SemanticArg>,
    /// Simple expansion and schema-variable reads extracted before binding.
    /// Example: `command.variable_references.len()`.
    pub variable_references: Vec<ArgumentReference>,
    /// Schema filesystem roles indexed into args.
    /// Example: `command.paths.len()`.
    pub paths: Vec<PathArgument>,
    // AI-generated additions.
    /// Original command-name spelling, before case-insensitive lookup.
    /// Example: `command.name.as_str()`.
    pub name: String,
    /// Whether the parser found the outer closing parenthesis.
    /// Example: `command.closed`.
    pub closed: bool,
    /// Shared selected schema, or None for a command unknown to the catalog.
    /// Example: `command.schema.is_none()`.
    pub schema: Option<Arc<CommandSchema>>,
    // AI-generated additions.
    /// Complete parser-node range, including arguments and parentheses.
    /// Example: `command.full_span.end_byte`.
    pub full_span: TextSpan,
    /// Innermost recovered block; an opener belongs to the block it opens.
    /// Example: `command.block.is_some()`.
    pub block: Option<BlockId>,
    /// Active variable scope; a scope-creating opener uses the surrounding scope.
    /// Example: `command.scope.is_none()`.
    pub scope: Option<ScopeId>,
}

/// Static roles accumulated on a source argument; constructing a role does not expand or execute it.
///
/// [`SemanticArg::kind`] stores possibly overlapping roles.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = ArgKind::ReferencingVar;
/// ```
#[derive(Clone, Debug)]
pub enum ArgKind {
    /// Argument contains an extracted variable read.
    /// Example: `ArgKind::ReferencingVar`.
    ReferencingVar,
    /// Schema slot may write a variable.
    /// Example: `ArgKind::Out`.
    Out,
    /// Uninterpreted pure-text role; current extraction does not populate it.
    /// Example: `ArgKind::Pure`.
    Pure,

    /// Generic path marker; detailed filesystem roles use PathArgument.
    /// Example: `ArgKind::Path`.
    Path,
}

/// Argument payload text plus its raw source range and static roles. Completion and value expansion use the form to avoid treating bracket text as interpolation. Construction performs no I/O.
///
/// [`Self::literal_text`] recognizes text needing no expansion.
///
/// # Examples
/// ```
/// use cmake_analyzer::{model::TextSpan, parser::semanticer::model::{SemanticArg, SemanticArgForm}};
/// let arg = SemanticArg { kind: vec![], content: "hello".into(), span: TextSpan::default(), form: SemanticArgForm::Unquoted };
/// assert_eq!(arg.literal_text(), Some("hello"));
/// ```
#[derive(Clone, Debug)]
pub struct SemanticArg {
    /// Roles accumulated by analysis; more than one may apply.
    /// Example: `arg.kind.push(ArgKind::Out)`.
    pub kind: Vec<ArgKind>,

    /// Payload spelling without outer quote/bracket delimiters; escapes remain unprocessed.
    /// Example: `arg.content.as_str()`.
    pub content: String,
    // AI-generated additions.
    /// Raw argument range, including any outer delimiters.
    /// Example: `arg.span.start_byte`.
    pub span: TextSpan,
    /// Lexical form controlling literal and expansion rules.
    /// Example: `arg.form == SemanticArgForm::Bracket`.
    pub form: SemanticArgForm,
}

// AI-generated implementation.
impl SemanticArg {
    /// Borrow payload text only when no expansion, escaping or unquoted list splitting is needed. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: argument payload and lexical form.
    ///
    /// # Returns
    /// Some text for bracket arguments or simple quoted/unquoted payloads; None for parentheses, expansion/escape syntax or unquoted semicolons.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::model::TextSpan;
    /// let arg = SemanticArg { kind: vec![], content: "a;b".into(), span: TextSpan::default(), form: SemanticArgForm::Bracket };
    /// assert_eq!(arg.literal_text(), Some("a;b"));
    /// ```
    pub fn literal_text(&self) -> Option<&str> {
        match self.form {
            // Bracket arguments suppress expansion and escape processing.
            SemanticArgForm::Bracket => Some(&self.content),
            SemanticArgForm::Quoted | SemanticArgForm::Unquoted => {
                (!self.content.contains(['$', '\\'])
                    && (self.form != SemanticArgForm::Unquoted || !self.content.contains(';')))
                .then_some(self.content.as_str())
            }
            _ => None,
        }
    }
}

// AI-generated implementation.
/// Lexical argument shape used to decide literal/expansion rules. Creating a form has no side effects.
///
/// [`SemanticArg::literal_text`] checks which payloads need no expansion.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = SemanticArgForm::Unquoted;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticArgForm {
    /// Unquoted payload; list separators may split it in CMake.
    /// Example: `SemanticArgForm::Unquoted`.
    Unquoted,
    /// Double-quoted payload; simple substitutions may apply.
    /// Example: `SemanticArgForm::Quoted`.
    Quoted,
    /// Bracket-delimited payload with no expansion or escape processing.
    /// Example: `SemanticArgForm::Bracket`.
    Bracket,
    /// Inner left parenthesis retained as an argument token.
    /// Example: `SemanticArgForm::LeftParen`.
    LeftParen,
    /// Inner right parenthesis retained as an argument token.
    /// Example: `SemanticArgForm::RightParen`.
    RightParen,
}

/// Schema command classification, distinct from actual execution. Creating it has no side effects.
///
/// [`SemanticCommandRepresent::kind`] stores this classification.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = CommandKind::Builtin(BuiltinCommand::Other);
/// ```
#[derive(Clone, Debug)]
pub enum CommandKind {
    /// Known schema classification.
    /// Example: `CommandKind::Builtin(BuiltinCommand::Other)`.
    Builtin(
        /// Builtin schema role; e.g. `CommandKind::Builtin(BuiltinCommand::Other)`.
        BuiltinCommand,
    ),
    /// Command without a known builtin schema.
    /// Example: `CommandKind::Other("custom".into())`.
    Other(
        /// Original spelling; e.g. `CommandKind::Other("custom".into())`.
        String,
    ),
}

/// Builtin schema distinction used by block recovery. Creating a value has no side effects.
///
/// [`CommandKind::Builtin`] wraps this role.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = BuiltinCommand::KeyWord;
/// ```
#[derive(Clone, Debug)]
pub enum BuiltinCommand {
    /// Structural keyword such as if or function.
    /// Example: `BuiltinCommand::KeyWord`.
    KeyWord,
    /// Nonstructural builtin command.
    /// Example: `BuiltinCommand::Other`.
    Other,
}

/// Legacy keyword-name vocabulary; current block recovery uses command spellings instead. No side effects.
///
/// [`BlockKind`] is the active recovered block vocabulary.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = KeyWordCommand::If;
/// ```
#[derive(Clone, Debug)]
pub enum KeyWordCommand {
    /// If opener name.
    /// Example: `KeyWordCommand::If`.
    If,
    /// If closer name.
    /// Example: `KeyWordCommand::EndIf`.
    EndIf,
    /// Function opener name.
    /// Example: `KeyWordCommand::Function`.
    Function,
    /// Function closer name.
    /// Example: `KeyWordCommand::EndFunction`.
    EndFunction,
}

// AI-generated implementation.
/// Typed block index; construction performs no lookup or I/O.
///
/// [`SemanticedBuffer::blocks`] uses this domain. The number is valid only in the owning
/// snapshot or collector/match generation, not across rebuilt arenas.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::BlockId;
/// let id = BlockId { id: 0 };
/// assert_eq!(id.id, 0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId {
    /// Zero-based block identity number, not a byte offset.
    /// Example: `id.id = 1` stores identity number one.
    pub id: u32,
}

// AI-generated implementation.
/// Typed scope index; construction performs no lookup or I/O.
///
/// [`SemanticedBuffer::scopes`] uses this domain. The number is valid only in the owning
/// snapshot or collector/match generation, not across rebuilt arenas.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::ScopeId;
/// let id = ScopeId { id: 0 };
/// assert_eq!(id.id, 0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScopeId {
    /// Zero-based scope identity number, not a byte offset.
    /// Example: `id.id = 1` stores identity number one.
    pub id: u32,
}

// AI-generated implementation.
/// Recovered opener family for static source nesting. This does not select branches or run bodies.
///
/// [`SemanticBlock`] stores nesting and recovery ranges.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = BlockKind::If;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockKind {
    /// Conditional if/elseif/else source group.
    /// Example: `BlockKind::If`.
    If,
    /// While-loop source body, not evaluated iterations.
    /// Example: `BlockKind::While`.
    While,
    /// Foreach-loop source body, not expanded iterations.
    /// Example: `BlockKind::Foreach`.
    Foreach,
    /// Deferred function declaration body with a variable scope.
    /// Example: `BlockKind::Function`.
    Function,
    /// Deferred macro declaration body without a file-local variable scope.
    /// Example: `BlockKind::Macro`.
    Macro,
    /// Explicit block; arguments decide whether it creates a variable scope.
    /// Example: `BlockKind::Block`.
    Block,
}

// AI-generated implementation.
/// Recovered source nesting for control flow or declarations. Symbol and scope passes use these links without choosing branches; construction performs no I/O.
///
/// [`SemanticScope`] marks the subset of blocks that create variable scopes.
///
/// # Flow
/// ```text
///  [parent block]
///         |
///         v
///  [opening command] --- [direct commands]
///         |                    |
///         v                    v
///  [child blocks]       [optional closer]
///         |                    |
///         +----------+---------+
///                    v
///             [aggregate span]
/// ```
/// # Examples
/// ```text
/// let block = &buffer.blocks[0];
/// let opener = &buffer.commands[block.opening.id as usize];
/// ```
#[derive(Clone, Debug)]
pub struct SemanticBlock {
    /// Dense index in this snapshot block arena.
    /// Example: `block.id.id`.
    pub id: BlockId,
    /// Opening keyword family.
    /// Example: `block.kind == BlockKind::If`.
    pub kind: BlockKind,
    /// Direct commands, including opener and any matched closer; nested blocks are separate.
    /// Example: `block.contains.len()`.
    pub contains: Vec<CommandId>,
    /// Directly nested block indices.
    /// Example: `block.children.len()`.
    pub children: Vec<BlockId>,
    /// Enclosing block, or None at file level.
    /// Example: `block.parent.is_none()`.
    pub parent: Option<BlockId>,
    /// Opening command index.
    /// Example: `block.opening.id`.
    pub opening: CommandId,
    /// Matched closing command, or None after recovery of an unclosed block.
    /// Example: `block.closing.is_none()`.
    pub closing: Option<CommandId>,
    /// Whether a matching closer was recovered.
    /// Example: `block.closed`.
    pub closed: bool,
    /// Range from opener through closer, or through the last recovery command.
    /// Example: `block.span.end_byte`.
    pub span: TextSpan,
}

// AI-generated implementation.
/// Reason a file-local variable scope exists. Creating it does not change any bindings.
///
/// [`SemanticScope`] stores parent and owning block.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = ScopeKind::Function;
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeKind {
    /// Variable scope created by a function body.
    /// Example: `ScopeKind::Function`.
    Function,
    /// Block whose SCOPE_FOR includes VARIABLES or is omitted.
    /// Example: `ScopeKind::VariablesBlock`.
    VariablesBlock,
}

// AI-generated implementation.
/// A function or variable-block scope within one file snapshot. Scope assignment uses parent links for nesting; macros do not create these scopes. Construction performs no I/O.
///
/// [`ScopeKind`] identifies why a scope exists.
///
/// # Flow
/// ```text
///  [file scope: None]
///           |
///           v
///    [function opener] ----> [declaration uses parent]
///           |
///           v
///    [new body scope]
///        /       \
///  [commands] [nested variable block]
///                  |
///                  v
///            [child scope]
/// ```
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::{SemanticScope, ScopeId, ScopeKind, BlockId};
/// let scope = SemanticScope { id: ScopeId { id: 0 }, kind: ScopeKind::Function, block: BlockId { id: 0 }, parent: None };
/// ```
#[derive(Clone, Debug)]
pub struct SemanticScope {
    /// Dense index into the owning scope arena.
    /// Example: `scope.id.id`.
    pub id: ScopeId,
    /// Function body or explicitly variable-scoped block.
    /// Example: `scope.kind == ScopeKind::Function`.
    pub kind: ScopeKind,
    /// Block that creates the scope.
    /// Example: `scope.block.id`.
    pub block: BlockId,
    /// Enclosing file-local scope; None means file scope.
    /// Example: `scope.parent.is_none()`.
    pub parent: Option<ScopeId>,
}

// AI-generated implementation.
/// Snapshot-local root symbol identity for document presentation. Construction has no side effects.
///
/// [`SemanticSymbol`] adds a display range.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::*;
/// let value = SemanticSymbolKind::Block(BlockId { id: 0 });
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SemanticSymbolKind {
    /// Whole recovered top-level block.
    /// Example: `SemanticSymbolKind::Block(BlockId { id: 0 })`.
    Block(
        /// Block arena index; e.g. `SemanticSymbolKind::Block(BlockId { id: 0 })`.
        BlockId,
    ),
    /// Top-level command outside a recovered block.
    /// Example: `SemanticSymbolKind::Command(CommandId { id: 0 })`.
    Command(
        /// Command arena index; e.g. `SemanticSymbolKind::Command(CommandId { id: 0 })`.
        CommandId,
    ),
}

// AI-generated implementation.
/// Top-level presentation entry pointing to a command or recovered block. Document symbol consumers use its aggregate range; construction performs no I/O.
///
/// [`SemanticSymbolKind`] selects the indexed arena.
///
/// # Examples
/// ```
/// use cmake_analyzer::{model::TextSpan, parser::semanticer::model::{SemanticSymbol, SemanticSymbolKind, CommandId}};
/// let symbol = SemanticSymbol { kind: SemanticSymbolKind::Command(CommandId { id: 0 }), span: TextSpan::default() };
/// ```
#[derive(Clone, Debug)]
pub struct SemanticSymbol {
    /// Snapshot-local command or block identity.
    /// Example: `matches!(symbol.kind, SemanticSymbolKind::Command(_))`.
    pub kind: SemanticSymbolKind,
    /// Display range: whole command or whole block.
    /// Example: `symbol.span.end_byte`.
    pub span: TextSpan,
}

// AI-generated implementation.
/// Numeric CMake version for catalog selection, not a requirement that a project has run. Parsing and comparing this value perform no I/O.
///
/// [`Self::parse`] parses two or three numeric components.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::CMakeVersion;
/// let version = CMakeVersion::parse("3.31").unwrap();
/// assert_eq!(version.patch, 0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CMakeVersion {
    /// Major release component.
    /// Example: `version.major == 3`.
    pub major: u32,
    /// Minor release component.
    /// Example: `version.minor == 31`.
    pub minor: u32,
    /// Patch component, defaulting to zero for a two-part version.
    /// Example: `version.patch == 0`.
    pub patch: u32,
}

/// Typed target trail node index; construction performs no lookup or I/O.
///
/// [`OutputMatch::trail`] uses this domain. The number is valid only in the owning
/// snapshot or collector/match generation, not across rebuilt arenas.
///
/// # Examples
/// ```text
/// let id = TargetTrailId::new(0);
/// assert_eq!(id.index(), 0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct TargetTrailId(
    /// Zero-based target trail node arena slot; not range-checked.
    /// Example: `id.0` reads the stored slot inside this module.
    usize,
);
impl TargetTrailId {
    /// Wrap a target trail node arena slot without checking its bounds. No side effects.
    ///
    /// # Arguments
    /// - `index` `[in]`: zero-based slot in the owning arena.
    ///
    /// # Returns
    /// Typed index; no allocation or lookup.
    ///
    /// # Examples
    /// ```text
    /// let id = TargetTrailId::new(0);
    /// ```
    pub(super) fn new(index: usize) -> Self {
        Self(index)
    }
    /// Read the stored target trail node arena slot. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: typed index copied by value.
    ///
    /// # Returns
    /// The original zero-based slot; no bounds validation.
    ///
    /// # Examples
    /// ```text
    /// let id = TargetTrailId::new(0);
    /// assert_eq!(id.index(), 0);
    /// ```
    pub(super) fn index(self) -> usize {
        self.0
    }
}
/// One schema-match frontier state. Output matching ranks keyword evidence and merges states by consumed position; construction does not access external state.
///
/// [`TargetTrailId`] points into shared target-slot history.
///
/// # Flow
/// ```text
///  [consumed position]       [keyword evidence]
///           |                        |
///           +------------+-----------+
///                        v
///                  [merge frontier]
///                   /          \
///         [ordinary indices] [target trail ID]
///                   |          |
///                   v          v
///           [union/intersect] [shared arena]
/// ```
/// # Examples
/// ```text
/// let state = OutputMatch::default();
/// assert_eq!(state.position, 0);
/// ```
#[derive(Clone, Default)]
pub(super) struct OutputMatch {
    // Full target matches share histories; other roles use indices directly.
    /// Shared target-history tail; used only for full-call target matching.
    /// Example: `state.trail.is_none()`.
    pub trail: Option<TargetTrailId>,
    /// Next argument slot to consume, not a byte offset.
    /// Example: `state.position == 0`.
    pub position: usize,
    /// Matched slots for non-target roles, or the active completion slot.
    /// Example: `state.indices.len()`.
    pub indices: Vec<usize>,
    /// Count of exact completed keyword/cache-selector evidence used to rank branches.
    /// Example: `state.keywords == 0`.
    pub keywords: usize,
}

/// Typed workspace file index; construction performs no lookup or I/O.
///
/// [`ReferenceLocation::file`] uses this domain. The number is valid only in the owning
/// workspace identity mapping; it is not a snapshot version.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::FileId;
/// let id = FileId { id: 0 };
/// assert_eq!(id.id, 0);
/// ```
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileId {
    /// Zero-based workspace file identity number, not a byte offset.
    /// Example: `id.id = 1` stores identity number one.
    pub id: u32,
}

// AI-generated derive change.
/// Typed command index; construction performs no lookup or I/O.
///
/// [`SemanticedBuffer::commands`] uses this domain. The number is valid only in the owning
/// snapshot or collector/match generation, not across rebuilt arenas.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::model::CommandId;
/// let id = CommandId { id: 0 };
/// assert_eq!(id.id, 0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CommandId {
    /// Zero-based command identity number, not a byte offset.
    /// Example: `id.id = 1` stores identity number one.
    pub id: u32,
}

impl SemanticCursor {
    /// Start inspection at parser-node offset zero. No side effects.
    ///
    /// # Arguments
    /// - `max_length` `[in]`: parser-node count, not a source byte count.
    ///
    /// # Returns
    /// Cursor with offset zero and the supplied maximum.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// let cursor = SemanticCursor::new(3);
    /// assert_eq!(cursor.max_length, 3);
    /// ```
    pub fn new(max_length: usize) -> Self {
        Self {
            offset: 0,
            max_length,
        }
    }
}

impl Semanticer {
    /// Own parser nodes and normalize injected schema keys to lowercase. No filesystem or process access.
    ///
    /// # Arguments
    /// - `buffer` `[in]`: parser snapshot consumed by the analyzer.
    /// - `schemas` `[in]`: command-name/schema map consumed as caller overrides.
    ///
    /// # Returns
    /// Unparsed analyzer with no selected version and a cursor spanning all parser nodes.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::parser::nparser::model::NParsedBuffer;
    /// let analyzer = Semanticer::new(NParsedBuffer::default(), Default::default());
    /// assert!(analyzer.version.is_none());
    /// ```
    pub fn new(buffer: NParsedBuffer, schemas: HashMap<String, CommandSchema>) -> Self {
        let max_length = buffer.nodes.len();
        // AI-generated addition.
        let schemas: HashMap<_, _> = schemas
            .into_iter()
            .map(|(name, schema)| (name.to_ascii_lowercase(), Arc::new(schema)))
            .collect();
        Self {
            buffer,
            cursor: SemanticCursor::new(max_length),
            // AI-generated additions.
            injected_schemas: schemas.clone(),
            schemas,
            version: None,
        }
    }
}

impl SemanticedBuffer {
    /// Create an empty semantic snapshot with the supplied file identity. No side effects.
    ///
    /// # Arguments
    /// - `id` `[in]`: workspace file identity retained by value.
    ///
    /// # Returns
    /// Empty arenas and diagnostics; version is None.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// let buffer = SemanticedBuffer::new_with_file_id(FileId { id: 2 });
    /// assert_eq!(buffer.id.id, 2);
    /// ```
    pub fn new_with_file_id(id: FileId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    /// Append a skeletal command, then panic: this legacy constructor is unfinished. Prefer the parser extraction pass; this method does not perform I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: snapshot; appends one command before panicking.
    /// - `command_name` `[in]`: original command spelling consumed into the appended record.
    /// - `span` `[in]`: range used for both name and full command.
    /// - `kind` `[in]`: classification consumed into the record.
    ///
    /// # Returns
    /// Never returns; the declared mutable command reference is not produced.
    ///
    /// # Panics
    /// Always panics at the existing `todo!()` after appending the command. No rollback occurs.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::model::TextSpan;
    /// let result = std::panic::catch_unwind(|| {
    ///     let mut buffer = SemanticedBuffer::default();
    ///     buffer.new_command("set".into(), TextSpan::default(), CommandKind::Other("set".into()));
    /// });
    /// assert!(result.is_err());
    /// ```
    pub fn new_command<'cmd>(
        &'cmd mut self,
        command_name: String,
        span: TextSpan,
        kind: CommandKind,
    ) -> &'cmd mut SemanticCommandRepresent {
        let result = SemanticCommandRepresent {
            documentation: None,
            id: CommandId {
                id: self.commands.len() as u32,
            },
            args: Vec::new(),
            variable_references: Vec::new(),
            paths: Vec::new(),
            kind,
            span,
            // AI-generated additions.
            name: command_name,
            closed: true,
            schema: None,
            full_span: span,
            block: None,
            scope: None,
        };

        self.commands.push(result);

        todo!()
    }
}

// 需要干什么?
//首先需要虑出所有symbol
// and then extract all variable and its !Span and where it might be changed bind the scope with id
// then extract all function and its !Span(also should know the builtin function sign)
// then find all blocks and its Span
// then find all out args function and mark the variable changes
// need a scope struct to represent the scope of a variable (can be invert!)
// scope should not own the variable struct
// need a struct to represent the sign of a command(can be inject via lua script)
//
//
// writing in the paper is extremely important,,, i see
