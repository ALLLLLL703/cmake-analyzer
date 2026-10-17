//! Resolve native dependency path operands to file-start definition locations.
//! Graph edges, static candidates and installed runtime observations are distinct from symbol bindings.
//! Reads may inspect filesystem metadata/search paths; no CMake process is started.

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

/// Recognize dependency commands only with native schema provenance; no side effects.
///
/// # Arguments
/// - `command` `[in]`: Semantic command with name and optional schema.
///
/// # Returns
/// `true` for include/find_package/add_subdirectory under Builtin/KeyWord schemas; `false` for other or external commands. User shadows are checked separately.
///
/// # Examples
/// ```text
/// let dependency_command = native(command);
/// ```
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
/// Read the first dependency operand, recovering an unfinished literal when necessary.
/// No I/O or source mutation.
///
/// # Arguments
/// - `snapshot` `[in]`: Matching immutable text and lexer errors.
/// - `command` `[in]`: Command whose first argument or immediate unfinished literal is inspected.
///
/// # Returns
/// `Some((span, text, literal))` gives wrapper-free UTF-8 span, borrowed content and whether static literal resolution is safe; `None` means no usable first operand. Unfinished quoted expansions/escapes are rejected; bracket contents are literal.
///
/// # Algorithm
/// Prefer the parsed first argument. For an unfinished command with no operands, require the next token to be its opening parenthesis and only whitespace before a quote/bracket lexer error. Remove opening delimiters and the bracket's optional initial newline without inventing parsed nodes.
///
/// # Flow
/// ```text
///  [Parsed first operand?] --yes--> [Span/content/literal flag]
///             | no
///             v
///  <unfinished + opening parenthesis?> --no--> [None]
///             | yes
///             v
///  [Immediate quote/bracket lexer error]
///             |
///  <recover safe payload?> --no--> [None]
///             | yes
///             v
///  [Wrapper-free literal operand]
/// ```
///
/// # Examples
/// ```text
/// let operand = first_argument(&snapshot, command);
/// ```
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
/// Select a native dependency operand at the client cursor, preserving variable navigation.
/// No I/O or mutation.
///
/// # Arguments
/// - `snapshot` `[in]`: Text and semantic/lexer data for one retained revision.
/// - `position` `[in]`: Client cursor in encoding units.
/// - `encoding` `[in]`: Negotiated coordinate encoding for byte conversion.
///
/// # Returns
/// `Ok(Some(id))` for a selected first-operand payload; `Ok(None)` for other syntax or a variable name inside ${...}. The unfinished payload end is selectable.
///
/// # Errors
/// `Err` reports an invalid client position, including an encoded-character interior.
///
/// # Algorithm
/// Convert the client position to bytes once. Inspect native commands' first
/// payloads, including recovered unfinished literals. Allow the unfinished end
/// boundary, but yield ordinary navigation for any variable-reference name span.
///
/// # Flow
/// ```text
/// [Client position] ---> [Byte cursor]
///                              |
///                    <first native payload hit?> --no--> [Next command]
///                              | yes
///                              v
///                     <variable name selected?>
///                       /                  \
///                    [None]            [Command ID]
/// ```
///
/// # Examples
/// ```text
/// let selected = selected(&snapshot, &position, encoding)?;
/// ```
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
/// Check static and installed runtime call bindings before treating a command as native.
/// No I/O or mutation.
///
/// # Arguments
/// - `workspace` `[in]`: Owner graph and optional installed runtime layer.
/// - `snapshot` `[in]`: Local command/function references used when no graph exists.
/// - `id` `[in]`: Snapshot-local command ID selected for dependency navigation.
/// - `path` `[in]`: Queried file path qualifying runtime resolved-call records.
///
/// # Returns
/// `true` if graph, file-local fallback or runtime data binds this call to a source function/macro; `false` otherwise.
///
/// # Examples
/// ```text
/// let user_call = shadowed(workspace, &snapshot, command.id, path);
/// ```
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
/// Append a usable dependency URI, preferring an open document even if unsaved.
/// May read filesystem metadata for closed targets; never writes files.
///
/// # Arguments
/// - `service` `[in]`: Open-document map retaining original client URIs.
/// - `path` `[in]`: Resolved dependency candidate path.
/// - `targets` `[in,out]`: Set receiving a deduplicated URI; existing URIs are preserved.
///
/// # Returns
/// `()`: open targets are added; closed regular files are encoded and added; missing, non-file or metadata-error targets are silently omitted.
///
/// # Examples
/// ```text
/// add(service, &resolved_path, &mut targets).await;
/// ```
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
/// Resolve one static dependency candidate in a visit's directory/scope and retain usable URI.
/// May read search-path/filesystem metadata; does not configure or execute CMake.
///
/// # Arguments
/// - `service` `[in]`: Open-document URI map.
/// - `workspace` `[in]`: Owner dependency resolver and settings.
/// - `dependency` `[in]`: Parsed or recovered dependency operand/options.
/// - `caller` `[in]`: Source file containing the dependency command.
/// - `directory` `[in]`: Visit-specific source/binary directory context.
/// - `scope` `[in]`: Owner graph scope for known values, or None for file-only resolution.
/// - `targets` `[in,out]`: Appends a usable resolved URI by set insertion; never clears existing entries.
///
/// # Returns
/// `()`: unresolved/missing candidates and resolver diagnostic details are omitted; usable targets are inserted.
///
/// # Examples
/// ```text
/// resolve(service, workspace, dependency, path, &directory, None, &mut targets).await;
/// ```
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
/// Intercept only native dependency path arguments for file-start definitions.
/// May read filesystem/search metadata and existing runtime observations; never executes CMake or mutates bindings.
///
/// # Arguments
/// - `service` `[in]`: Flushed owner workspaces, open-document URIs and negotiated encoding.
/// - `path` `[in]`: Queried source file path.
/// - `position` `[in]`: Client cursor in the service's negotiated coordinates.
/// - `cancel` `[in]`: Cooperative signal checked across owners, edges, runtime events and before returning; individual metadata awaits are not raced.
///
/// # Returns
/// `Ok(None)` leaves ordinary symbol navigation active (including native names, variable-name spans and user shadows). `Ok(Some(locations))` handles a dependency operand, possibly with no targets; all returned ranges are zero-length at file start and URIs are deduplicated.
///
/// # Errors
/// `Err` reports cancellation, invalid client coordinates or joined selection-worker failure. Missing/unresolvable targets are omitted rather than errors.
///
/// # Algorithm
/// Use a cheap line filter, then join precise byte selection. Reject source-shadowed calls. Prefer qualified graph edges, resolving unresolved edges with caller directory/scope; without edges, replay matching visit contexts or file-only root/script context. Recover missing first literals when needed, supplement installed runtime dependency targets, and retain only existing files or open overlays. Conditional/deferred literal paths are navigation candidates, not proof of execution or symbol bindings.
///
/// # Flow
/// ```text
///  [Owner + native operand cursor]
///                 |
///        [Joined precise selection]
///                 |
///        <user shadow/variable name?> --yes--> [Ordinary navigation]
///                 | no
///                 v
///         <qualified graph edge?>
///           /                 \
///        yes                  no
///         |                    |
///  [Reuse/resolve edge] [Resolve visit/root context]
///         +----------+---------+
///                    v
///       [Supplement installed runtime targets]
///                    |
///         <open overlay or saved file?> --no--> [Omit]
///                    | yes
///                    v
///         [Deduplicated URI at file start]
/// ```
///
/// # Examples
/// ```text
/// let handled = dependencies::query(service, path, &position, &cancel).await?;
/// ```
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
