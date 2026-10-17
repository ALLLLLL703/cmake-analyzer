//! Replay resolved visits in command order to build immutable owner-qualified reference indexes.
//! Includes/packages share environments; subdirectories fork them; functions-only visits export
//! declarations without variable/value effects. Analysis itself performs no I/O.
// Contains AI-generated documentation additions.
// AI-authored source-order reference aggregation over resolved dependency visits.
use super::model::*;
use crate::parser::semanticer::{
    model::{ReferenceIndex, ReferenceInvocationId, VariableValueAt},
    references::ReferenceCollector,
};
use std::collections::HashMap;

/// Replay graph visits into owner-qualified references and sparse possible values. No I/O or
/// snapshot mutation.
///
/// # Arguments
/// - `graph` `[in]`: Retained visits/edges with caller-before-child order and matching revisions.
///
/// # Returns
/// (ReferenceIndex, contextual value observations). Conflicting values for the same source position
/// across visits become unknown in reference summaries.
///
/// # Algorithm
/// Build child schedules keyed by visit and dependency command. Replay commands with a stack so
/// resolved children run before the caller continues. Include/package visits share environments;
/// subdirectories fork reference/value state. Functions-only visits export declarations only.
/// Finish the reference collector, annotate observations, and merge possible values conservatively
/// across visits.
///
/// # Flow
/// ```text
/// [root environment]
///          |
///    [command replay] ----> <functions-only?>
///          |                  |
///          |            [declarations only]
///          v
///    [values + references]
///          |
///   <dependency child?>
///     /              \
///  include/share   subdir/fork
///     \              /
///       [child before caller]
///               |
///        [merge value agreement]
/// ```
///
/// # Examples
/// ```text
/// let (references, values) = analyze(&graph);
/// ```
fn analyze(graph: &RootGraph) -> (ReferenceIndex, Vec<VariableValueAt>) {
    let mut collector = ReferenceCollector::default();
    let root = collector.root();
    let mut values = super::reference_values::ValueReplay::new(root, graph);
    let mut children: HashMap<_, Vec<_>> = HashMap::new();
    for edge in &graph.edges {
        if let Some(child) = edge.invocation {
            children
                .entry((ReferenceInvocationId::new(edge.caller), edge.source.command))
                .or_default()
                .push((ReferenceInvocationId::new(child), edge.kind));
        }
    }
    let mut contexts = HashMap::new();
    let Some(first) = graph.invocations.first() else {
        return (ReferenceIndex::default(), Vec::new());
    };
    let root_visit = ReferenceInvocationId::new(0);
    contexts.insert(root_visit, collector.file(&first.snapshot.semantics, root));
    let mut stack = vec![(root_visit, 0)];
    while let Some((invocation, next)) = stack.pop() {
        let visit = &graph.invocations[invocation.index()];
        let Some(command) = visit.snapshot.semantics.commands.get(next) else {
            continue;
        };
        let Some(context) = contexts.get_mut(&invocation) else {
            continue;
        };
        let environment = if visit.index == ExternalIndex::Functions {
            collector.function_command(context, &visit.snapshot.semantics, command)
        } else {
            let environment = collector.command(
                context,
                &visit.snapshot.semantics,
                &visit.snapshot.text,
                command,
                invocation,
            );
            values.command(
                &collector,
                environment,
                visit,
                command,
                invocation,
                children.contains_key(&(invocation, command.id)),
            );
            environment
        };
        stack.push((invocation, next + 1));
        if let Some(children) = children.get(&(invocation, command.id)) {
            for &(child, kind) in children.iter().rev() {
                let environment =
                    if kind == crate::parser::build_struct::model::DependencyKind::Subdirectory {
                        let child = collector.fork(environment, false);
                        values.fork(child, collector.owner(environment));
                        child
                    } else {
                        environment
                    };
                contexts.insert(
                    child,
                    collector.file(
                        &graph.invocations[child.index()].snapshot.semantics,
                        environment,
                    ),
                );
                stack.push((child, 0));
            }
        }
    }
    let mut index = collector.finish();
    for value in &mut values.observations {
        value.location.possible_value = value.possible_value.clone();
    }
    let mut possible: HashMap<_, Option<crate::parser::semanticer::model::PossibleValue>> =
        HashMap::new();
    for value in &values.observations {
        possible
            .entry((
                value.location.file,
                value.location.command,
                value.name.as_str(),
                value.location.kind == crate::parser::semanticer::model::ReferenceKind::Read,
            ))
            .and_modify(|previous| {
                if *previous != value.possible_value {
                    *previous = None
                }
            })
            .or_insert_with(|| value.possible_value.clone());
    }
    for variable in &mut index.variables {
        for reference in &mut variable.references {
            reference.possible_value = possible
                .get(&(
                    reference.file,
                    reference.command,
                    variable.name.as_str(),
                    reference.kind == crate::parser::semanticer::model::ReferenceKind::Read,
                ))
                .cloned()
                .flatten();
        }
        let mut candidates = variable.references.iter().filter_map(|location| {
            possible.get(&(
                location.file,
                location.command,
                variable.name.as_str(),
                location.kind == crate::parser::semanticer::model::ReferenceKind::Read,
            ))
        });
        if let Some(first) = candidates.next() {
            if candidates.all(|value| value == first) {
                variable.possible_value = first.clone();
            }
        }
    }
    (index, values.observations)
}
impl Workspace {
    /// Replace graph reference/value/target indexes on a bounded blocking worker. Static snapshots
    /// remain unchanged; no file reads or execution occur.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : Temporarily takes the saved graph, then restores it with new shared
    ///   indexes on success.
    ///
    /// # Returns
    /// Ok(()) with refreshed indexes or no graph; Err on permit/join failure. A join failure queues
    /// root refresh.
    ///
    /// # Errors
    /// Closed worker semaphore or failed blocking join. The taken graph is not restored on these
    /// failures; join failure marks the root dirty.
    ///
    /// # Algorithm
    /// Take the current graph, acquire one worker permit and move the graph into a joined blocking
    /// task. Replay variable/function/value indexes first and target bindings second. Restore the
    /// completed graph on success; a join failure queues root refresh instead.
    ///
    /// # Flow
    /// ```text
    /// [take saved graph]
    ///           |
    ///     <graph present?> -- no --> [Ok]
    ///           | yes
    ///     [worker permit]
    ///           |
    ///  [references + values]
    ///           |
    ///       [targets]
    ///           |
    ///     <join success?>
    ///        /          \
    ///  [restore graph] [queue root refresh]
    ///        |                  |
    ///       Ok                 Err
    /// ```
    ///
    /// # Examples
    /// ```text
    /// workspace.refresh_references().await?;
    /// ```
    pub(super) async fn refresh_references(&mut self) -> std::io::Result<()> {
        let Some(mut graph) = self.graph.take() else {
            return Ok(());
        };
        let permit = self
            .workers
            .clone()
            .acquire_owned()
            .await
            .map_err(std::io::Error::other)?;
        let result = tokio::task::spawn_blocking(move || {
            let _permit = permit;
            let (references, values) = analyze(&graph);
            graph.references = std::sync::Arc::new(references);
            graph.values = values;
            graph.targets = std::sync::Arc::new(super::targets::analyze(&graph));
            graph
        })
        .await;
        match result {
            Ok(graph) => {
                self.graph = Some(graph);
                Ok(())
            }
            Err(error) => {
                self.refresh_root();
                Err(std::io::Error::other(error))
            }
        }
    }
}
