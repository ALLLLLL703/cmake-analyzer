use super::model::{Entry, TargetCollector, TargetDirectoryId, TargetEntryId, Use};
use crate::parser::build_struct::model::{
    BuildEffect, BuildStruct, BuildStructAnalyzer, TargetKind, TargetReference,
};
use crate::parser::semanticer::{
    model::{
        ReferenceIndex, ReferenceKind, SemanticArgForm, SemanticCommandRepresent, SemanticedBuffer,
    },
    parsing::{has_target_arguments, target_arguments},
    schema::CommandSchemaFrom,
    values::argument_value,
};

fn shadows(
    index: Option<&ReferenceIndex>,
    buffer: &SemanticedBuffer,
) -> std::collections::HashSet<crate::parser::semanticer::model::CommandId> {
    if let Some(index) = index {
        return index
            .resolved_calls
            .iter()
            .filter(|call| call.file == buffer.id)
            .map(|call| call.command)
            .collect();
    }
    buffer
        .functions
        .iter()
        .filter(|function| function.definition.is_some())
        .flat_map(|function| &function.references)
        .filter(|reference| reference.file == buffer.id && reference.kind == ReferenceKind::Call)
        .map(|reference| reference.command)
        .collect()
}
fn constructor(command: &SemanticCommandRepresent) -> bool {
    matches!(
        command.name.to_ascii_lowercase().as_str(),
        "add_library" | "add_executable" | "add_custom_target"
    )
}
impl TargetCollector {
    pub(crate) fn directory(&mut self, parent: Option<TargetDirectoryId>) -> TargetDirectoryId {
        let id = TargetDirectoryId::new(self.directories.len());
        self.directories.push(parent);
        id
    }
    pub(crate) fn file(
        &mut self,
        build: &BuildStruct,
        buffer: &SemanticedBuffer,
        directory: TargetDirectoryId,
        index: Option<&ReferenceIndex>,
    ) {
        let shadows = shadows(index, buffer);
        for fact in &build.facts {
            let BuildEffect::Target(target) = &fact.effect else {
                continue;
            };
            if !target.name.literal
                || target.name.text.is_empty()
                || shadows.contains(&fact.source.command)
            {
                continue;
            }
            let Some(command) = buffer.commands.get(fact.source.command.id as usize) else {
                continue;
            };
            let Some(argument) = command.args.first() else {
                continue;
            };
            let local = target.imported && !target.global || target.alias.is_some();
            let key = (build.analysis, command.id, local.then_some(directory));
            if self.definitions.contains_key(&key) {
                continue;
            }
            let mut declaration = target.clone_unbound();
            declaration.references.push(TargetReference {
                source: fact.source.clone(),
                span: argument.span,
                kind: ReferenceKind::Definition,
            });
            let class = if target.kind == TargetKind::Alias {
                if command.name.eq_ignore_ascii_case("add_executable") {
                    TargetKind::Executable
                } else {
                    TargetKind::Library
                }
            } else {
                target.kind
            };
            let id = TargetEntryId::new(self.entries.len());
            self.names
                .entry(target.name.text.clone())
                .or_default()
                .push(id);
            self.definitions.insert(key, id);
            self.entries.push(Entry {
                declaration,
                directory,
                local,
                class,
            });
        }
        let analyzer = BuildStructAnalyzer::new(buffer, build.analysis);
        for command in &buffer.commands {
            if !command.closed {
                continue;
            }
            let Some(schema) = &command.schema else {
                continue;
            };
            let native = matches!(
                schema.from,
                CommandSchemaFrom::Builtin | CommandSchemaFrom::KeyWord
            );
            if native && shadows.contains(&command.id) {
                continue;
            }
            if !has_target_arguments(schema) {
                continue;
            }
            let constructor = native && constructor(command);
            let expanded: Vec<_> = command
                .args
                .iter()
                .enumerate()
                .map(|(position, arg)| {
                    if arg.literal_text().is_none() {
                        argument_value(buffer, index, command, position)
                    } else {
                        None
                    }
                })
                .collect();
            let arguments: Vec<_> = command
                .args
                .iter()
                .zip(&expanded)
                .map(|(arg, value)| {
                    value
                        .as_ref()
                        .map_or(arg.content.as_str(), |value| value.text())
                })
                .collect();
            let unstable = command.args.iter().enumerate().position(|(position, arg)| {
                arg.form == SemanticArgForm::Unquoted
                    && arg.literal_text().is_none()
                    && expanded[position]
                        .as_ref()
                        .is_none_or(|value| value.text().is_empty() || value.text().contains(';'))
            });
            for (argument, role) in target_arguments(command, &arguments) {
                if argument == 0 && constructor {
                    continue;
                }
                // Unquoted expansion/list splitting can move all subsequent schema slots.
                if unstable.is_some_and(|unstable| unstable < argument) {
                    continue;
                }
                let arg = &command.args[argument];
                let Some(name) = arg.literal_text().filter(|name| !name.is_empty()) else {
                    continue;
                };
                self.uses.push(Use {
                    name: name.to_owned(),
                    role,
                    directory,
                    reference: TargetReference {
                        source: analyzer.source_for(command),
                        span: arg.span,
                        kind: ReferenceKind::Read,
                    },
                });
            }
        }
    }
}
pub(in crate::parser::build_struct) fn bind_local(
    build: &mut BuildStruct,
    buffer: &SemanticedBuffer,
) {
    let mut collector = TargetCollector::default();
    let directory = collector.directory(None);
    collector.file(build, buffer, directory, None);
    let mut references: std::collections::HashMap<_, _> = collector
        .finish()
        .into_iter()
        .filter_map(|mut target| {
            target
                .references
                .iter()
                .find(|reference| reference.kind == ReferenceKind::Definition)
                .map(|reference| reference.source.command)
                .map(|command| (command, std::mem::take(&mut target.references)))
        })
        .collect();
    for fact in &mut build.facts {
        if let BuildEffect::Target(target) = &mut fact.effect {
            target.references = references.remove(&fact.source.command).unwrap_or_default();
        }
    }
}
