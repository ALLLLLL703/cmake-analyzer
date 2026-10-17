use crate::{
    parser::{
        build_struct::model::{BuildEffect, Dependency},
        semanticer::{
            model::{CommandId, ReferenceKind, SemanticCommandRepresent},
            parsing::references::argument_name_span,
            schema::CommandSchemaFrom,
        },
    },
    server::{model::Service, text::TextIndex, uri},
    workspace::{
        Workspace,
        model::{DirectoryContext, DocumentSnapshot},
    },
};
use lspally::server::model::*;
use std::{collections::BTreeSet, path::Path};

fn native(command: &SemanticCommandRepresent) -> bool {
    (command.name.eq_ignore_ascii_case("include")
        || command.name.eq_ignore_ascii_case("find_package")
        || command.name.eq_ignore_ascii_case("add_subdirectory"))
        && command.schema.as_ref().is_some_and(|schema| {
            matches!(
                schema.from,
                CommandSchemaFrom::Builtin | CommandSchemaFrom::KeyWord
            )
        })
}
fn first_argument<'a>(
    snapshot: &'a DocumentSnapshot,
    command: &'a SemanticCommandRepresent,
) -> Option<(crate::model::TextSpan, &'a str, bool)> {
    if let Some(argument) = command.args.first() {
        return Some((
            argument_name_span(&snapshot.text, argument),
            &argument.content,
            argument.literal_text().is_some(),
        ));
    }
    if command.closed {
        return None;
    }
    // Unfinished literals have no parsed argument; recover only the first operand.
    let open = snapshot
        .lexed
        .lex
        .iter()
        .find(|token| token.span.start_byte >= command.span.end_byte)?;
    if open.content != crate::parser::lexer::model::LexContent::LeftParentheses {
        return None;
    }
    for error in &snapshot.lexed.errors {
        if error.span.start_byte < open.span.end_byte
            || !snapshot
                .text
                .get(open.span.end_byte..error.span.start_byte)?
                .trim()
                .is_empty()
        {
            continue;
        }
        let mut span = error.span;
        let bracket = match error.content {
            crate::parser::lexer::model::LexError::UnclosedStringLiteral => {
                span.start_byte += 1;
                false
            }
            crate::parser::lexer::model::LexError::UnclosedBracketArgument => {
                let raw = span.text(&snapshot.text)?;
                span.start_byte += raw.bytes().skip(1).take_while(|byte| *byte == b'=').count() + 2;
                let tail = snapshot.text.get(span.start_byte..)?;
                span.start_byte += if tail.starts_with("\r\n") {
                    2
                } else {
                    usize::from(tail.starts_with('\n'))
                };
                true
            }
            _ => continue,
        };
        let raw = span.text(&snapshot.text)?;
        if !bracket && raw.contains(['$', '\\']) {
            return None;
        }
        return Some((span, raw, true));
    }
    None
}
fn selected(
    snapshot: &DocumentSnapshot,
    position: &Position,
    encoding: CoordinateEncoding,
) -> ServerResult<Option<CommandId>> {
    let offset = TextIndex::new(&snapshot.text, encoding).byte(position)?;
    for command in snapshot
        .semantics
        .commands
        .iter()
        .filter(|command| native(command))
    {
        let Some((span, _, _)) = first_argument(snapshot, command) else {
            continue;
        };
        if !(span.start_byte <= offset
            && (offset < span.end_byte || !command.closed && offset == span.end_byte))
        {
            continue;
        }
        // Names inside ${...} retain ordinary variable-definition navigation.
        if command.variable_references.iter().any(|reference| {
            reference.location.span.start_byte <= offset
                && offset < reference.location.span.end_byte
        }) {
            return Ok(None);
        }
        return Ok(Some(command.id));
    }
    Ok(None)
}
fn shadowed(
    workspace: &Workspace,
    snapshot: &DocumentSnapshot,
    id: CommandId,
    path: &Path,
) -> bool {
    if let Some(graph) = workspace.root_graph() {
        if graph
            .references
            .resolved_calls
            .iter()
            .any(|reference| reference.file == snapshot.semantics.id && reference.command == id)
        {
            return true;
        }
    } else if snapshot
        .semantics
        .functions
        .iter()
        .filter(|function| function.definition.is_some())
        .flat_map(|function| &function.references)
        .any(|reference| reference.kind == ReferenceKind::Call && reference.command == id)
    {
        return true;
    }
    workspace
        .runtime()
        .and_then(|layer| layer.resolved_calls.get(path))
        .is_some_and(|commands| commands.contains(&id))
}
async fn add(service: &Service, path: &Path, targets: &mut BTreeSet<String>) {
    // An unsaved open target is usable even before it exists on disk.
    if let Some(document) = service.documents.get(path) {
        targets.insert(document.uri.0.clone());
    } else if tokio::fs::metadata(path)
        .await
        .is_ok_and(|metadata| metadata.is_file())
    {
        targets.insert(uri::from_path(path).0);
    }
}
async fn resolve(
    service: &Service,
    workspace: &Workspace,
    dependency: &Dependency,
    caller: &Path,
    directory: &DirectoryContext,
    scope: Option<usize>,
    targets: &mut BTreeSet<String>,
) {
    if let Some(path) = workspace
        .resolve_dependency_at(dependency, caller, directory, scope)
        .await
        .0
    {
        add(service, &path, targets).await;
    }
}
/// Only path arguments are intercepted; native command names retain symbol navigation.
pub(super) async fn query(
    service: &Service,
    path: &Path,
    position: &Position,
    cancel: &Cancellation,
) -> ServerResult<Option<Vec<Location>>> {
    let mut targets = BTreeSet::new();
    let mut matched = false;
    for owner in service.owners(path) {
        if cancel.is_cancelled() {
            return Err(ServerError::cancelled());
        }
        let workspace = &service.projects[owner].workspace;
        let Some(snapshot) = workspace.snapshot(path).ok().flatten() else {
            continue;
        };
        if !snapshot
            .semantics
            .commands
            .iter()
            .filter(|command| native(command))
            .any(|command| {
                !command.closed
                    || command.args.first().is_some_and(|argument| {
                        argument.span.row <= u64::from(position.line)
                            && argument.span.text(&snapshot.text).is_some_and(|text| {
                                u64::from(position.line)
                                    <= argument.span.row
                                        + text.bytes().filter(|byte| *byte == b'\n').count() as u64
                            })
                    })
            })
        {
            continue;
        }
        let source = snapshot.clone();
        let position = Position {
            line: position.line,
            character: position.character,
        };
        let encoding = service.encoding;
        let Some(id) = tokio::task::spawn_blocking(move || selected(&source, &position, encoding))
            .await
            .map_err(crate::server::service::internal)??
        else {
            continue;
        };
        if shadowed(workspace, &snapshot, id, path) {
            continue;
        }
        matched = true;
        let command = &snapshot.semantics.commands[id.id as usize];
        let fallback =
            crate::parser::build_struct::dependencies::from_command(command).or_else(|| {
                let (_, text, literal) = first_argument(&snapshot, command)?;
                Some(crate::parser::build_struct::dependencies::from_arguments(
                    &command.name.to_ascii_lowercase(),
                    crate::parser::build_struct::model::Argument {
                        text: text.to_owned(),
                        literal,
                    },
                    Vec::new(),
                ))
            });
        let dependency = snapshot
            .build
            .facts
            .iter()
            .filter(|fact| fact.source.command == id)
            .find_map(|fact| {
                if let BuildEffect::Dependency(dependency) = &fact.effect {
                    Some(dependency)
                } else {
                    None
                }
            })
            .or(fallback.as_ref());
        let mut has_edge = false;
        if let Some(graph) = workspace.root_graph() {
            for edge in graph.edges.iter().filter(|edge| {
                edge.source.analysis.file == snapshot.semantics.id.id && edge.source.command == id
            }) {
                if cancel.is_cancelled() {
                    return Err(ServerError::cancelled());
                }
                has_edge = true;
                if let Some(target) = &edge.resolved {
                    add(service, target, &mut targets).await;
                } else if let (Some(dependency), Some(caller)) =
                    (dependency, graph.invocations.get(edge.caller))
                {
                    resolve(
                        service,
                        workspace,
                        dependency,
                        path,
                        &caller.directory,
                        Some(caller.scope),
                        &mut targets,
                    )
                    .await;
                }
            }
        }
        if !has_edge {
            if let Some(dependency) = dependency {
                if let Some(graph) = workspace.root_graph() {
                    for caller in graph
                        .invocations
                        .iter()
                        .filter(|caller| caller.snapshot.semantics.id == snapshot.semantics.id)
                    {
                        resolve(
                            service,
                            workspace,
                            dependency,
                            path,
                            &caller.directory,
                            Some(caller.scope),
                            &mut targets,
                        )
                        .await;
                    }
                } else {
                    let root = workspace.root();
                    let source = root
                        .script_working_directory
                        .clone()
                        .unwrap_or_else(|| root.file.parent().unwrap_or(Path::new("/")).to_owned());
                    let directory = DirectoryContext {
                        source,
                        binary: root.binary_directory.clone(),
                    };
                    resolve(
                        service,
                        workspace,
                        dependency,
                        path,
                        &directory,
                        None,
                        &mut targets,
                    )
                    .await;
                }
            }
        }
        if let Some(layer) = workspace.runtime() {
            for dependency in &layer.trace.dependencies {
                if cancel.is_cancelled() {
                    return Err(ServerError::cancelled());
                }
                let event = &layer.trace.events[dependency.event.index()];
                if event.file == path && layer.source_commands[dependency.event.index()] == Some(id)
                {
                    if let Some(target) = &dependency.resolved {
                        add(service, target, &mut targets).await;
                    }
                }
            }
        }
    }
    if cancel.is_cancelled() {
        return Err(ServerError::cancelled());
    }
    Ok(matched.then(|| {
        targets
            .into_iter()
            .map(|uri| Location {
                uri: Uri(uri),
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 0,
                    },
                },
            })
            .collect()
    }))
}
