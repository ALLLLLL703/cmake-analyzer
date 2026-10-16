// AI-authored schema path resolution using command-qualified values, not final variable summaries.
use super::model::Candidate;
use crate::{
    parser::semanticer::{model::*, values::flags},
    server::model::Project,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
fn resolve(base: &Path, text: &str) -> Option<PathBuf> {
    crate::parser::interpreter_commands::resolve_path(base, Path::new(text)).ok()
}
fn candidates(
    command: &SemanticCommandRepresent,
    file: &Path,
    dirs: &ValueDirectories,
    bindings: &ValueBindings,
    project: &Project,
    observed: Option<&[String]>,
    only_outputs: bool,
    result: &mut Vec<Candidate>,
) {
    let evaluated = |index: usize| -> Option<String> {
        if let Some(arguments) = observed {
            if command.args.len() != arguments.len() {
                return None;
            }
            if command.args[..index]
                .iter()
                .zip(arguments)
                .any(|(raw, value)| {
                    raw.form == SemanticArgForm::Unquoted
                        && (value.is_empty() || value.contains([';', '\\']))
                })
            {
                return None;
            }
            return arguments.get(index).cloned().filter(|value| {
                !value.is_empty() && !value.contains(';') && !value.contains("$<")
            });
        }
        if command.args[..index]
            .iter()
            .any(|arg| arg.form == SemanticArgForm::Unquoted && arg.content.contains(['$', ';']))
        {
            return None;
        }
        bindings
            .expand(&command.args[index], Some(dirs))
            .map(|value| value.text().to_owned())
            .filter(|value| {
                !value.is_empty() && !value.contains([';', '*', '?']) && !value.contains("$<")
            })
    };
    let mut paths = command.paths.clone();
    if matches!(
        command.name.to_ascii_lowercase().as_str(),
        "add_custom_command" | "add_custom_target"
    ) {
        let mut output = false;
        for (argument, arg) in command.args.iter().enumerate() {
            match arg.content.as_str() {
                "OUTPUT" | "BYPRODUCTS" => output = true,
                "COMMAND"
                | "DEPENDS"
                | "MAIN_DEPENDENCY"
                | "WORKING_DIRECTORY"
                | "COMMENT"
                | "VERBATIM"
                | "APPEND"
                | "USES_TERMINAL"
                | "COMMAND_EXPAND_LISTS"
                | "DEPFILE"
                | "JOB_POOL"
                | "JOB_SERVER_AWARE"
                | "CODEGEN"
                | "SOURCES"
                | "DEPENDS_EXPLICIT_ONLY" => output = false,
                _ => {
                    if output {
                        paths.push(PathArgument {
                            argument,
                            kind: PathKind::Output,
                        });
                    }
                }
            }
        }
    }
    if command
        .name
        .eq_ignore_ascii_case("set_source_files_properties")
        && command.args.windows(2).any(|args| {
            args[0].content == "GENERATED"
                && !matches!(
                    args[1].content.to_ascii_lowercase().as_str(),
                    "0" | "false" | "off" | "no" | "n" | "ignore" | "notfound" | ""
                )
        })
    {
        if let Some(end) = command
            .args
            .iter()
            .position(|arg| arg.content == "PROPERTIES")
        {
            paths.extend((0..end).map(|argument| PathArgument {
                argument,
                kind: PathKind::Output,
            }));
        }
    }
    for path in paths {
        if only_outputs && path.kind != PathKind::Output {
            continue;
        }
        let Some(text) = evaluated(path.argument) else {
            continue;
        };
        let choices = if path.kind == PathKind::Module
            && Path::new(&text).extension().is_none()
            && Path::new(&text).components().count() == 1
        {
            if !project.settings.modules.enable {
                continue;
            }
            project
                .settings
                .modules
                .search_paths
                .iter()
                .chain(project.settings.modules.builtin_directory.iter())
                .filter_map(|base| resolve(&dirs.source.join(base), &format!("{text}.cmake")))
                .collect()
        } else {
            resolve(&dirs.source, &text).into_iter().collect()
        };
        result.push(Candidate {
            file: file.to_owned(),
            span: command.args[path.argument].span,
            kind: path.kind,
            choices,
            optional: path.kind == PathKind::Module
                && command.args.iter().any(|arg| arg.content == "OPTIONAL"),
            dependency: matches!(
                command.name.to_ascii_lowercase().as_str(),
                "include" | "add_subdirectory"
            ),
        });
    }
}
fn relevant(command: &SemanticCommandRepresent) -> bool {
    !command.paths.is_empty()
        || command.name.eq_ignore_ascii_case("add_custom_command")
        || command.name.eq_ignore_ascii_case("add_custom_target")
        || command
            .name
            .eq_ignore_ascii_case("set_source_files_properties")
}
pub(super) fn collect(project: &Project) -> Vec<Candidate> {
    let mut result = Vec::new();
    let workspace = &project.workspace;
    if let Some(graph) = workspace.root_graph() {
        let mut reads: HashMap<_, Vec<_>> = HashMap::new();
        for value in &graph.values {
            if value.location.kind == ReferenceKind::Read {
                reads
                    .entry((value.invocation, value.location.command))
                    .or_default()
                    .push(value);
            }
        }
        let shadowed: std::collections::HashSet<_> = graph
            .references
            .resolved_calls
            .iter()
            .map(|location| (location.file, location.command))
            .collect();
        for (invocation, visit) in graph.invocations.iter().enumerate() {
            let dirs = ValueDirectories {
                source_root: graph.invocations[0].directory.source.clone(),
                source: visit.directory.source.clone(),
                binary_root: graph.invocations[0].directory.binary.clone(),
                binary: visit.directory.binary.clone(),
                file: visit.file.clone(),
            };
            for command in &visit.snapshot.semantics.commands {
                if shadowed.contains(&(visit.snapshot.semantics.id, command.id)) {
                    continue;
                }
                if !relevant(command) {
                    continue;
                }
                let (conditional, deferred) = flags(&visit.snapshot.semantics, command);
                if deferred || !command.closed {
                    continue;
                }
                let mut bindings = ValueBindings::default();
                if let Some(values) =
                    reads.get(&(Some(ReferenceInvocationId::new(invocation)), command.id))
                {
                    for value in values {
                        bindings.assign(&value.name, value.possible_value.clone());
                    }
                }
                candidates(
                    command,
                    &visit.file,
                    &dirs,
                    &bindings,
                    project,
                    None,
                    conditional,
                    &mut result,
                );
            }
        }
    } else {
        for path in &project.members {
            if let Ok(Some(snapshot)) = workspace.snapshot(path) {
                let base = path.parent().unwrap_or(Path::new("/"));
                let dirs = ValueDirectories {
                    source_root: base.to_owned(),
                    source: base.to_owned(),
                    binary_root: None,
                    binary: None,
                    file: path.clone(),
                };
                let shadowed: std::collections::HashSet<_> = snapshot
                    .semantics
                    .functions
                    .iter()
                    .filter(|function| function.definition.is_some())
                    .flat_map(|function| &function.references)
                    .filter(|reference| reference.kind == ReferenceKind::Call)
                    .map(|reference| reference.command)
                    .collect();
                for command in &snapshot.semantics.commands {
                    if shadowed.contains(&command.id) || !relevant(command) {
                        continue;
                    }
                    let (conditional, deferred) = flags(&snapshot.semantics, command);
                    if deferred || !command.closed {
                        continue;
                    }
                    let mut bindings = ValueBindings::default();
                    for reference in &command.variable_references {
                        bindings.assign(&reference.name, reference.location.possible_value.clone());
                    }
                    candidates(
                        command,
                        path,
                        &dirs,
                        &bindings,
                        project,
                        None,
                        conditional,
                        &mut result,
                    );
                }
            }
        }
    }
    if let Some(layer) = workspace.runtime() {
        for event in &layer.trace.events {
            if !event.builtin {
                continue;
            }
            let Some(snapshot) = layer.documents.get(&event.file) else {
                continue;
            };
            let Some(id) = layer.source_commands[event.id.index()] else {
                continue;
            };
            let command = &snapshot.semantics.commands[id.id as usize];
            if !relevant(command) {
                continue;
            }
            let dirs = ValueDirectories {
                source_root: event.source_directory.as_ref().clone(),
                source: event.source_directory.as_ref().clone(),
                binary_root: None,
                binary: None,
                file: event.file.clone(),
            };
            candidates(
                command,
                &event.file,
                &dirs,
                &ValueBindings::default(),
                project,
                Some(&event.arguments),
                false,
                &mut result,
            );
        }
    }
    result
}
