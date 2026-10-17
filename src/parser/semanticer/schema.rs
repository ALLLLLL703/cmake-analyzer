//! Versioned CMake argument trees shared by semantic binding and completion.
//! Constructing these records only allocates data; it never executes CMake commands.
//! [`builtin::schemas_for_version`] supplies native trees; callers may inject
//! [`CommandSchemaFrom::External`] trees for their own commands.
//!
//! A sequence consumes arguments left to right. Literal keywords distinguish
//! overloads; ordinary scalar roles usually accept any one token, not validated
//! numbers, paths or expressions. Full-call binding prefers more literal matches
//! and keeps only roles shared by equally strong matches. Completion accepts an
//! unfinished suffix and offers reachable alternatives instead. Thus `file(READ
//! input.txt result)` identifies an input file and output variable, while
//! `file(WRITE output.txt text)` identifies a generated path, not a variable write.
//!
//! ```text
//! [Version]                   [Injected schemas]
//!     |                              |
//!     v                              v
//! [Builtin layers] --------> [Effective command tree]
//!                                    |
//!                     +--------------+---------------+
//!                     v                              v
//!             [Complete call]                [Editing prefix]
//!                     |                              |
//!             [Agreed bindings]             [Possible names/keywords]
//! ```

pub mod builtin;
pub(crate) mod languages;
unsafe extern "Rust" {}
use crate::parser::semanticer::model::CommandKind;

/// One command's ordered argument grammar and origin; construction has no external effects.
/// Semantic parsing attaches this record to calls; completion reads its required
/// prefix and typed slots. [`ArgSchemaKind`] describes operands and groups;
/// [`CommandSchemaFrom`] determines whether native-only analysis is appropriate.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::schema::builtin::schemas_for_version;
/// let catalogue = schemas_for_version(4, 4).unwrap();
/// let read_write = &catalogue["file"];
/// assert_eq!(read_write.command_name, "file");
/// assert!(!read_write.args.is_empty());
/// ```
#[derive(Clone, Debug)]
pub struct CommandSchema {
    /// Ordered roots, each consuming one token or a structured group.
    /// Example: `schema.args.push(ArgSchema { kind: ArgSchemaKind::Out })` adds an output slot.
    pub args: Vec<ArgSchema>,
    /// Lookup spelling; native catalogue names are lowercase.
    /// Example: `assert_eq!(catalogue["set"].command_name, "set")` reads the registered name.
    pub command_name: String,
    /// Origin used by native-only build analysis and completion classification.
    /// Example: `schema.from = CommandSchemaFrom::External` marks an injected grammar.
    pub from: CommandSchemaFrom,
    /// Semantic command classification, separate from the argument grammar.
    /// Example: `matches!(schema.kind, CommandKind::Builtin(_))` recognizes native commands.
    pub kind: CommandKind,
}

/// Name classes offered by unfinished-argument matching; creating a role has no effects.
/// Completion maps [`ArgSchemaKind`] slots to these classes before collecting
/// visible variables, targets, languages or discovered module/package names.
/// Roles are suggestions, not proof that a name exists or is visible at runtime.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::schema::NameRole;
/// let role = NameRole::Library;
/// assert_ne!(role, NameRole::Executable);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameRole {
    /// Ordinary variable name, including output and unset slots.
    /// Example: `let role = NameRole::Variable;` describes the first operand of `set`.
    Variable,
    /// Target name without a class restriction.
    /// Example: `let role = NameRole::Target;` describes `target_link_libraries` operands.
    Target,
    /// Executable target name. Example: `let role = NameRole::Executable;` describes an executable alias operand.
    Executable,
    /// Library target name. Example: `let role = NameRole::Library;` describes a library alias operand.
    Library,
    /// Custom-target name. Example: `let role = NameRole::CustomTarget;` requests that target class.
    CustomTarget,
    /// Language name; custom languages remain legal scalars.
    /// Example: `let role = NameRole::LanguageType;` describes `enable_language(CXX)`.
    LanguageType,
    /// Bare module names discovered for a module/path slot.
    /// Example: `let role = NameRole::Module;` describes `include(CTest)`.
    Module,
    /// Package names discovered for package lookup.
    /// Example: `let role = NameRole::Package;` describes `find_package(ZLIB)`.
    Package,
}

/// A root operand or group in [`CommandSchema::args`]; construction only owns data.
/// [`ArgSchemaKind`] holds the actual tree; builtin builders wrap each root in
/// this record when assembling a command catalogue.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::schema::{ArgSchema, ArgSchemaKind};
/// let output = ArgSchema { kind: ArgSchemaKind::Out };
/// assert!(matches!(output.kind, ArgSchemaKind::Out));
/// ```
#[derive(Clone, Debug)]
pub struct ArgSchema {
    /// Operand meaning or nested grammar. Example: `argument.kind = ArgSchemaKind::Variable` changes the role to a variable read.
    pub kind: ArgSchemaKind,
}

