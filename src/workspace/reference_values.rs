// AI-authored value replay alongside reference-index aggregation.
use super::model::{Invocation, RootGraph};
use crate::parser::semanticer::{model::*, references::ReferenceCollector, values::flags};
use std::collections::HashMap;
pub(super) struct ValueReplay {
    bindings: HashMap<ReferenceEnvironmentId, ValueBindings>,
    directories: Vec<ValueDirectories>,
    pub observations: Vec<VariableValueAt>,
}
impl ValueReplay {
    pub fn new(root: ReferenceEnvironmentId, graph: &RootGraph) -> Self {
        let mut values = ValueBindings::default();
        if let Some(visit) = graph.invocations.first().filter(|_| graph.project_defaults) {
            values.assign(
                "PROJECT_SOURCE_DIR",
                visit
                    .directory
                    .source
                    .to_str()
                    .map(|path| PossibleValue::Static(path.into())),
            );
        }
        let directories = graph
            .invocations
            .iter()
            .map(|visit| ValueDirectories {
                source_root: graph.invocations[0].directory.source.clone(),
                source: visit.directory.source.clone(),
                binary_root: graph.invocations[0].directory.binary.clone(),
                binary: visit.directory.binary.clone(),
                file: visit.file.clone(),
            })
            .collect();
        Self {
            bindings: HashMap::from([(root, values)]),
            directories,
            observations: Vec::new(),
        }
    }
    pub fn fork(&mut self, child: ReferenceEnvironmentId, parent: ReferenceEnvironmentId) {
        self.bindings.insert(
            child,
            self.bindings.get(&parent).cloned().unwrap_or_default(),
        );
    }
    pub fn command(
        &mut self,
        collector: &ReferenceCollector,
        environment: ReferenceEnvironmentId,
        visit: &Invocation,
        command: &SemanticCommandRepresent,
        invocation: ReferenceInvocationId,
        traversed: bool,
    ) {
        let owner = collector.owner(environment);
        if !self.bindings.contains_key(&owner) {
            let parent = collector
                .parent(owner)
                .map(|parent| collector.owner(parent));
            let bindings = parent
                .and_then(|parent| self.bindings.get(&parent))
                .cloned()
                .unwrap_or_default();
            self.bindings.insert(owner, bindings);
        }
        let dirs = &self.directories[invocation.index()];
        let (conditional, deferred) = flags(&visit.snapshot.semantics, command);
        if deferred {
            return;
        }
        let Some(values) = self.bindings.get_mut(&owner) else {
            return;
        };
        for reference in &command.variable_references {
            self.observations.push(VariableValueAt {
                name: reference.name.clone(),
                location: reference.location.clone(),
                scope: command.scope,
                invocation: Some(invocation),
                possible_value: values.get(&reference.name, Some(dirs)),
            });
        }
        if !(traversed && command.name.eq_ignore_ascii_case("find_package")) {
            values.apply(command, conditional, Some(dirs));
        }
        if !traversed && command.name.eq_ignore_ascii_case("include") {
            values.invalidate();
        }
        for arg in command
            .args
            .iter()
            .filter(|arg| arg.kind.iter().any(|kind| matches!(kind, ArgKind::Out)))
        {
            if let Some(name) = arg.literal_text() {
                self.observations.push(VariableValueAt {
                    name: name.to_owned(),
                    location: ReferenceLocation {
                        file: visit.snapshot.semantics.id,
                        command: command.id,
                        span: arg.span,
                        kind: ReferenceKind::Write,
                        possible_value: values.get(name, Some(dirs)),
                    },
                    scope: command.scope,
                    invocation: Some(invocation),
                    possible_value: values.get(name, Some(dirs)),
                });
            }
        }
    }
}
