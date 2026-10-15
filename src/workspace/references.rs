// AI-authored source-order reference aggregation over resolved dependency visits.
use super::model::*;
use crate::parser::semanticer::{
    model::{ReferenceIndex, ReferenceInvocationId},
    references::ReferenceCollector,
};
use std::collections::HashMap;

fn analyze(graph: &RootGraph) -> ReferenceIndex {
    let mut collector = ReferenceCollector::default();
    let root = collector.root();
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
        return ReferenceIndex::default();
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
        stack.push((invocation, next + 1));
        if let Some(children) = children.get(&(invocation, command.id)) {
            for &(child, kind) in children.iter().rev() {
                let environment =
                    if kind == crate::parser::build_struct::model::DependencyKind::Subdirectory {
                        collector.fork(environment, false)
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
    collector.finish()
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
            graph.references = analyze(&graph);
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
