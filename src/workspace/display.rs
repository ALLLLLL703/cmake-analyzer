// AI-generated implementation.
// review over
use super::model::{
    DependencyEdge, DependencyGraphDisplay, RootGraph, Workspace, WorkspaceDisplay,
};
use crate::parser::build_struct::model::{BuildEffect, Dependency, DependencyKind};
use owo_colors::{OwoColorize, Style};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display, Formatter},
    path::Path,
};

const TITLE: Style = Style::new().bright_magenta().bold();
const HEADING: Style = Style::new().bright_blue().bold();
const ID: Style = Style::new().bright_cyan();
const FILE: Style = Style::new().bright_white().bold();
const INCLUDE: Style = Style::new().green();
const CHILD: Style = Style::new().blue().bold();
const SCOPE: Style = Style::new().magenta();
const WARNING: Style = Style::new().bright_yellow().bold();
const ERROR: Style = Style::new().bright_red().bold();
const MUTED: Style = Style::new().bright_black();

fn paint(f: &mut Formatter<'_>, colored: bool, style: Style, text: impl Display) -> fmt::Result {
    if colored {
        write!(f, "{}", text.style(style))
    } else {
        write!(f, "{text}")
    }
}
impl Workspace {
    pub fn display(&self, colored: bool) -> WorkspaceDisplay<'_> {
        WorkspaceDisplay {
            workspace: self,
            colored,
            DependcyGraphDisplay: DependencyGraphDisplay {
                graph: self.root_graph(),
                root: &self.root,
                colored,
            },
        }
    }
}
impl DependencyGraphDisplay<'_> {
    fn path(&self, f: &mut Formatter<'_>, path: &Path, style: Style) -> fmt::Result {
        let base = self.root.file.parent().unwrap_or(Path::new(""));
        let short = path.strip_prefix(base).unwrap_or(path);
        // Debug formatting keeps control characters inside labels, not in the terminal.
        paint(f, self.colored, style, format_args!("{short:?}"))
    }
    fn visit(&self, f: &mut Formatter<'_>, graph: &RootGraph, id: usize) -> fmt::Result {
        let visit = &graph.invocations[id];
        paint(
            f,
            self.colored,
            ID,
            format_args!(
                "[V{id} F{} r{}]",
                visit.analysis.file, visit.analysis.revision
            ),
        )?;
        write!(f, " ")?;
        paint(f, self.colored, SCOPE, format_args!("S{}", visit.scope))?;
        write!(f, " ")?;
        self.path(f, &visit.file, FILE)
    }
    fn edge(
        &self,
        f: &mut Formatter<'_>,
        graph: &RootGraph,
        edge: &DependencyEdge,
        dependency: Option<&Dependency>,
    ) -> fmt::Result {
        let (name, style) = match edge.kind {
            DependencyKind::Include => ("include", INCLUDE),
            DependencyKind::Subdirectory => ("add_subdirectory", CHILD),
        };
        paint(f, self.colored, style, name)?;
        write!(f, " ")?;
        if let Some(child) = edge.invocation {
            self.visit(f, graph, child)?;
        } else {
            if let Some(path) = &edge.resolved {
                self.path(f, path, FILE)?;
            } else if let Some(dependency) = dependency {
                paint(
                    f,
                    self.colored,
                    WARNING,
                    format_args!("{:?}", dependency.path.text),
                )?;
            } else {
                paint(f, self.colored, WARNING, "?")?;
            }
            let status = if edge.source.conditional || edge.source.deferred {
                "[candidate; not traversed]"
            } else if edge.resolved.is_some() {
                "[not visited]"
            } else if dependency.is_some_and(|d| !d.path.literal) {
                "[unresolved]"
            } else if edge.optional {
                "[optional; absent]"
            } else {
                "[missing]"
            };
            write!(f, " ")?;
            paint(f, self.colored, WARNING, status)?;
        }
        if edge.optional {
            write!(f, " ")?;
            paint(f, self.colored, INCLUDE, "OPTIONAL")?;
        }
        if edge.source.conditional {
            write!(f, " ")?;
            paint(f, self.colored, WARNING, "[conditional]")?;
        }
        if edge.source.deferred {
            write!(f, " ")?;
            paint(f, self.colored, SCOPE, "[deferred]")?;
        }
        write!(f, " ")?;
        paint(
            f,
            self.colored,
            MUTED,
            format_args!(
                "@ {}:{} C{}",
                edge.source.span.row + 1,
                edge.source.span.column + 1,
                edge.source.command.id
            ),
        )?;
        writeln!(f)
    }
    fn tree(&self, f: &mut Formatter<'_>, graph: &RootGraph) -> fmt::Result {
        let mut children = vec![Vec::new(); graph.invocations.len()];
        for (index, edge) in graph.edges.iter().enumerate() {
            children[edge.caller].push(index);
        }
        // Resolve labels once per snapshot; repeated visits still get their own branches.
        let mut seen = HashSet::new();
        let mut dependencies = HashMap::new();
        for visit in &graph.invocations {
            if !seen.insert(visit.analysis) {
                continue;
            }
            for fact in &visit.snapshot.build.facts {
                if let BuildEffect::Dependency(dependency) = &fact.effect {
                    dependencies.insert((fact.source.analysis, fact.source.command), dependency);
                }
            }
        }
        if graph.invocations.is_empty() {
            paint(f, self.colored, WARNING, "(root was not loaded)")?;
            return writeln!(f);
        }
        self.visit(f, graph, 0)?;
        writeln!(f)?;
        // A frame is (visit, next edge, last sibling). Iteration avoids recursive printing.
        let mut stack = vec![(0usize, 0usize, true)];
        while let Some(&(visit, next, _)) = stack.last() {
            let Some(&edge_index) = children[visit].get(next) else {
                stack.pop();
                continue;
            };
            let last = next + 1 == children[visit].len();
            for &(_, _, ancestor_last) in stack.iter().skip(1) {
                paint(
                    f,
                    self.colored,
                    MUTED,
                    if ancestor_last { "    " } else { "|   " },
                )?;
            }
            paint(f, self.colored, MUTED, if last { "`-- " } else { "+-- " })?;
            let edge = &graph.edges[edge_index];
            self.edge(
                f,
                graph,
                edge,
                dependencies
                    .get(&(edge.source.analysis, edge.source.command))
                    .copied(),
            )?;
            if let Some(frame) = stack.last_mut() {
                frame.1 += 1;
            }
            if let Some(child) = edge.invocation {
                stack.push((child, 0, last));
            }
        }
        Ok(())
    }
    fn diagnostics(&self, f: &mut Formatter<'_>, graph: &RootGraph) -> fmt::Result {
        writeln!(f)?;
        paint(f, self.colored, HEADING, "DIAGNOSTICS")?;
        writeln!(f)?;
        let callers: HashMap<_, _> = graph
            .invocations
            .iter()
            .map(|visit| (visit.analysis, visit.file.as_path()))
            .collect();
        let mut count = 0;
        for diagnostic in &graph.diagnostics {
            count += 1;
            write!(f, "  ")?;
            paint(f, self.colored, ERROR, "WORKSPACE")?;
            write!(f, " ")?;
            self.path(f, &diagnostic.file, FILE)?;
            if let Some(source) = &diagnostic.source {
                write!(f, " <- ")?;
                if let Some(caller) = callers.get(&source.analysis) {
                    self.path(f, caller, FILE)?;
                }
                write!(f, " ")?;
                paint(
                    f,
                    self.colored,
                    MUTED,
                    format_args!(
                        "@ {}:{} C{}",
                        source.span.row + 1,
                        source.span.column + 1,
                        source.command.id
                    ),
                )?;
            }
            write!(f, " ")?;
            paint(
                f,
                self.colored,
                ERROR,
                format_args!("{:?}", diagnostic.message),
            )?;
            writeln!(f)?;
        }
        let mut seen = HashSet::new();
        for visit in &graph.invocations {
            if !seen.insert(visit.analysis) {
                continue;
            }
            for diagnostic in &visit.snapshot.semantics.diagnostics {
                count += 1;
                write!(f, "  ")?;
                paint(f, self.colored, ERROR, "SEMANTIC")?;
                write!(f, " ")?;
                self.path(f, &visit.file, FILE)?;
                write!(f, " ")?;
                paint(
                    f,
                    self.colored,
                    MUTED,
                    format_args!(
                        "@ {}:{}",
                        diagnostic.span.row + 1,
                        diagnostic.span.column + 1
                    ),
                )?;
                write!(f, " ")?;
                paint(
                    f,
                    self.colored,
                    ERROR,
                    format_args!("{:?}", diagnostic.content),
                )?;
                writeln!(f)?;
            }
            for diagnostic in &visit.snapshot.build.diagnostics {
                count += 1;
                write!(f, "  ")?;
                paint(f, self.colored, ERROR, "BUILD")?;
                write!(f, " ")?;
                self.path(f, &visit.file, FILE)?;
                write!(f, " ")?;
                paint(
                    f,
                    self.colored,
                    MUTED,
                    format_args!(
                        "@ {}:{}",
                        diagnostic.source.span.row + 1,
                        diagnostic.source.span.column + 1
                    ),
                )?;
                write!(f, " ")?;
                paint(
                    f,
                    self.colored,
                    ERROR,
                    format_args!("{:?}", diagnostic.message),
                )?;
                writeln!(f)?;
            }
        }
        if count == 0 {
            paint(f, self.colored, INCLUDE, "  No diagnostics")?;
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Display for DependencyGraphDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        paint(f, self.colored, HEADING, "DEPENDENCY GRAPH")?;
        writeln!(f)?;
        let Some(graph) = self.graph else {
            paint(
                f,
                self.colored,
                WARNING,
                "Pending or stale; call flush() before displaying the graph.",
            )?;
            return writeln!(f);
        };
        self.tree(f, graph)?;
        writeln!(f)?;
        paint(f, self.colored, HEADING, "SCOPES")?;
        writeln!(f)?;
        for (index, scope) in graph.scopes.iter().enumerate() {
            write!(f, "  ")?;
            paint(f, self.colored, SCOPE, format_args!("S{index}"))?;
            if let Some(parent) = scope.parent {
                write!(f, " <- ")?;
                paint(f, self.colored, SCOPE, format_args!("S{parent}"))?;
            }
            if let Some(project) = &scope.project {
                write!(f, " project=")?;
                paint(f, self.colored, FILE, format_args!("{project:?}"))?;
            }
            write!(f, " writes=[")?;
            for (index, name) in scope.possible_writes.iter().enumerate() {
                if index > 0 {
                    write!(f, ", ")?;
                }
                paint(f, self.colored, INCLUDE, format_args!("{name:?}"))?;
            }
            writeln!(f, "]")?;
        }
        self.diagnostics(f, graph)
    }
}
impl Display for WorkspaceDisplay<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        paint(f, self.colored, TITLE, "CMAKE WORKSPACE")?;
        writeln!(f)?;
        write!(f, "root=")?;
        paint(
            f,
            self.colored,
            FILE,
            format_args!("{:?}", self.workspace.root.file),
        )?;
        writeln!(f)?;
        if let Some(binary) = &self.workspace.root.binary_directory {
            write!(f, "binary=")?;
            paint(f, self.colored, CHILD, format_args!("{binary:?}"))?;
            writeln!(f)?;
        }
        writeln!(
            f,
            "documents={} overlays={} pending={}",
            self.workspace.documents.len(),
            self.workspace
                .documents
                .values()
                .filter(|d| d.overlay.is_some())
                .count(),
            self.workspace.dirty.len()
        )?;
        if let Some(graph) = self.DependcyGraphDisplay.graph {
            writeln!(
                f,
                "visits={} edges={} scopes={} facts={}",
                graph.invocations.len(),
                graph.edges.len(),
                graph.scopes.len(),
                graph.facts.len()
            )?;
        }
        writeln!(
            f,
            "Static traversal, not a configured build. V=visit F=file S=scope.\n"
        )?;
        Display::fmt(&self.DependcyGraphDisplay, f)
    }
}
