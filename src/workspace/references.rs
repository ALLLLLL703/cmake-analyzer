// AI-authored source-order reference aggregation over resolved dependency visits.
use super::model::*;
use crate::parser::semanticer::{
    model::{ReferenceIndex, ReferenceInvocationId, VariableValueAt},
    references::ReferenceCollector,
};
use std::collections::HashMap;

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
