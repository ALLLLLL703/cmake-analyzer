use super::{
    model::{Name, Namespace, Reference, ReferenceFrame},
    text::argument_text,
};
use crate::{
    model::TextSpan,
    parser::nparser::model::{NParsedArgument, NParsedBuffer},
};

pub(super) fn expansions(
    parsed: &NParsedBuffer,
    command: usize,
    argument: usize,
    scope: usize,
    output: &mut Vec<Reference>,
) {
    let arg = &parsed.commands[command].args[argument];
    if matches!(arg.content, NParsedArgument::Bracked(_)) {
        return;
    }
    let Some(text) = argument_text(&arg.content) else {
        return;
    };
    let mut position = arg.span;
    if matches!(arg.content, NParsedArgument::Quoted(_)) {
        position.start_byte += 1;
        position.column += 1;
    }
    position.end_byte = position.start_byte;
    let mut index = 0;
    let mut stack: Vec<ReferenceFrame> = Vec::new();
    while index < text.len() {
        let rest = &text[index..];
        let prefix = if rest.starts_with("${") {
            Some((2, Namespace::Variable))
        } else if rest.starts_with("$ENV{") {
            Some((5, Namespace::Environment))
        } else if rest.starts_with("$CACHE{") {
            Some((7, Namespace::Cache))
        } else {
            None
        };
        if let Some((length, namespace)) = prefix {
            if let Some(parent) = stack.last_mut() {
                parent.dynamic = true;
            }
            index += length;
            position.start_byte += length;
            position.end_byte = position.start_byte;
            position.column += length as u64;
            stack.push(ReferenceFrame {
                start: index,
                span: position,
                namespace,
                dynamic: false,
            });
            continue;
        }
        let Some(ch) = rest.chars().next() else {
            break;
        };
        if ch == '}' {
            if let Some(frame) = stack.pop() {
                output.push(Reference {
                    name: Name {
                        command,
                        argument: Some(argument),
                        bytes: frame.start..index,
                    },
                    namespace: frame.namespace,
                    scope,
                    span: TextSpan {
                        end_byte: position.start_byte,
                        ..frame.span
                    },
                    dynamic: frame.dynamic,
                    resolved: Vec::new(),
                });
            }
        }
        step(&mut position, ch);
        index += ch.len_utf8();
        if ch == '\\' {
            if let Some(escaped) = text[index..].chars().next() {
                step(&mut position, escaped);
                index += escaped.len_utf8();
            }
        }
    }
}

fn step(position: &mut TextSpan, ch: char) {
    position.start_byte += ch.len_utf8();
    position.end_byte = position.start_byte;
    if ch == '\n' {
        position.row += 1;
        position.column = 0;
    } else {
        position.column += 1;
    }
}
