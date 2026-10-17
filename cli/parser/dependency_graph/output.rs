//! Blocking static dependency graph inspection and buffered caller-supplied output.
//! No source files are rewritten; output errors propagate to the executable.
//!
//! Contains AI-generated documentation additions.
// AI-generated implementation.
use cmake_analyzer::workspace::Workspace;
use std::io::{self, Write};

/// Display the prepared Workspace and flush the supplied writer. Does not load sources or
/// launch processes here. Resolved cross-file calls suppress local unknown-command failures.
///
/// # Arguments
/// - `workspace` `[in]`: already flushed static graph; not mutated.
/// - `colored` `[in]`: enable explicit terminal color escapes.
/// - `output` `[in,out]`: consumed writer handle; formatted bytes are appended at its current position and it is flushed. Existing content is not cleared.
///
/// # Returns
/// `Ok(true)` for graph/build diagnostics or unsuppressed semantic diagnostics; `Ok(false)` for a clean or absent graph. `Err` for output failure.
///
/// # Errors
/// Writer formatting/flush errors; partial output may already have been written.
///
/// # Algorithm
/// Write and flush the graph display. Build a set of resolved call spans, then scan graph visits for diagnostics not explained by those calls.
///
/// # Flow
/// ```text
/// [Workspace display] ---> [writer + flush] --error--> [Err]
///                                |
///                       [resolved call span set]
///                                |
///                         [scan diagnostics]
///                           /         \
///              [resolved unknown call] [other diagnostics]
///                        |                    |
///                   [suppress]           [Ok(true)]
///                        |
///                [Ok(false) if no others]
/// ```
///
/// # Examples
/// ```text
/// let errors = inspect(&workspace, false, Vec::new())?;
/// ```
pub(crate) fn inspect(
    workspace: &Workspace,
    colored: bool,
    mut output: impl Write,
) -> io::Result<bool> {
    write!(output, "{}", workspace.display(colored))?;
    output.flush()?;
    Ok(workspace.root_graph().is_some_and(|graph| {
        let resolved: std::collections::HashSet<_> = graph
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
        !graph.diagnostics.is_empty()
            || graph.invocations.iter().any(|visit| {
                visit
                    .snapshot
                    .semantics
                    .diagnostics
                    .iter()
                    .any(|diagnostic| {
                        !matches!(
                        diagnostic.content,
                        cmake_analyzer::parser::semanticer::model::SemanticError::CommandNotFound(
                            _
                        )
                    ) || !resolved.contains(&(
                            visit.snapshot.semantics.id.id,
                            diagnostic.span.start_byte,
                            diagnostic.span.end_byte,
                        ))
                    })
                    || !visit.snapshot.build.diagnostics.is_empty()
            })
    }))
}
