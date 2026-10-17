//! Build required common-prefix call snippets from command schemas.
//! Only returned strings/output vectors change; no source edits or I/O occur.

use super::model::Parameter;
use crate::parser::semanticer::schema::{ArgSchemaKind as Arg, CommandSchema};

/// Extract a safe required call prefix for snippet insertion; no side effects.
///
/// # Arguments
/// - `schema` `[in]`: Command grammar; optional or ambiguous suffixes must not be guessed.
///
/// # Returns
/// Owned parameters through the first unsupported or divergent grammar portion; may be empty.
///
/// # Algorithm
/// Walk required grammar nodes with sequence(). Optional nodes add nothing, repeats contribute their minimum count, choices keep their shared prefix and stop on divergence. Structured grammar contributes its single placeholder.
///
/// # Flow
/// ```text
/// [Schema argument sequence]
///             |
///             v
///      <next node exists?> --no--> [Return required prefix]
///             | yes                       ^
///             v                           |
///      [append required node]             |
///             |                           |
///      <safe to continue?> --no-----------+
///             | yes
///             +---------------------> [Repeat for next node]
/// ```
///
/// # Examples
/// ```text
/// let parameters = arguments::parameters(schema);
/// ```
pub(super) fn parameters(schema: &CommandSchema) -> Vec<Parameter> {
    let mut result = Vec::new();
    sequence(
        schema.args.iter().map(|argument| &argument.kind),
        &mut result,
    );
    result
}
/// Append consecutive safe parameters until a node cannot be completed; no I/O.
///
/// # Arguments
/// - `args` `[in]`: Ordered schema nodes to visit.
/// - `out` `[in,out]`: Parameter vector; appends a safe prefix, never clears existing entries, even on false.
///
/// # Returns
/// `true` if all nodes completed; `false` after the first unsupported/divergent node, retaining already appended parameters.
///
/// # Examples
/// ```text
/// let complete = sequence([&Arg::String, &Arg::Unknown], &mut out);
/// ```
fn sequence<'a>(args: impl IntoIterator<Item = &'a Arg>, out: &mut Vec<Parameter>) -> bool {
    for arg in args {
        if !append(arg, out) {
            return false;
        }
    }
    true
}
/// Append one node's required/common prefix without choosing an arbitrary overload.
/// Only the output vector changes; no I/O.
///
/// # Arguments
/// - `arg` `[in]`: Schema node, including nested sequence alternatives.
/// - `out` `[in,out]`: Appends literals/placeholders; preserves preexisting entries and any safe prefix on false.
///
/// # Returns
/// `true` if this node is safely complete, including omitted optional nodes; `false` for unknown/unmodeled nodes, empty choices or incomplete/divergent alternatives.
///
/// # Algorithm
/// Skip optionals, expand minimum repeats, and intersect choice prefixes by exact Parameter equality. For AllOf, use only its first non-BindValue child. Scalars become descriptive placeholders; structured Grammar uses its declared label.
///
/// # Flow
/// ```text
///  [Schema node]
///        |
///        +----------------+-----------------+
///        v                v                 v
///  [Optional]       [Choice/Repeat]       [Scalar/keyword]
///        |                |                 |
///  [no append]    [recursive prefixes]  [append one piece]
///        |                |
///        |         <alternatives agree?>
///        |           /              \
///        |        [true]      [append common; false]
///        +-----------+--------------+
///                    v
///           [Caller may continue or stop]
/// ```
///
/// # Examples
/// ```text
/// let complete = append(&Arg::Keyword("VERSION".into()), &mut out);
/// ```
fn append(arg: &Arg, out: &mut Vec<Parameter>) -> bool {
    let name = match arg {
        Arg::Grammar { placeholder, .. } => placeholder.as_str(),
        Arg::Optional(_) => return true,
        Arg::Repeat { args, min } => {
            for _ in 0..*min {
                if !sequence(args, out) {
                    return false;
                }
            }
            return true;
        }
        Arg::Choice(branches) => {
            let mut branches = branches.iter().map(|branch| {
                let mut result = Vec::new();
                let complete = sequence(branch, &mut result);
                (result, complete)
            });
            let Some((mut common, mut complete)) = branches.next() else {
                return false;
            };
            for (branch, finished) in branches {
                let shared = common
                    .iter()
                    .zip(&branch)
                    .take_while(|(left, right)| left == right)
                    .count();
                complete &= finished && shared == common.len() && shared == branch.len();
                common.truncate(shared);
            }
            // Do not pick an arbitrary overload or optional suffix.
            out.extend(common);
            return complete;
        }
        Arg::AllOf(args) => {
            return args
                .iter()
                .find(|arg| !matches!(arg, Arg::BindValue { .. }))
                .is_none_or(|arg| append(arg, out));
        }
        Arg::Keyword(value) => {
            out.push(Parameter::Literal(value.clone()));
            return true;
        }
        Arg::Unmodeled | Arg::Unknown => return false,
        Arg::Path | Arg::InputPath => "path",
        Arg::InputDirectory => "directory",
        Arg::OutputPath => "output_path",
        Arg::InputFile => "input_file",
        Arg::ModulePath => "module",
        Arg::TargetExe | Arg::TargetLib | Arg::Target | Arg::CustomTarget(_) => "target",
        Arg::String => "string",
        Arg::Action => "action",
        Arg::Variable | Arg::Out | Arg::Unset => "variable",
        Arg::BindValue { .. } | Arg::Value => "value",
        Arg::List => "list",
        Arg::Command => "command",
        Arg::Property => "property",
        Arg::Condition => "condition",
        Arg::Integer => "integer",
        Arg::Number => "number",
        Arg::Boolean => "boolean",
        Arg::Version => "version",
        Arg::LanguageType => "language",
        Arg::TestName => "test_name",
        Arg::PackageName => "package",
        Arg::Url => "url",
        Arg::Expression => "expression",
        Arg::CacheEntry => "CACHE{variable}",
    };
    out.push(Parameter::Placeholder(name.to_owned()));
    true
}
/// Render a complete command-call snippet; no source mutation or I/O.
///
/// # Arguments
/// - `name` `[in]`: Command spelling already validated as an identifier by candidate collection.
/// - `parameters` `[in]`: Required literal words and editable placeholder labels.
///
/// # Returns
/// Owned `name(...)$0` text; placeholders are numbered from one and their contents are snippet-escaped.
///
/// # Algorithm
/// Separate pieces with spaces. Literal words are escaped without tab stops;
/// placeholders increment a separate tab-stop counter and escape their default labels.
/// Finish with a closing parenthesis and the final cursor stop `$0`.
///
/// # Flow
/// ```text
/// [name + opening parenthesis]
///              |
///              v
///       <next parameter?> --none--> [append closing parenthesis + $0]
///          /          \
///     [Literal]   [Placeholder]
///          |          |
///     [escape]   [number + escape label]
///          +-----+----+
///                |
///                v
///       [Append piece; repeat]
/// ```
///
/// # Examples
/// ```text
/// let text = arguments::snippet("set", &[Parameter::Placeholder("variable".into())]);
/// assert_eq!(text, "set(${1:variable})$0");
/// ```
pub(super) fn snippet(name: &str, parameters: &[Parameter]) -> String {
    let mut text = format!("{name}(");
    let mut index = 0;
    for (position, parameter) in parameters.iter().enumerate() {
        if position > 0 {
            text.push(' ');
        }
        match parameter {
            Parameter::Literal(literal) => escape(literal, &mut text),
            Parameter::Placeholder(label) => {
                index += 1;
                text.push_str(&format!("${{{index}:"));
                escape(label, &mut text);
                text.push('}');
            }
        }
    }
    text.push_str(")$0");
    text
}
/// Append snippet-safe text by escaping backslash, dollar and closing brace; no I/O.
///
/// # Arguments
/// - `value` `[in]`: Raw literal or placeholder label.
/// - `out` `[in,out]`: String receiving escaped characters by append; existing contents are preserved.
///
/// # Returns
/// `()`: the escaped text has been appended.
///
/// # Examples
/// ```text
/// let mut text = String::new();
/// escape("$x}", &mut text);
/// assert_eq!(text, "\\$x\\}");
/// ```
fn escape(value: &str, out: &mut String) {
    for character in value.chars() {
        if matches!(character, '\\' | '$' | '}') {
            out.push('\\');
        }
        out.push(character);
    }
}