/// Operand roles and tree nodes; constructing them only allocates owned data.
/// Used by [`CommandSchema`] builders and by semantic/completion matchers. Most
/// scalars consume one arbitrary argument; only [`Self::Keyword`] and
/// [`Self::CacheEntry`] enforce token spelling. Roles describe analysis intent,
/// not CMake evaluation. `BindValue` records intent but does not assign a value.
/// In examples below, `K` means `ArgSchemaKind`.
///
/// # Tree shape
/// ```text
///                [Choice: file modes]
///                  /             \
///                 v               v
///         [READ sequence]    [WRITE sequence]
///           /    |    \        /     |     \
///          v     v     v      v      v      v
///       Keyword Input Out  Keyword Output Repeat
///                                           |
///                                           v
///                                         String
/// ```
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::schema::ArgSchemaKind as K;
/// let read = vec![K::Keyword("READ".into()), K::InputFile, K::Out];
/// let write = vec![K::Keyword("WRITE".into()), K::OutputPath,
///     K::Repeat { args: vec![K::String], min: 0 }];
/// let modes = K::Choice(vec![read, write]);
/// ```
#[derive(Clone, Debug)]
pub enum ArgSchemaKind {
    /// Unspecified path; no required-input existence role. Example: `let hint = K::Path;` models a search hint.
    Path,
    /// Required source file role. Example: `let input = K::InputFile;` models `file(READ input.txt result)`'s input.
    InputFile,
    /// Required source directory role. Example: `let dir = K::InputDirectory;` models `add_subdirectory(src)`.
    InputDirectory,
    /// Required file-or-directory input role. Example: `let source = K::InputPath;` models the source of `file(RENAME old new)`.
    InputPath,
    /// Module name or module file input; also yields [`NameRole::Module`]. Example: `let module = K::ModulePath;` models `include(CTest)`.
    ModulePath,
    /// Generated/output path, not an input to warn about missing. Example: `let dest = K::OutputPath;` models `file(WRITE generated.txt text)`.
    OutputPath,
    /// Executable target name. Example: `let target = K::TargetExe;` models `add_executable(app main.c)`'s name.
    TargetExe,
    /// Library target name. Example: `let target = K::TargetLib;` models `add_library(core core.c)`'s name.
    TargetLib,
    /// Custom-target slot; the payload is retained metadata, not a literal constraint.
    /// Example: `let slot = K::CustomTarget("generated".into());` requests custom-target names.
    CustomTarget(
        /// Caller-supplied label; the matcher does not inspect it.
        /// Example: `if let K::CustomTarget(label) = K::CustomTarget("docs".into()) { assert_eq!(label, "docs"); }`.
        String,
    ),
    /// Arbitrary text operand. Example: `let text = K::String;` models a project description.
    String,
    /// Legacy open action operand, not an explicit keyword list. Example: `let action = K::Action;` consumes one operation token.
    Action,
    /// Variable-name read role. Example: `let list_name = K::Variable;` models `list(LENGTH items count)`'s `items`.
    Variable,
    /// One list-valued argument, not a repeated sequence. Example: `let options = K::List;` models a `cmake_parse_arguments` list operand.
    List,
    /// Ordinary variable write role; matching records a possible write, not execution.
    /// Example: `let output = K::Out;` models `file(READ input.txt result)`'s `result`.
    Out,
    /// Literal nonempty `CACHE{name}` token; never an ordinary output write.
    /// Example: `let cache = K::CacheEntry;` matches `CACHE{MODE}` in the 4.2 `set` signature.
    CacheEntry,
    /// Variable-name removal intent; offers variable names but is not an `Out` write.
    /// Example: `let removed = K::Unset;` models `unset(temp)`.
    Unset,
    /// Value associated with another argument; metadata only in this matcher.
    /// Example: `let value = K::BindValue { var_place: 0 };` describes values following `set`'s variable name.
    BindValue {
        /// Zero-based argument position naming the intended variable.
        /// Example: `K::BindValue { var_place: 0 }` points to the first argument.
        var_place: usize,
    },
    /// Unspecified single operand; snippet extraction stops here.
    /// Example: `let opaque = K::Unknown;` reserves one token without a typed role.
    Unknown,
    // AI-generated additions.
    /// Opaque tail accepting zero or more tokens; supplies no invented keywords.
    /// Example: `let tail = K::Unmodeled;` marks an incomplete external grammar.
    Unmodeled,
    /// Target name of any class. Example: `let dep = K::Target;` models `add_dependencies(app core)`'s operands.
    Target,
    /// Command/program name, without process execution. Example: `let program = K::Command;` models `execute_process(COMMAND tool)`.
    Command,
    /// Property name, not a catalogue of valid properties. Example: `let property = K::Property;` models `get_target_property(out app TYPE)`'s `TYPE`.
    Property,
    /// Legacy single condition token; structured conditions use [`Self::Grammar`]. Example: `let token = K::Condition;` consumes one operand.
    Condition,
    /// Integer intent; numeric validity is not checked. Example: `let index = K::Integer;` models a list index.
    Integer,
    /// Numeric intent, including fractional values; not parsed here. Example: `let seconds = K::Number;` models a timeout.
    Number,
    /// Boolean intent; spelling is not validated. Example: `let enabled = K::Boolean;` models `TLS_VERIFY ON`'s value.
    Boolean,
    /// Version or version-range intent; not parsed here. Example: `let version = K::Version;` models `find_package(ZLIB 1.2)`'s version.
    Version,
    /// Open language name; offers version-supported builtin names. Example: `let language = K::LanguageType;` models `project(app LANGUAGES CXX)`.
    LanguageType,
    /// Test-name operand. Example: `let name = K::TestName;` models `add_test(smoke tool)`'s name.
    TestName,
    /// Package-name operand yielding [`NameRole::Package`]. Example: `let name = K::PackageName;` models `find_package(ZLIB)`.
    PackageName,
    /// URL intent; never fetched by schema construction. Example: `let remote = K::Url;` models a `file(DOWNLOAD ...)` URL.
    Url,
    /// Unevaluated expression, such as a regular expression. Example: `let regex = K::Expression;` models `string(REGEX MATCH ...)`'s pattern.
    Expression,
    /// Arbitrary value with no name/path role. Example: `let item = K::Value;` models a command's uninterpreted argument.
    Value,
    /// Exact, case-sensitive literal. Example: `let read = K::Keyword("READ".into());` selects `file`'s read mode.
    Keyword(
        /// Required token spelling. Example: `K::Keyword("READ".into())` requires uppercase `READ`.
        String,
    ),
    /// Multi-token sequence rendered as one snippet placeholder.
    /// Example: `let condition = K::Grammar { placeholder: "condition".into(), args: vec![K::Value] };`.
    Grammar {
        /// Snippet label only, not a matched token. Example: `K::Grammar { placeholder: "condition".into(), args: vec![K::Value] }` displays one condition placeholder.
        placeholder: String,
        /// Ordered child operands used for real matching. Example: `K::Grammar { placeholder: "pair".into(), args: vec![K::Property, K::Value] }` consumes a property/value pair.
        args: Vec<ArgSchemaKind>,
    },
    /// Entire sequence may be absent. Example: `let optional = K::Optional(vec![K::Keyword("RESULT_VARIABLE".into()), K::Out]);`.
    Optional(
        /// Children included together or skipped. Example: `K::Optional(vec![K::Version])` permits one version or none.
        Vec<ArgSchemaKind>,
    ),
    /// Repeat an ordered group at least `min` times; no maximum is encoded.
    /// Example: `let files = K::Repeat { args: vec![K::InputFile], min: 1 };` requires at least one source token.
    Repeat {
        /// Group repeated as a unit. Example: `K::Repeat { args: vec![K::Property, K::Value], min: 1 }` models property/value pairs.
        args: Vec<ArgSchemaKind>,
        /// Minimum group count. Example: `K::Repeat { args: vec![K::String], min: 0 }` accepts an empty text list.
        min: usize,
    },
    /// Alternative ordered sequences, not a first-match dispatch.
    /// Example: `let mode = K::Choice(vec![vec![K::Keyword("READ".into()), K::InputFile, K::Out], vec![K::Keyword("WRITE".into()), K::OutputPath]]);`.
    Choice(
        /// Each inner vector is one full alternative. Example: `K::Choice(vec![vec![K::Target], vec![K::InputPath]])` offers target or path interpretations.
        Vec<Vec<ArgSchemaKind>>,
    ),
    /// Overlay multiple roles on one token, not a sequence; direct keywords constrain spelling.
    /// Example: `let default = K::AllOf(vec![K::BindValue { var_place: 0 }, K::Boolean]);` models an option's default.
    AllOf(
        /// Roles applied at the same position; use scalar roles rather than nested sequences.
        /// Example: `K::AllOf(vec![K::Out, K::Boolean])` tags one token with both intents.
        Vec<ArgSchemaKind>,
    ),
}

/// Schema origin used to distinguish native handling from injected metadata.
/// Construction has no side effects. [`CommandSchema::from`] carries this value;
/// build analysis only dispatches native commands for `Builtin` or `KeyWord`.
///
/// # Examples
/// ```
/// use cmake_analyzer::parser::semanticer::schema::CommandSchemaFrom;
/// let origin = CommandSchemaFrom::External;
/// assert!(matches!(origin, CommandSchemaFrom::External));
/// ```
#[derive(Clone, Debug)]
pub enum CommandSchemaFrom {
    /// Caller-injected signature; does not grant native build-command semantics.
    /// Example: `schema.from = CommandSchemaFrom::External` marks a custom command.
    External,
    /// Ordinary native command. Example: `matches!(catalogue["file"].from, CommandSchemaFrom::Builtin)` identifies native `file`.
    Builtin,
    /// Native control keyword, such as `if` or `function`.
    /// Example: `matches!(catalogue["if"].from, CommandSchemaFrom::KeyWord)` identifies block syntax.
    KeyWord,
}
