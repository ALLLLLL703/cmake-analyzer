pub mod builtin;
pub(crate) mod languages;
unsafe extern "C" {}
use crate::parser::semanticer::model::CommandKind;

#[derive(Clone, Debug)]
pub struct CommandSchema {
    pub args: Vec<ArgSchema>,
    pub command_name: String,
    pub from: CommandSchemaFrom,
    pub kind: CommandKind,
}

/// Name classes offered by incomplete-argument schema matching.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameRole {
    Variable,
    Target,
    Executable,
    Library,
    CustomTarget,
    LanguageType,
}

#[derive(Clone, Debug)]
pub struct ArgSchema {
    pub kind: ArgSchemaKind,
}

#[derive(Clone, Debug)]
pub enum ArgSchemaKind {
    Path,
    InputFile,
    InputDirectory,
    InputPath,
    ModulePath,
    OutputPath,
    TargetExe,
    TargetLib,
    CustomTarget(String),
    String,
    Action,
    Variable,
    List,
    Out,
    /// A literal CACHE{name} token, not an ordinary variable output.
    CacheEntry,
    Unset,
    BindValue {
        var_place: usize,
    },
    Unknown,
    // AI-generated additions.
    Unmodeled,
    Target,
    Command,
    Property,
    Condition,
    Integer,
    Number,
    Boolean,
    Version,
    LanguageType,
    TestName,
    PackageName,
    Url,
    Expression,
    Value,
    Keyword(String),
    /// Structured multi-token operand rendered as one argument placeholder in snippets.
    Grammar {
        placeholder: String,
        args: Vec<ArgSchemaKind>,
    },
    Optional(Vec<ArgSchemaKind>),
    Repeat {
        args: Vec<ArgSchemaKind>,
        min: usize,
    },
    Choice(Vec<Vec<ArgSchemaKind>>),
    AllOf(Vec<ArgSchemaKind>),
}

#[derive(Clone, Debug)]
pub enum CommandSchemaFrom {
    External,
    Builtin,
    KeyWord,
}
