//! Stream escaped workspace summaries, visit trees, scopes and diagnostics to a formatter.
//! Iterative rendering preserves repeated visits and never loads dependencies or executes CMake.
// Contains AI-generated documentation additions.
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

/// Report title style; ignored when color output is disabled.
const TITLE: Style = Style::new().bright_magenta().bold();
/// Section heading style; ignored when color output is disabled.
const HEADING: Style = Style::new().bright_blue().bold();
/// File, command and visit identifiers style; ignored when color output is disabled.
const ID: Style = Style::new().bright_cyan();
/// Escaped file paths style; ignored when color output is disabled.
const FILE: Style = Style::new().bright_white().bold();
/// Include/package and successful-state labels style; ignored when color output is disabled.
const INCLUDE: Style = Style::new().green();
/// Child-directory and binary-directory labels style; ignored when color output is disabled.
const CHILD: Style = Style::new().blue().bold();
/// Scope and deferred-context labels style; ignored when color output is disabled.
const SCOPE: Style = Style::new().magenta();
/// Unresolved, absent and conditional graph labels style; ignored when color output is disabled.
const WARNING: Style = Style::new().bright_yellow().bold();
/// Error labels style; ignored when color output is disabled.
const ERROR: Style = Style::new().bright_red().bold();
/// Coordinates and tree guides style; ignored when color output is disabled.
const MUTED: Style = Style::new().bright_black();

