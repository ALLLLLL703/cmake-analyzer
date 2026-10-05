use super::model::{Name, SemanticBuffer, Severity};
use crate::parser::nparser::model::{NParsedArgument, NParsedBuffer};

pub(super) fn argument_text(argument: &NParsedArgument) -> Option<&str> {
    match argument {
        NParsedArgument::Unquoted(text)
        | NParsedArgument::Quoted(text)
        | NParsedArgument::Bracked(text) => Some(text),
        _ => None,
    }
}

pub(super) fn name_text<'a>(parsed: &'a NParsedBuffer, name: &Name) -> Option<&'a str> {
    let command = parsed.commands.get(name.command)?;
    let text = if let Some(argument) = name.argument {
        argument_text(&command.args.get(argument)?.content)?
    } else {
        &command.name.content
    };
    text.get(name.bytes.clone())
}

pub(super) fn expands(text: &str) -> bool {
    text.contains("${") || text.contains("$ENV{") || text.contains("$CACHE{")
}

pub(super) fn literal(argument: &NParsedArgument) -> Option<&str> {
    let text = argument_text(argument)?;
    if matches!(argument, NParsedArgument::Bracked(_))
        || (!expands(text) && !text.contains(['\\', ';']) && !text.contains("$<"))
    {
        Some(text)
    } else {
        None
    }
}

pub(super) fn dynamic_arity(parsed: &NParsedBuffer, command: usize) -> bool {
    parsed.commands[command].args.iter().any(|arg| matches!(&arg.content, NParsedArgument::Unquoted(text) if expands(text) || text.contains(';')))
}

impl SemanticBuffer {
    pub fn name(&self, name: &Name) -> Option<&str> {
        name_text(&self.parsed, name)
    }

    pub fn has_errors(&self) -> bool {
        !self.parsed.error.is_empty()
            || self
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.content.severity == Severity::Error)
    }
}
