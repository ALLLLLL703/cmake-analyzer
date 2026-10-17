//! Collect literal target declarations and schema operands; no I/O or CMake execution.
//! Native command shadows and unstable unquoted expansions cannot supply guessed target uses.
// Contains AI-generated documentation additions.
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

/// Collect calls bound to user functions so native target roles cannot leak through shadows. No
/// mutation or I/O.
///
/// # Arguments
/// - `index` `[in]` : Optional workspace reference index; used instead of local function references
///   when supplied.
/// - `buffer` `[in]`: Semantic file whose command IDs are selected.
///
/// # Returns
/// Set of this file's resolved user-call command IDs.
///
/// # Examples
/// ```text
/// let shadowed = shadows(Some(index), buffer);
/// ```
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
/// Classify native target constructors case-insensitively. No side effects.
///
/// # Arguments
/// - `command` `[in]`: Command whose name is inspected.
///
/// # Returns
/// True for add_library, add_executable or add_custom_target; false otherwise.
///
/// # Examples
/// ```text
/// let is_constructor = constructor(command);
/// ```
fn constructor(command: &SemanticCommandRepresent) -> bool {
    matches!(
        command.name.to_ascii_lowercase().as_str(),
        "add_library" | "add_executable" | "add_custom_target"
    )
}
impl TargetCollector {
    /// Append a target directory with its optional parent. No I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Collector; appends one directory, retaining existing IDs.
    /// - `parent` `[in]`: Ancestor in the same collector, or None for a root.
    ///
    /// # Returns
    /// Fresh collector-local directory ID.
    ///
    /// # Examples
    /// ```text
    /// let root = collector.directory(None);
    /// let child = collector.directory(Some(root));
    /// ```
    pub(crate) fn directory(&mut self, parent: Option<TargetDirectoryId>) -> TargetDirectoryId {
        let id = TargetDirectoryId::new(self.directories.len());
        self.directories.push(parent);
        id
    }
    /// Append literal target definitions and schema-identified reads from a file visit. Does not
    /// mutate snapshots or perform I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Collector; appends deduplicated entries and pending uses.
    /// - `build` `[in]`: Build facts from the same semantic revision.
    /// - `buffer` `[in]`: Semantic commands used for spans, schemas and expansion safety.
    /// - `directory` `[in]`: Use-site directory ID belonging to this collector.
    /// - `index` `[in]` : Optional owner-qualified references/values for shadows and argument
    ///   expansion.
    ///
    /// # Returns
    /// () after staging eligible names; binding is deferred to finish.
    ///
    /// # Algorithm
    /// Collect nonempty literal declarations unless shadowed, deduplicating global or
    /// directory-local definition keys. Inspect closed schema-bearing commands, excluding native
    /// shadows. Expand earlier operands only to select schema roles; uses themselves must be
    /// literal, and unstable unquoted expansion suppresses later slots.
    ///
    /// # Flow
    /// ```text
    /// [build facts]              [semantic commands]
    ///       |                           |
    ///  [literal/unshadowed?]      [schema target slots]
    ///       |                           |
    ///  [deduplicate definitions] [stable literal operands]
    ///       |                           |
    ///       +----> [entries]      [pending uses]
    ///                        \       /
    ///                          [finish]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// collector.file(build, semantics, directory, Some(index));
    /// ```
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
/// Overwrite every target fact's reference list with bindings from this file alone. No I/O or
/// semantic mutation.
///
/// # Arguments
/// - `build` `[in,out]` : Build output; overwrites target references, including empty lists for
///   unbound declarations.
/// - `buffer` `[in]`: Matching semantic snapshot.
///
/// # Returns
/// () after a fresh single-directory collection and resolution.
///
/// # Algorithm
/// Collect this file in one root target directory and finish normal visibility lookup. Index each
/// retained definition by command, then overwrite every target fact with that command's new
/// reference list or an empty list.
///
/// # Flow
/// ```text
/// [one local directory]
///           |
///  [collect + finish targets]
///           |
///  [references by definition command]
///           |
///      [target fact loop]
///        /           \
///  matched          absent
///     |                |
///  overwrite list  clear list
///     +--------+-------+
///              |
///      [updated build facts]
/// ```
///
/// # Examples
/// ```text
/// bind_local(&mut build, semantics);
/// ```
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