/// Append a styled or plain display value. Writes only to the supplied formatter, not files or
/// protocol streams directly.
///
/// # Arguments
/// - `f` `[in,out]`: Formatter; appends without clearing previous text.
/// - `colored` `[in]`: Whether terminal styles are emitted.
/// - `style` `[in]`: Style used only when colored is true.
/// - `text` `[in]`: Display value to append.
///
/// # Returns
/// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
///
/// # Errors
/// Returns fmt::Error if writing any fragment to the formatter fails; already appended output is
/// not rolled back.
///
/// # Examples
/// ```text
/// paint(f, false, TITLE, "CMAKE WORKSPACE")?;
/// ```
fn paint(f: &mut Formatter<'_>, colored: bool, style: Style, text: impl Display) -> fmt::Result {
    if colored {
        write!(f, "{}", text.style(style))
    } else {
        write!(f, "{text}")
    }
}
impl Workspace {
    /// Create a borrowed workspace overview and graph-only adapter. No output, parsing or I/O
    /// occurs until the caller formats the view.
    ///
    /// # Arguments
    /// - `self` `[in]` : Workspace borrowed for statistics and a graph passing dirty/revision
    ///   checks.
    /// - `colored` `[in]`: Whether later rendering uses terminal styles.
    ///
    /// # Returns
    /// WorkspaceDisplay; its public DependcyGraphDisplay field can be rendered separately.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// fn report(workspace: &Workspace) -> String {
    ///     workspace.display(false).DependcyGraphDisplay.to_string()
    /// }
    /// ```
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
    /// Append an escaped path relative to the root's parent where possible. No filesystem access or
    /// graph mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed graph view, root and color policy.
    /// - `f` `[in,out]`: Formatter; appends output without clearing earlier text.
    /// - `path` `[in]`: Absolute or external path to label.
    /// - `style` `[in]`: Requested terminal label style.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Examples
    /// ```text
    /// view.path(f, &visit.file, FILE)?;
    /// ```
    fn path(&self, f: &mut Formatter<'_>, path: &Path, style: Style) -> fmt::Result {
        let base = self.root.file.parent().unwrap_or(Path::new(""));
        let short = path.strip_prefix(base).unwrap_or(path);
        // Debug formatting keeps control characters inside labels, not in the terminal.
        paint(f, self.colored, style, format_args!("{short:?}"))
    }
    /// Append visit/file/revision/scope IDs and an escaped path. No filesystem access or graph
    /// mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed graph view, root and color policy.
    /// - `f` `[in,out]`: Formatter; appends output without clearing earlier text.
    /// - `graph` `[in]`: Graph containing the selected visit.
    /// - `id` `[in]`: Valid invocation index in graph.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Panics
    /// Panics if id is not a graph invocation index; tree callers use graph-owned indices.
    ///
    /// # Examples
    /// ```text
    /// view.visit(f, graph, 0)?;
    /// ```
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
    /// Append dependency kind, child/path/status and caller-source markers. No dependency loading,
    /// execution or graph mutation.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed graph view, root and color policy.
    /// - `f` `[in,out]`: Formatter; appends output without clearing earlier text.
    /// - `graph` `[in]`: Graph used to label entered child visits.
    /// - `edge` `[in]`: Dependency result and conditional/deferred/optional flags.
    /// - `dependency` `[in]`: Optional original operands used to label unresolved paths.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Panics
    /// Panics if an entered edge points outside graph.invocations; produced graphs preserve this
    /// invariant.
    ///
    /// # Algorithm
    /// Show the child visit if entered. Otherwise label the resolved path, original operand or
    /// unknown marker, then classify candidate/not-visited/unresolved/optional-absent/missing
    /// state. Append optional and source-control context without claiming execution.
    ///
    /// # Flow
    /// ```text
    /// [dependency edge]
    ///         |
    ///  <child invocation?>
    ///     /           \
    ///  [visit label] [path/original/?]
    ///     |           |
    ///     |      [unvisited status]
    ///     +-----------+
    ///            |
    ///  [flags + caller coordinates]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// view.edge(f, graph, &graph.edges[0], Some(dependency))?;
    /// ```
    fn edge(
        &self,
        f: &mut Formatter<'_>,
        graph: &RootGraph,
        edge: &DependencyEdge,
        dependency: Option<&Dependency>,
    ) -> fmt::Result {
        let (name, style) = match edge.kind {
            DependencyKind::Package => ("find_package", INCLUDE),
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
    /// Append an ASCII dependency tree without recursive printing. Repeated visits retain separate
    /// branches; no I/O beyond formatter output.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed graph view, root and color policy.
    /// - `f` `[in,out]`: Formatter; appends output without clearing earlier text.
    /// - `graph` `[in]`: Graph whose caller/child indices and snapshot origins are consistent.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Panics
    /// Panics for inconsistent edge caller/child indices; this expects a graph produced by
    /// workspace traversal.
    ///
    /// # Algorithm
    /// Index child edges once per caller and dependency labels once per snapshot. Print root, then
    /// use an explicit visit/next-edge/last-sibling stack to preserve indentation. Empty graphs
    /// print a root-not-loaded marker.
    ///
    /// # Flow
    /// ```text
    /// [graph edges]        [snapshot facts]
    ///        |                    |
    ///  [children by caller] [dependency label map]
    ///        +----------+---------+
    ///                   |
    ///             <root loaded?>
    ///              /          \
    ///        [empty marker] [iterative stack]
    ///                          |       ^
    ///                    [edge/indent]-+
    ///                          |
    ///                        [done]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// view.tree(f, graph)?;
    /// ```
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
    /// Append workspace failures and deduplicated semantic/build/reference diagnostics. Suppresses
    /// unknown-command errors resolved across files; no analysis mutation or filesystem access.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed graph view, root and color policy.
    /// - `f` `[in,out]`: Formatter; appends output without clearing earlier text.
    /// - `graph` `[in]`: Saved diagnostics, snapshots and contextual reference warnings.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Algorithm
    /// Map file revisions to callers and resolved calls to source spans. Print graph failures, then
    /// each unique snapshot's semantic/build errors while suppressing resolved unknown commands.
    /// Append contextual warnings and print a clean marker only when none were emitted.
    ///
    /// # Flow
    /// ```text
    /// [workspace failures]    [unique snapshots]    [reference warnings]
    ///           |                      |                    |
    ///  [caller/source labels] [suppress resolved calls]     |
    ///           |                      |                    |
    ///           +----------> [append diagnostics] <---------+
    ///                                |
    ///                           <count zero?>
    ///                             /      \
    ///                          clean    done
    /// ```
    ///
    /// # Examples
    /// ```text
    /// view.diagnostics(f, graph)?;
    /// ```
    fn diagnostics(&self, f: &mut Formatter<'_>, graph: &RootGraph) -> fmt::Result {
        writeln!(f)?;
        paint(f, self.colored, HEADING, "DIAGNOSTICS")?;
        writeln!(f)?;
        let callers: HashMap<_, _> = graph
            .invocations
            .iter()
            .map(|visit| (visit.analysis, visit.file.as_path()))
            .collect();
        let resolved: HashSet<_> = graph
            .references
            .resolved_calls
            .iter()
            .map(|location| {
                (
                    location.file.id,
                    location.span.start_byte,
                    location.span.end_byte,
                )
            })
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
                if matches!(
                    diagnostic.content,
                    crate::parser::semanticer::model::SemanticError::CommandNotFound(_)
                ) && resolved.contains(&(
                    visit.snapshot.semantics.id.id,
                    diagnostic.span.start_byte,
                    diagnostic.span.end_byte,
                )) {
                    continue;
                }
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
        for warning in &graph.references.warnings {
            count += 1;
            write!(f, "  WARNING ")?;
            if let Some(visit) = graph.invocations.get(warning.invocation.index()) {
                self.path(f, &visit.file, FILE)?;
            }
            writeln!(
                f,
                " @ {}:{} C{} {:?}",
                warning.warning.span.row + 1,
                warning.warning.span.column + 1,
                warning.location.command.id,
                warning.warning.content.message()
            )?;
        }
        if count == 0 {
            paint(f, self.colored, INCLUDE, "  No diagnostics")?;
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Display for DependencyGraphDisplay<'_> {
    /// Append graph tree, scope summaries and diagnostics, or a pending/stale notice. No graph
    /// mutation or filesystem access.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed graph view, root and color policy.
    /// - `f` `[in,out]`: Formatter; appends output without clearing earlier text.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Algorithm
    /// Write the graph heading; absent captured graph returns after a refresh notice. Otherwise
    /// print the tree, graph scope ancestry/project/write sets, then diagnostics.
    ///
    /// # Flow
    /// ```text
    /// [graph heading]
    ///        |
    ///  <captured graph?>
    ///    /             \
    ///  pending      [tree]
    ///    |             |
    ///  return       [scope loop]
    ///                  |
    ///              [diagnostics]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// fn report(workspace: &Workspace) -> String {
    ///     format!("{}", workspace.display(false).DependcyGraphDisplay)
    /// }
    /// ```
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
    /// Append root/cache statistics and the borrowed dependency graph report. Escapes source
    /// labels; no analysis mutation, file loading or execution occurs.
    ///
    /// # Arguments
    /// - `self` `[in]`: Borrowed workspace overview and its captured graph adapter.
    /// - `f` `[in,out]`: Formatter; appends the overview and graph without clearing previous text.
    ///
    /// # Returns
    /// Ok(()) after appending the requested text; Err(fmt::Error) on formatter failure.
    ///
    /// # Errors
    /// Returns fmt::Error if writing any fragment to the formatter fails; already appended output
    /// is not rolled back.
    ///
    /// # Algorithm
    /// Render root and optional binary path, document/overlay/pending counts, optional graph counts
    /// and the static-analysis notice. Delegate graph/scopes/diagnostics to the captured graph
    /// adapter.
    ///
    /// # Flow
    /// ```text
    /// [workspace root]      [cache counters]
    ///         |                     |
    ///         +----> [overview] <---+
    ///                     |
    ///              <captured graph?>
    ///                 /        \
    ///            counts        omit counts
    ///                 \        /
    ///               [graph Display]
    /// ```
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::workspace::Workspace;
    /// fn report(workspace: &Workspace) -> String {
    ///     format!("{}", workspace.display(false))
    /// }
    /// ```
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
