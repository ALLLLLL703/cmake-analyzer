//! Iteratively visit dependencies in source order with conservative values and bounded work.
//! Includes/packages share caller scope and directory; subdirectories fork both contexts.
//! Reads files and search metadata but never configures or executes project code.
// Contains AI-generated documentation additions.
// AI-generated implementation.
// review over(except for the build_graph)
mod values;
use super::{documents::absolute, model::*};
use crate::parser::semanticer::model::{PossibleValue, ValueBindings, ValueDirectories};
use crate::parser::{
    build_struct::model::{BuildEffect, Dependency, DependencyKind},
    semanticer::model::SemanticedBuffer,
};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
};

/// Suspended command-order replay state for one dependency visit. No I/O on construction.
///
/// [`Task::Resume`] owns this state while children run; [`RootGraph`] holds the accumulated
/// effects.
///
/// # Flow
/// ```text
/// [caller frame]             [child task]
///        |                        |
///   [suspended] <--- stack ---- [Enter]
///        |                        |
///        +---- [Resume after child]+
/// ```
///
/// # Examples
/// ```text
/// let next_fact = frame.next;
/// ```
struct Frame {
    /// Graph visit being resumed Example: `frame.invocation`.
    invocation: usize,
    /// Immutable file result read by this visit Example: `frame.snapshot.build.facts.len()`.
    snapshot: Arc<DocumentSnapshot>,
    /// Next build-fact index, advanced before visiting a dependency Example: `frame.next`.
    next: usize,
    /// Next semantic command whose value effects have not been replayed Example:
    /// `frame.next_command` .
    next_command: usize,
    /// Directory builtin context used for value expansion Example:
    /// `frame.directories.file.as_path()` .
    directories: ValueDirectories,
    /// File-local scope ID to graph-scope index for this visit only Example:
    /// `frame.local_scopes.get(&0)` .
    local_scopes: HashMap<u32, usize>,
    /// Current dependency depth, root zero Example: `frame.depth`.
    depth: usize,
}
// Stack order: enter a child, finish it, then continue the caller. See README.md.
/// Explicit depth-first traversal step; avoids recursive async graph walking. Constructing it has
/// no side effects.
///
/// [`Frame`] retains suspended caller state; [`Workspace::build_graph`] consumes this stack.
///
/// # Flow
/// ```text
/// [Enter] ----> [Resume]
///     |             |
///  [active set]  <dependency?>
///                   /      \
///             [child Enter] [next fact]
///                   |          |
///                   +----> [Exit]
///                             |
///                        [remove active]
/// ```
///
/// # Examples
/// ```text
/// tasks.push(Task::Exit(file));
/// ```
enum Task {
    /// Attempt a bounded file visit, resolve external policy, then load a snapshot Example:
    /// `Task::Enter { file, parent: None, scope: 0, directory, edge: None, depth: 0 }` .
    Enter {
        /// File identity to load. Example:
        /// `Task::Enter { file, parent: None, scope: 0, directory, edge: None, depth: 0 }` .
        file: PathBuf,
        /// Caller visit or None for root. Example:
        /// `Task::Enter { parent: None, file, scope: 0, directory, edge: None, depth: 0 }` .
        parent: Option<usize>,
        /// Graph scope inherited or forked by the caller. Example:
        /// `Task::Enter { scope: 0, file, parent: None, directory, edge: None, depth: 0 }` .
        scope: usize,
        /// Source/binary context for the entered file. Example:
        /// `Task::Enter { directory, file, parent: None, scope: 0, edge: None, depth: 0 }` .
        directory: DirectoryContext,
        /// Origin edge index updated after successful entry. Example:
        /// `Task::Enter { edge: None, file, parent: None, scope: 0, directory, depth: 0 }` .
        edge: Option<usize>,
        /// Dependency nesting count with root zero. Example:
        /// `Task::Enter { depth: 0, file, parent: None, scope: 0, directory, edge: None }` .
        depth: usize,
    },
    /// Continue value/fact replay after a child completes Example: `Task::Resume(frame)`.
    Resume(/** Suspended caller state. Example: `Task::Resume(frame)`. */ Frame),
    /// Remove the file from the active call path after all children finish Example:
    /// `Task::Exit(file)` .
    Exit(/** File identity to remove from the active set. Example: `Task::Exit(file)`. */ PathBuf),
}
// A file's scope 0 is not another file's scope 0. Map IDs anew for each visit.
/// Map a file-local scope into this visit's graph scopes, copying parent state at first use. No
/// I/O.
///
/// # Arguments
/// - `local` `[in]`: Optional semantic scope number; None selects the visit scope.
/// - `invocation` `[in]`: Existing graph visit index.
/// - `semantics` `[in]`: File-local scope ancestry for this snapshot.
/// - `map` `[in,out]`: Per-visit mapping; appends each newly created scope ID.
/// - `graph` `[in,out]`: Appends cloned child scopes; existing scope values are not overwritten.
///
/// # Returns
/// Graph scope index for this local context.
///
/// # Panics
/// Panics for invalid invocation or graph-parent scope indices; callers use IDs from the graph
/// under construction.
///
/// # Algorithm
/// Return the visit base for no local scope, or reuse an existing mapping. Otherwise map the
/// semantic parent first, clone its current graph state, record parent ancestry and allocate one
/// graph scope for this visit.
///
/// # Flow
/// ```text
/// [local scope]
///       |
///  <already mapped?> -- yes --> [existing graph ID]
///       | no
///  [map semantic parent]
///       |
///  [clone current parent state]
///       /                \
///  [append graph scope] [record visit mapping]
///       \                /
///          [new graph ID]
/// ```
///
/// # Examples
/// ```text
/// let scope = local_scope(Some(0), visit_id, semantics, &mut mapping, &mut graph);
/// ```
fn local_scope(
    local: Option<u32>,
    invocation: usize,
    semantics: &SemanticedBuffer,
    map: &mut HashMap<u32, usize>,
    graph: &mut RootGraph,
) -> usize {
    let Some(local) = local else {
        return graph.invocations[invocation].scope;
    };
    if let Some(scope) = map.get(&local) {
        return *scope;
    }
    let parent = local_scope(
        semantics
            .scopes
            .get(local as usize)
            .and_then(|s| s.parent)
            .map(|p| p.id),
        invocation,
        semantics,
        map,
        graph,
    );
    let id = graph.scopes.len();
    let mut scope = graph.scopes[parent].clone();
    scope.parent = Some(parent);
    graph.scopes.push(scope);
    map.insert(local, id);
    id
}
impl Workspace {
    /// Build an ordered, bounded static dependency graph from one entry. Reads sources/search
    /// metadata but never executes project code; per-file analysis may inspect CMake version.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Workspace document caches may be loaded/updated; the returned graph is
    ///   not installed by this method.
    /// - `root` `[in]`: Entry file and source/binary/script context.
    ///
    /// # Returns
    /// New graph, possibly partial, with read/cycle/limit diagnostics rather than a returned I/O
    /// error.
    ///
    /// # Algorithm
    /// Use Enter/Resume/Exit tasks so each resolved child completes before the caller's next fact.
    /// Track active lexical paths for cycles and loaded snapshots for unique-file limits; canonical
    /// saved paths select external policy. Functions-only descendants keep that mode; none skips
    /// loading and invalidates values. Full visits advance conservative values, retain
    /// conditional/deferred candidate edges without visiting them, expand only known single-value
    /// paths, share include/package scopes, and fork subdirectory state.
    ///
    /// # Flow
    /// ```text
    /// [Enter file] ----> <cycle/budget?> ---- yes --> [diagnostic]
    ///       | no
    ///  <external policy?>
    ///    /        |        \
    ///  none    functions    full
    ///   |         |          |
    /// [invalidate] |     [advance values]
    ///              +-----[next dependency]
    ///                          |
    ///                     <resolvable?>
    ///                      /         \
    ///               [child Enter] [resume caller]
    ///                      |             |
    ///                      +----> [Exit: remove active]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let graph = workspace.build_graph(&root).await;
    /// ```
    pub(super) async fn build_graph(&mut self, root: &Root) -> RootGraph {
        let mut graph = RootGraph::default();
        graph.scopes.push(VariableScope {
            parent: None,
            possible_writes: Default::default(),
            project: None,
            values: ValueBindings::default(),
        });
        let directory = DirectoryContext {
            source: root
                .script_working_directory
                .clone()
                .unwrap_or_else(|| root.file.parent().unwrap_or(Path::new("/")).to_owned()),
            binary: root
                .script_working_directory
                .clone()
                .or_else(|| root.binary_directory.clone()),
        };
        graph.project_defaults = root.script_working_directory.is_none()
            && root
                .file
                .file_name()
                .is_some_and(|name| name == "CMakeLists.txt");
        if graph.project_defaults {
            graph.scopes[0].values.assign(
                "PROJECT_SOURCE_DIR",
                directory
                    .source
                    .to_str()
                    .map(|path| PossibleValue::Static(path.into())),
            );
        }
        let mut tasks = vec![Task::Enter {
            file: root.file.clone(),
            parent: None,
            scope: 0,
            directory,
            edge: None,
            depth: 0,
        }];
        let source_root = root.file.parent().unwrap_or(Path::new("/")).to_owned();
        let source_root = tokio::fs::canonicalize(&source_root)
            .await
            .unwrap_or(source_root);
        let mut attempts = 0;
        // A -> B -> A is a loop. A -> B, then A -> B again is allowed.
        let mut active = HashSet::new();
        let mut loaded: HashMap<PathBuf, Arc<DocumentSnapshot>> = HashMap::new();
        while let Some(task) = tasks.pop() {
            match task {
                Task::Exit(path) => {
                    active.remove(&path);
                }
                // find invocations
                Task::Enter {
                    file,
                    parent,
                    scope,
                    directory,
                    edge,
                    depth,
                } => {
                    let limited = attempts >= self.limits.invocations;
                    attempts += 1;
                    if active.contains(&file)
                        || depth > self.limits.depth
                        || limited
                        || (!loaded.contains_key(&file) && loaded.len() >= self.limits.files)
                    {
                        graph.diagnostics.push(WorkspaceDiagnostic {
                            kind: super::model::WorkspaceDiagnosticKind::Traversal,
                            file,
                            source: edge.map(|i| graph.edges[i].source.clone()),
                            message: "Dependency cycle or workspace traversal limit".into(),
                        });
                        continue;
                    }
                    let physical = tokio::fs::canonicalize(&file)
                        .await
                        .unwrap_or_else(|_| file.clone());
                    let index = if parent.is_some_and(|parent| {
                        graph.invocations[parent].index == ExternalIndex::Functions
                    }) {
                        ExternalIndex::Functions
                    } else if edge.is_some() && !physical.starts_with(&source_root) {
                        match edge.map(|edge| graph.edges[edge].kind) {
                            Some(DependencyKind::Package) => self.external_find_package,
                            Some(DependencyKind::Include) => self.external_include,
                            _ => ExternalIndex::Full,
                        }
                    } else {
                        ExternalIndex::Full
                    };
                    if index == ExternalIndex::None {
                        graph.scopes[scope].values.invalidate();
                        continue;
                    }
                    let snapshot = match loaded.get(&file) {
                        Some(snapshot) => snapshot.clone(),
                        None => match self.ensure_snapshot(&file).await {
                            Ok(snapshot) => {
                                loaded.insert(file.clone(), snapshot.clone());
                                snapshot
                            }
                            Err(error) => {
                                if error.kind() == std::io::ErrorKind::NotFound
                                    && edge.is_some_and(|i| graph.edges[i].optional)
                                {
                                    continue;
                                }
                                graph.diagnostics.push(WorkspaceDiagnostic {
                                    kind: super::model::WorkspaceDiagnosticKind::ReadFailure,
                                    file,
                                    source: edge.map(|i| graph.edges[i].source.clone()),
                                    message: error.to_string(),
                                });
                                continue;
                            }
                        },
                    };
                    let invocation = graph.invocations.len();
                    if let Some(edge) = edge {
                        graph.edges[edge].invocation = Some(invocation);
                    }
                    graph.invocations.push(Invocation {
                        index,
                        snapshot: snapshot.clone(),
                        file: file.clone(),
                        analysis: snapshot.build.analysis,
                        parent,
                        scope,
                        directory,
                    });
                    active.insert(file.clone());
                    // Run Resume before Exit; keep the file on the call path until its children finish.
                    tasks.push(Task::Exit(file));
                    tasks.push(Task::Resume(Frame {
                        invocation,
                        snapshot,
                        next: 0,
                        next_command: 0,
                        directories: ValueDirectories {
                            source_root: root.script_working_directory.clone().unwrap_or_else(
                                || root.file.parent().unwrap_or(Path::new("/")).to_owned(),
                            ),
                            source: graph.invocations[invocation].directory.source.clone(),
                            binary_root: root
                                .script_working_directory
                                .clone()
                                .or_else(|| root.binary_directory.clone()),
                            binary: graph.invocations[invocation].directory.binary.clone(),
                            file: graph.invocations[invocation].file.clone(),
                        },
                        local_scopes: HashMap::new(),
                        depth,
                    }));
                }
                //太长了不看(or later?)
                Task::Resume(mut frame) => {
                    let Some(fact) = frame.snapshot.build.facts.get(frame.next).cloned() else {
                        let last = frame.snapshot.semantics.commands.len().saturating_sub(1);
                        if graph.invocations[frame.invocation].index == ExternalIndex::Full {
                            values::advance(&mut frame, &mut graph, last);
                        }
                        continue;
                    };
                    let functions_only =
                        graph.invocations[frame.invocation].index == ExternalIndex::Functions;
                    if functions_only && !matches!(fact.effect, BuildEffect::Dependency(_)) {
                        frame.next += 1;
                        tasks.push(Task::Resume(frame));
                        continue;
                    }
                    if !functions_only {
                        values::advance(&mut frame, &mut graph, fact.source.command.id as usize);
                    }
                    frame.next += 1;
                    let caller = frame.invocation;
                    if !functions_only {
                        graph.facts.push((caller, frame.next - 1));
                    }
                    // Keep the facts, but do not assume a branch or function body actually runs.
                    if fact.source.conditional || fact.source.deferred {
                        if let BuildEffect::Dependency(dependency) = &fact.effect {
                            graph.edges.push(DependencyEdge {
                                caller,
                                source: fact.source,
                                kind: dependency.kind,
                                optional: dependency.optional,
                                candidates: Vec::new(),
                                resolved: None,
                                invocation: None,
                            });
                        }
                        tasks.push(Task::Resume(frame));
                        continue;
                    }
                    let scope = local_scope(
                        fact.source.scope.map(|s| s.id),
                        caller,
                        &frame.snapshot.semantics,
                        &mut frame.local_scopes,
                        &mut graph,
                    );
                    match fact.effect {
                        BuildEffect::Project(project) if project.name.literal => {
                            graph.scopes[scope].project = Some(project.name.text);
                        }
                        BuildEffect::VariableWrites { names, local: true } => {
                            Arc::make_mut(&mut graph.scopes[scope].possible_writes).extend(names);
                        }
                        BuildEffect::Dependency(mut dependency) => {
                            if !dependency.path.literal {
                                if let Some(arg) = frame.snapshot.semantics.commands
                                    [fact.source.command.id as usize]
                                    .args
                                    .first()
                                {
                                    if let Some(value) = graph.scopes[scope]
                                        .values
                                        .expand(arg, Some(&frame.directories))
                                        .filter(|value| !value.text().contains(';'))
                                    {
                                        dependency.path.text = value.text().to_owned();
                                        dependency.path.literal = true;
                                    }
                                }
                            }
                            if dependency.kind == DependencyKind::Package {
                                let command = &frame.snapshot.semantics.commands
                                    [fact.source.command.id as usize];
                                for (argument, source) in dependency
                                    .arguments
                                    .iter_mut()
                                    .zip(command.args.iter().skip(1))
                                {
                                    if !argument.literal {
                                        if let Some(value) = graph.scopes[scope]
                                            .values
                                            .expand(source, Some(&frame.directories))
                                            .filter(|value| !value.text().contains(';'))
                                        {
                                            argument.text = value.text().to_owned();
                                            argument.literal = true;
                                        }
                                    }
                                }
                            }
                            let invocation = &graph.invocations[caller];
                            let (resolved, candidates) = self
                                .resolve_dependency_in(
                                    &dependency,
                                    &invocation.file,
                                    &invocation.directory,
                                    Some(&graph.scopes[scope].values),
                                )
                                .await;
                            if resolved.is_none()
                                && matches!(
                                    dependency.kind,
                                    DependencyKind::Include | DependencyKind::Package
                                )
                            {
                                graph.scopes[scope].values.invalidate();
                            }
                            let edge = graph.edges.len();
                            graph.edges.push(DependencyEdge {
                                caller,
                                source: fact.source.clone(),
                                kind: dependency.kind,
                                optional: dependency.optional,
                                candidates,
                                resolved: resolved.clone(),
                                invocation: None,
                            });
                            if let Some(file) = resolved {
                                let (child_scope, directory) = if dependency.kind
                                    == DependencyKind::Subdirectory
                                {
                                    // A child starts with the parent's current names; its changes stay in the child.
                                    let id = graph.scopes.len();
                                    let mut child = graph.scopes[scope].clone();
                                    child.parent = Some(scope);
                                    graph.scopes.push(child);
                                    let source = file.parent().unwrap_or(Path::new("/")).to_owned();
                                    // Unknown build paths stay unknown; do not invent a directory.
                                    let binary = match dependency.binary.as_ref() {
                                        Some(arg) if !arg.literal => None,
                                        Some(arg) if Path::new(&arg.text).is_absolute() => {
                                            absolute(Path::new(&arg.text)).ok()
                                        }
                                        Some(arg) => graph.invocations[caller]
                                            .directory
                                            .binary
                                            .as_ref()
                                            .and_then(|base| absolute(&base.join(&arg.text)).ok()),
                                        None => source
                                            .strip_prefix(
                                                &graph.invocations[caller].directory.source,
                                            )
                                            .ok()
                                            .and_then(|relative| {
                                                graph.invocations[caller]
                                                    .directory
                                                    .binary
                                                    .as_ref()
                                                    .and_then(|base| {
                                                        absolute(&base.join(relative)).ok()
                                                    })
                                            }),
                                    };
                                    (id, DirectoryContext { source, binary })
                                } else {
                                    (scope, graph.invocations[caller].directory.clone())
                                };
                                let depth = frame.depth + 1;
                                // Read the child first, then return to the caller's next command.
                                tasks.push(Task::Resume(frame));
                                tasks.push(Task::Enter {
                                    file,
                                    parent: Some(caller),
                                    scope: child_scope,
                                    directory,
                                    edge: Some(edge),
                                    depth,
                                });
                                continue;
                            } else if dependency.path.literal && !dependency.optional {
                                graph.diagnostics.push(WorkspaceDiagnostic {
                                    kind: super::model::WorkspaceDiagnosticKind::ReadFailure,
                                    file: graph.invocations[caller].file.clone(),
                                    source: Some(fact.source),
                                    message: format!(
                                        "Cannot resolve dependency {:?}",
                                        dependency.path.text
                                    ),
                                });
                            }
                        }
                        _ => {}
                    }
                    tasks.push(Task::Resume(frame));
                }
            }
        }
        graph
    }
    // Return the chosen file and checked paths, including missing earlier choices.
    /// Resolve using Workspace module precedence; direct file candidates may not exist yet.
    /// Resolve a navigation dependency using an optional saved graph scope. Probes filesystem
    /// metadata but does not load/execute the chosen script.
    ///
    /// # Arguments
    /// - `self` `[in]`: Workspace search configuration and overlay identities; unchanged.
    /// - `dependency` `[in]` : Request with a literal first operand; dynamic requests decline
    ///   lookup.
    /// - `caller` `[in]`: List-file identity used to select builtin-module precedence.
    /// - `directory` `[in]`: Current source directory and optional binary context.
    /// - `scope` `[in]` : Optional graph-scope index supplying known values; invalid/absent means
    ///   no bindings.
    ///
    /// # Returns
    /// (chosen path, checked candidate paths). Chosen direct paths may not exist; unresolved lookup
    /// returns None. Candidates include missing higher-priority choices.
    ///
    /// # Examples
    /// ```text
    /// let (target, checked) = workspace.resolve_dependency_at(dependency, caller, directory, Some(scope)).await;
    /// ```
    pub(crate) async fn resolve_dependency_at(
        &self,
        dependency: &Dependency,
        caller: &Path,
        directory: &DirectoryContext,
        scope: Option<usize>,
    ) -> (Option<PathBuf>, Vec<PathBuf>) {
        self.resolve_dependency_in(
            dependency,
            caller,
            directory,
            scope
                .and_then(|scope| self.graph.as_ref()?.scopes.get(scope))
                .map(|scope| &scope.values),
        )
        .await
    }
    /// Dispatch literal dependency lookup with explicit contextual values. Probes files/environment
    /// only; no source loading, graph mutation or execution.
    ///
    /// # Arguments
    /// - `self` `[in]`: Workspace search configuration and overlay identities; unchanged.
    /// - `dependency` `[in]` : Request with a literal first operand; dynamic requests decline
    ///   lookup.
    /// - `caller` `[in]`: List-file identity used to select builtin-module precedence.
    /// - `directory` `[in]`: Current source directory and optional binary context.
    /// - `values` `[in]` : Optional conservative search hints such as CMAKE_MODULE_PATH and package
    ///   roots.
    ///
    /// # Returns
    /// (chosen path, candidate paths); nonliteral requests or disabled package lookup return (None,
    /// empty). Direct file candidates may be missing.
    ///
    /// # Examples
    /// ```text
    /// let (target, candidates) = workspace.resolve_dependency_in(dependency, caller, directory, Some(values)).await;
    /// ```
    pub(crate) async fn resolve_dependency_in(
        &self,
        dependency: &Dependency,
        caller: &Path,
        directory: &DirectoryContext,
        values: Option<&ValueBindings>,
    ) -> (Option<PathBuf>, Vec<PathBuf>) {
        if !dependency.path.literal {
            return (None, Vec::new());
        }
        if dependency.kind == DependencyKind::Package {
            if !self.module_lookup_enabled {
                return (None, Vec::new());
            }
            self.resolve_package(dependency, caller, directory, values)
                .await
        } else {
            self.resolve_module(dependency, caller, directory, values)
                .await
        }
    }
    /// Resolve source-directory paths or bare modules with caller-sensitive precedence. Reads
    /// metadata/overlay identities, not file contents; no execution.
    ///
    /// # Arguments
    /// - `self` `[in]`: Workspace search configuration and overlay identities; unchanged.
    /// - `dependency` `[in]` : Request with a literal first operand; dynamic requests decline
    ///   lookup.
    /// - `caller` `[in]`: List-file identity used to select builtin-module precedence.
    /// - `directory` `[in]`: Current source directory and optional binary context.
    /// - `values` `[in]`: Known CMAKE_MODULE_PATH list, when available.
    ///
    /// # Returns
    /// (chosen path, checked paths). Subdirectories/direct paths are returned without existence
    /// checks; bare modules choose the first overlay or existing file, otherwise None with all
    /// checked candidates.
    ///
    /// # Algorithm
    /// Append CMakeLists.txt for subdirectories. Treat absolute, extension-bearing or
    /// multi-component include paths as direct source-relative paths. For bare names, append .cmake
    /// and probe ordered search roots, preserving missing earlier candidates. Disabled module
    /// lookup blocks only bare-name search.
    ///
    /// # Flow
    /// ```text
    /// [literal operand]
    ///        |
    ///  <subdirectory/direct path?>
    ///       /                \
    ///  [normalize only]   <lookup enabled?>
    ///       |               /         \
    ///  [candidate path]    no       [search roots]
    ///                       |            |
    ///                    [none]     [first file/overlay]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let (target, checked) = workspace.resolve_module(dependency, caller, directory, None).await;
    /// ```
    pub(super) async fn resolve_module(
        &self,
        dependency: &Dependency,
        caller: &Path,
        directory: &DirectoryContext,
        values: Option<&ValueBindings>,
    ) -> (Option<PathBuf>, Vec<PathBuf>) {
        if !dependency.path.literal {
            return (None, Vec::new());
        }
        let input = Path::new(&dependency.path.text);
        if dependency.kind == DependencyKind::Subdirectory {
            // Remember missing paths too, so creating the file can rebuild its root.
            let path = absolute(&directory.source.join(input).join("CMakeLists.txt")).ok();
            return (path.clone(), path.into_iter().collect());
        }
        if dependency.kind != DependencyKind::Package
            && (input.is_absolute()
                || input.extension().is_some()
                || input.components().count() > 1)
        {
            let path = absolute(&directory.source.join(input)).ok();
            return (path.clone(), path.into_iter().collect());
        }
        if !self.module_lookup_enabled {
            return (None, Vec::new());
        }
        let name = format!("{}.cmake", dependency.path.text);
        let paths = self.module_search_paths(caller, directory, values);
        let mut candidates = Vec::new();
        for base in paths {
            let Ok(path) = absolute(&base.join(&name)) else {
                continue;
            };
            candidates.push(path.clone());
            if self
                .documents
                .get(&path)
                .is_some_and(|d| d.overlay.is_some())
                || tokio::fs::metadata(&path)
                    .await
                    .is_ok_and(|meta| meta.is_file())
            {
                return (Some(path), candidates);
            }
        }
        (None, candidates)
    }
    /// Assemble module roots without reading directories. Builtin callers search builtin first;
    /// others search it last. No mutation occurs.
    ///
    /// # Arguments
    /// - `self` `[in]`: Configured explicit and builtin module directories.
    /// - `caller` `[in]`: Caller identity; exact parent equality identifies a builtin caller.
    /// - `directory` `[in]`: Source directory used to join relative module paths.
    /// - `values` `[in]`: Optional known semicolon-separated CMAKE_MODULE_PATH.
    ///
    /// # Returns
    /// Ordered roots: builtin first for builtin caller, known variable paths, configured paths,
    /// then builtin for other callers. Duplicates are retained.
    ///
    /// # Algorithm
    /// Determine builtin precedence by exact caller-parent equality. Insert known CMAKE_MODULE_PATH
    /// entries before configured module paths, joining relative entries to the visit source. Put
    /// the supplied builtin directory first for builtin callers and last for others.
    ///
    /// # Flow
    /// ```text
    /// [caller parent]        [builtin directory]
    ///         |                      |
    ///         +----> <same directory?>
    ///                  /         \
    ///            builtin first   defer builtin
    ///                  |         |
    ///                  +----+----+
    ///                       |
    ///       [known CMAKE_MODULE_PATH entries]
    ///                       |
    ///         [configured module paths]
    ///                       |
    ///           [builtin last if deferred]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let roots = workspace.module_search_paths(caller, directory, Some(values));
    /// ```
    pub(super) fn module_search_paths(
        &self,
        caller: &Path,
        directory: &DirectoryContext,
        values: Option<&ValueBindings>,
    ) -> Vec<PathBuf> {
        let builtin = self
            .builtin_module_directory
            .as_ref()
            .is_some_and(|base| caller.parent() == Some(base.as_path()));
        let mut paths = Vec::new();
        if builtin {
            paths.extend(self.builtin_module_directory.iter().cloned());
        }
        if let Some(value) = values.and_then(|values| values.get("CMAKE_MODULE_PATH", None)) {
            paths.extend(
                value
                    .text()
                    .split(';')
                    .filter(|path| !path.is_empty())
                    .map(|path| directory.source.join(path)),
            );
        }
        paths.extend(
            self.module_paths
                .iter()
                .map(|path| directory.source.join(path)),
        );
        if !builtin {
            paths.extend(self.builtin_module_directory.iter().cloned());
        }
        paths
    }
}
