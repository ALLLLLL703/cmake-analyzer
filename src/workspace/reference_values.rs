//! Replay conservative values alongside reference binding without changing source snapshots.
//! Observations carry visit identity; deferred bodies and uncertain imports do not invent values.
// Contains AI-generated documentation additions.
// AI-authored value replay alongside reference-index aggregation.
use super::model::{Invocation, RootGraph};
use crate::parser::semanticer::{model::*, references::ReferenceCollector, values::flags};
use std::collections::HashMap;
/// Conservative bindings and sparse observations synchronized with reference environments. No I/O.
///
/// Reference aggregation during [`crate::workspace::Workspace::flush`] drives replay;
/// [`Self::command`] records reads before applying writes.
///
/// # Flow
/// ```text
/// [reference environments]      [visit directories]
///           |                          |
///           +----> [command replay] <---+
///                          |
///                [read observations]
///                          |
///                  [apply/invalidate]
///                          |
///               [write observations]
/// ```
///
/// # Examples
/// ```text
/// let mut replay = ValueReplay::new(root_environment, graph);
/// ```
pub(super) struct ValueReplay {
    /// Copy-on-write value state per owning reference environment Example: `replay.bindings.len()`.
    bindings: HashMap<ReferenceEnvironmentId, ValueBindings>,
    /// Directory builtin contexts indexed by graph invocation Example: `replay.directories.len()`.
    directories: Vec<ValueDirectories>,
    /// Appended command/visit-qualified read and output-value observations Example:
    /// `replay.observations.len()` .
    pub observations: Vec<VariableValueAt>,
}
impl ValueReplay {
    /// Initialize root bindings and visit directory contexts without I/O. Seeds PROJECT_SOURCE_DIR
    /// only for project-default entries.
    ///
    /// # Arguments
    /// - `root` `[in]`: Root reference environment allocated by the collector.
    /// - `graph` `[in]`: Visit contexts; empty graphs produce no directory contexts.
    ///
    /// # Returns
    /// Replay state with root bindings and no observations.
    ///
    /// # Examples
    /// ```text
    /// let replay = ValueReplay::new(root_environment, graph);
    /// ```
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
    /// Copy the parent's currently known values into a child environment. No I/O; later writes
    /// remain independent.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: Overwrites the child binding entry; observations are unchanged.
    /// - `child` `[in]`: New child reference-environment key.
    /// - `parent` `[in]`: Parent key; missing parent means empty bindings.
    ///
    /// # Returns
    /// () after installing the child's initial state.
    ///
    /// # Examples
    /// ```text
    /// replay.fork(child_environment, parent_environment);
    /// ```
    pub fn fork(&mut self, child: ReferenceEnvironmentId, parent: ReferenceEnvironmentId) {
        self.bindings.insert(
            child,
            self.bindings.get(&parent).cloned().unwrap_or_default(),
        );
    }
    /// Append sparse reads/writes and update owning-environment values for a nondeferred command.
    /// No I/O or source mutation.
    ///
    /// # Arguments
    /// - `self` `[in,out]` : May create inherited bindings, update/invalidate them and append
    ///   observations.
    /// - `collector` `[in]`: Reference owner/parent lookup for this command environment.
    /// - `environment` `[in]`: Reference environment active at the command.
    /// - `visit` `[in]`: Owning graph visit and semantic snapshot.
    /// - `command` `[in]`: Command with variable references and schema output arguments.
    /// - `invocation` `[in]`: Visit ID indexing this replay's directory contexts.
    /// - `traversed` `[in]`: Whether a resolved dependency child is scheduled at this command.
    ///
    /// # Returns
    /// () after recording reads before writes; deferred bodies produce no observations or value
    /// effects.
    ///
    /// # Panics
    /// Panics if invocation does not index this replay's directory vector; callers pass an ID from
    /// the same graph.
    ///
    /// # Algorithm
    /// Lazily inherit owner bindings. Record variable reads from the incoming state, apply
    /// conservative command effects, skip opaque find_package invalidation when its child is
    /// replayed, and invalidate unresolved includes. Finally append literal Out-argument values
    /// from the resulting state.
    ///
    /// # Flow
    /// ```text
    /// [owner bindings] ----> <deferred?> -- yes --> [return]
    ///          | no
    ///   [observe reads]
    ///          |
    ///   <traversed package?>
    ///      /             \
    ///     skip       apply/invalidate
    ///      \             /
    ///        [observe Out writes]
    ///                  |
    ///            [append results]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// replay.command(&collector, environment, visit, command, invocation, true);
    /// ```
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
