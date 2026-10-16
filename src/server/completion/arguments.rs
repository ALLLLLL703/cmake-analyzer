use super::model::Parameter;
use crate::parser::semanticer::schema::{ArgSchemaKind as Arg, CommandSchema};

pub(super) fn parameters(schema: &CommandSchema) -> Vec<Parameter> {
    let mut result = Vec::new();
    sequence(
        schema.args.iter().map(|argument| &argument.kind),
        &mut result,
    );
    result
}
fn sequence<'a>(args: impl IntoIterator<Item = &'a Arg>, out: &mut Vec<Parameter>) -> bool {
    for arg in args {
        if !append(arg, out) {
            return false;
        }
    }
    true
}
fn append(arg: &Arg, out: &mut Vec<Parameter>) -> bool {
    let name = match arg {
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
        Arg::Language => "language",
        Arg::TestName => "test_name",
        Arg::PackageName => "package",
        Arg::Url => "url",
        Arg::Expression => "expression",
    };
    out.push(Parameter::Placeholder(name.to_owned()));
    true
}
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
fn escape(value: &str, out: &mut String) {
    for character in value.chars() {
        if matches!(character, '\\' | '$' | '}') {
            out.push('\\');
        }
        out.push(character);
    }
}
