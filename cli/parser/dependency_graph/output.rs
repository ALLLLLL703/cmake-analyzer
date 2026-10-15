// AI-generated implementation.
use cmake_analyzer::workspace::Workspace;
use std::io::{self, Write};

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
