//! Advance conservative scope values in command order before each dependency effect.
//! Deferred bodies are skipped and uncertain imports invalidate values; no I/O occurs.
// Contains AI-generated documentation additions.
// AI-authored command-order Workspace value propagation.
use super::{Frame, local_scope};
use crate::{parser::semanticer::values::flags, workspace::model::RootGraph};
/// Apply not-yet-replayed semantic commands through a fact's command position. No I/O or source
/// mutation.
///
/// # Arguments
/// - `frame` `[in,out]` : Replay cursor and local-scope mapping; next_command is overwritten with
///   the exclusive end.
/// - `graph` `[in,out]`: May append local scopes and update/invalidate their value bindings.
/// - `through` `[in]`: Inclusive command position, clamped to the command count.
///
/// # Returns
/// () after advancing values. Deferred bodies do not apply; conditional includes/packages
/// invalidate certainty.
///
/// # Panics
/// Panics if the internal cursor is beyond the requested end or graph IDs are inconsistent; callers
/// advance monotonically within a valid visit.
///
/// # Algorithm
/// Replay only the unreplayed command slice through the requested position. Skip deferred bodies,
/// map scopes per visit, apply conservative values, and invalidate conditional imports. Package
/// effects are supplied by dependency visits, so unconditional find_package is not treated as an
/// opaque local call.
///
/// # Flow
/// ```text
/// [unreplayed commands]
///          |
///  <deferred?> -- yes --> [next]
///          | no
///  [map scope] ----> <package?>
///                      /       \
///              conditional?   apply values
///                   |             |
///               invalidate?  conditional include?
///                   |             |
///                   +----> [next]
///                              |
///                       [update cursor]
/// ```
///
/// # Examples
/// ```text
/// advance(&mut frame, &mut graph, command.id.id as usize);
/// ```
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
        // Package effects are replayed by its dependency visit, not an opaque-call invalidation.
        if command.name.eq_ignore_ascii_case("find_package") {
            if conditional {
                values.invalidate();
            }
            continue;
        }
        values.apply(command, conditional, Some(&frame.directories));
        if conditional && command.name.eq_ignore_ascii_case("include") {
            values.invalidate();
        }
    }
    frame.next_command = end;
}
