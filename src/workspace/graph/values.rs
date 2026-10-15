// AI-authored command-order Workspace value propagation.
use super::{Frame, local_scope};
use crate::{parser::semanticer::values::flags, workspace::model::RootGraph};
pub(super) fn advance(frame: &mut Frame, graph: &mut RootGraph, through: usize) {
    let end = (through + 1).min(frame.snapshot.semantics.commands.len());
    for command in &frame.snapshot.semantics.commands[frame.next_command..end] {
        let (conditional, deferred) = flags(&frame.snapshot.semantics, command);
        if deferred {
            continue;
        }
        let scope = local_scope(
            command.scope.map(|id| id.id),
            frame.invocation,
            &frame.snapshot.semantics,
            &mut frame.local_scopes,
            graph,
        );
        let values = &mut graph.scopes[scope].values;
        values.apply(command, conditional, Some(&frame.directories));
        if conditional && command.name.eq_ignore_ascii_case("include") {
            values.invalidate();
        }
    }
    frame.next_command = end;
}
