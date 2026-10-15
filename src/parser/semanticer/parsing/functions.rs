// AI-authored function/macro definitions discovered from semantic blocks.
use super::references::argument_name_span;
use crate::parser::semanticer::model::*;

impl Semanticer {
    pub(crate) fn scan_functions(&self, source: &str, result: &mut SemanticedBuffer) {
        result.functions.clear();
        for block in &result.blocks {
            let kind = match block.kind {
                BlockKind::Function => FunctionKind::Function,
                BlockKind::Macro => FunctionKind::Macro,
                _ => continue,
            };
            let command = &result.commands[block.opening.id as usize];
            let Some(arg) = command.args.first() else {
                continue;
            };
            let Some(name) = arg.literal_text().filter(|name| !name.is_empty()) else {
                continue;
            };
            let location = |arg: &SemanticArg| ReferenceLocation {
                file: result.id,
                command: command.id,
                span: argument_name_span(source, arg),
                kind: ReferenceKind::Definition,
            };
            let definition = location(arg);
            let parameters = command
                .args
                .iter()
                .skip(1)
                .filter_map(|arg| {
                    arg.literal_text()
                        .filter(|name| !name.is_empty())
                        .map(|name| FunctionParameter {
                            name: name.to_owned(),
                            definition: location(arg),
                        })
                })
                .collect();
            result.functions.push(SemanticFunction {
                name: name.to_owned(),
                kind,
                block: block.id,
                scope: command.scope,
                definition: definition.clone(),
                parameters,
                references: vec![definition],
            });
        }
    }
}
