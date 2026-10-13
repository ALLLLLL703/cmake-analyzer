// AI-generated implementation.
use super::{documents::absolute, model::*};
use crate::parser::{
    build_struct::model::{BuildEffect, Dependency, DependencyKind},
    semanticer::model::SemanticedBuffer,
};
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
};

struct Frame {
    invocation: usize,
    snapshot: Arc<DocumentSnapshot>,
    next: usize,
    local_scopes: HashMap<u32, usize>,
    depth: usize,
}
// Stack order: enter a child, finish it, then continue the caller. See README.md.
enum Task {
    Enter {
        file: PathBuf,
        parent: Option<usize>,
        scope: usize,
        directory: DirectoryContext,
        edge: Option<usize>,
        depth: usize,
    },
    Resume(Frame),
    Exit(PathBuf),
}
// A file's scope 0 is not another file's scope 0. Map IDs anew for each visit.
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
    pub(super) async fn build_graph(&mut self, root: &Root) -> RootGraph {
        let mut graph = RootGraph::default();
        graph.scopes.push(VariableScope {
            parent: None,
            possible_writes: Default::default(),
            project: None,
        });
        let directory = DirectoryContext {
            source: root.file.parent().unwrap_or(Path::new("/")).to_owned(),
            binary: root.binary_directory.clone(),
        };
        let mut tasks = vec![Task::Enter {
            file: root.file.clone(),
            parent: None,
            scope: 0,
            directory,
            edge: None,
            depth: 0,
        }];
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
                            file,
                            source: edge.map(|i| graph.edges[i].source.clone()),
                            message: "Dependency cycle or workspace traversal limit".into(),
                        });
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
                        local_scopes: HashMap::new(),
                        depth,
                    }));
                }
                //太长了不看(or later?)
                Task::Resume(mut frame) => {
                    let Some(fact) = frame.snapshot.build.facts.get(frame.next).cloned() else {
                        continue;
                    };
                    frame.next += 1;
                    let caller = frame.invocation;
                    graph.facts.push((caller, frame.next - 1));
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
                        BuildEffect::Dependency(dependency) => {
                            let invocation = &graph.invocations[caller];
                            let (resolved, candidates) = self
                                .resolve_dependency(
                                    &dependency,
                                    &invocation.file,
                                    &invocation.directory,
                                )
                                .await;
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
    async fn resolve_dependency(
        &self,
        dependency: &Dependency,
        caller: &Path,
        directory: &DirectoryContext,
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
        if input.is_absolute() || input.extension().is_some() || input.components().count() > 1 {
            let path = absolute(&directory.source.join(input)).ok();
            return (path.clone(), path.into_iter().collect());
        }
        let name = format!("{}.cmake", dependency.path.text);
        // A builtin module checks the builtin directory first; other callers check user paths first.
        let caller_builtin = self
            .builtin_module_directory
            .as_ref()
            .is_some_and(|base| caller.parent() == Some(base.as_path()));
        let mut paths = Vec::new();
        if caller_builtin {
            paths.extend(self.builtin_module_directory.iter().cloned());
        }
        paths.extend(self.module_paths.iter().map(|path| {
            if path.is_absolute() {
                path.clone()
            } else {
                directory.source.join(path)
            }
        }));
        if !caller_builtin {
            paths.extend(self.builtin_module_directory.iter().cloned());
        }
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
}
