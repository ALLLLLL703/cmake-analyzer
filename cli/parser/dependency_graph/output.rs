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
        !graph.diagnostics.is_empty()
            || graph.invocations.iter().any(|visit| {
                !visit.snapshot.semantics.diagnostics.is_empty()
                    || !visit.snapshot.build.diagnostics.is_empty()
            })
    }))
}
