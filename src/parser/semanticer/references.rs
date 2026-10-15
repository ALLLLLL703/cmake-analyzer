// AI-authored static reference binding and literal variable-state tracking.
use super::{model::*, parsing::references::argument_name_span};
use crate::model::Spanned;
use std::collections::{HashMap, HashSet};

mod model;
use model::{
    Binding, Environment, Frame, IndexedFunctionId, IndexedVariableId, SnapshotFunctionId,
    SnapshotVariableId, Value,
};
pub(crate) use model::{FileContext, ReferenceCollector};
impl ReferenceCollector {
    pub fn owner(&self, environment: ReferenceEnvironmentId) -> ReferenceEnvironmentId {
        self.environments[environment.index()].owner
    }
    pub fn parent(&self, environment: ReferenceEnvironmentId) -> Option<ReferenceEnvironmentId> {
        self.environments[environment.index()].parent
    }
    pub fn finish(mut self) -> ReferenceIndex {
        // Deferred bodies can call declarations reached later; these remain static candidates.
        let mut functions: HashMap<String, Vec<IndexedFunctionId>> = HashMap::new();
        for (id, function) in self.index.functions.iter().enumerate() {
            functions
                .entry(function.name.to_ascii_lowercase())
                .or_default()
                .push(IndexedFunctionId::new(id));
        }
        for reference in std::mem::take(&mut self.index.unresolved) {
            let key = (reference.location.file, reference.location.command);
            if reference.target == ReferenceTarget::Function && self.deferred_calls.contains(&key) {
                if let Some(candidates) = functions.get(&reference.name.to_ascii_lowercase()) {
                    for &id in candidates {
                        if self.call_seen.insert((id, key.0, key.1)) {
                            self.index.functions[id.index()]
                                .references
                                .push(reference.location.clone());
                        }
                    }
                    if self.resolved_seen.insert(key) {
                        self.index.resolved_calls.push(reference.location);
                    }
                    continue;
                }
            }
            self.index.unresolved.push(reference);
        }
        self.index
    }
    pub fn root(&mut self) -> ReferenceEnvironmentId {
        let id = ReferenceEnvironmentId::new(self.environments.len());
        self.environments.push(Environment {
            owner: id,
            ..Default::default()
        });
        id
    }
    pub fn fork(
        &mut self,
        parent: ReferenceEnvironmentId,
        deferred: bool,
    ) -> ReferenceEnvironmentId {
        let id = self.root();
        self.environments[id.index()].parent = Some(parent);
        self.environments[id.index()].deferred =
            deferred || self.environments[parent.index()].deferred;
        id
    }
    pub fn file(&mut self, buffer: &SemanticedBuffer, root: ReferenceEnvironmentId) -> FileContext {
        for variable in buffer.variables.iter().filter(|variable| variable.builtin) {
            if !self.builtins.contains(&variable.name) {
                self.builtins.insert(variable.name.clone());
            }
        }
        FileContext {
            root,
            frames: HashMap::new(),
            local_blocks: buffer.scopes.iter().map(|scope| scope.block).collect(),
            definitions: buffer
                .functions
                .iter()
                .enumerate()
                .map(|(id, function)| (function.definition.command, SnapshotFunctionId::new(id)))
                .collect(),
            function_blocks: buffer
                .functions
                .iter()
                .enumerate()
                .map(|(id, function)| (function.block, SnapshotFunctionId::new(id)))
                .collect(),
        }
    }
    fn lookup(&self, mut environment: ReferenceEnvironmentId, name: &str) -> Option<Binding> {
        // Call epochs invalidate value facts without copying every visible binding.
        let mut invalidated = 0;
        loop {
            invalidated = invalidated.max(self.environments[environment.index()].invalidate_after);
            if let Some(binding) = self.environments[environment.index()].variables.get(name) {
                let mut binding = *binding;
                if binding.written_at < invalidated {
                    binding.value = Value::Unknown;
                }
                return Some(binding);
            }
            environment = self.environments[environment.index()].parent?;
        }
    }
    fn function(
        &self,
        mut environment: ReferenceEnvironmentId,
        name: &str,
    ) -> Vec<IndexedFunctionId> {
        loop {
            if let Some(functions) = self.environments[environment.index()].functions.get(name) {
                return functions.clone();
            }
            let Some(parent) = self.environments[environment.index()].parent else {
                return Vec::new();
            };
            environment = parent;
        }
    }
    fn variable(
        &mut self,
        environment: ReferenceEnvironmentId,
        name: &str,
        scope: Option<ScopeId>,
        definition: Option<ReferenceLocation>,
    ) -> IndexedVariableId {
        let owner = self.environments[environment.index()].owner;
        let key = (owner, name.to_owned());
        if let Some(&id) = self.variables.get(&key) {
            return id;
        }
        let id = IndexedVariableId::new(self.index.variables.len());
        self.index.variables.push(IndexedVariable {
            name: name.to_owned(),
            scope: owner,
            local_scope: scope,
            possible_value: None,
            definition,
            references: Vec::new(),
        });
        self.variables.insert(key, id);
        id
    }
    fn record(&mut self, variable: IndexedVariableId, location: ReferenceLocation) {
        if self.reference_seen.insert((
            variable,
            location.file,
            location.command,
            location.span.start_byte,
            location.span.end_byte,
            location.kind,
        )) {
            self.index.variables[variable.index()]
                .references
                .push(location);
        }
    }
    fn define_function(
        &mut self,
        function: &SemanticFunction,
        environment: ReferenceEnvironmentId,
    ) {
        let key = (function.definition.file, function.definition.command);
        let id = if let Some(&id) = self.functions.get(&key) {
            id
        } else {
            let id = IndexedFunctionId::new(self.index.functions.len());
            let mut function = function.clone();
            function.references = vec![function.definition.clone()];
            self.index.functions.push(function);
            self.functions.insert(key, id);
            id
        };
        self.environments[environment.index()]
            .functions
            .insert(function.name.to_ascii_lowercase(), vec![id]);
    }
    pub fn environment(
        &mut self,
        context: &mut FileContext,
        buffer: &SemanticedBuffer,
        command: &SemanticCommandRepresent,
    ) -> ReferenceEnvironmentId {
        let Some(owner) = command.block else {
            return context.root;
        };
        if buffer.blocks[owner.id as usize].opening != command.id {
            if let Some(frame) = context.frames.get(&owner) {
                return frame.environment;
            }
        }
        let mut chain = Vec::new();
        let mut block = Some(owner);
        while let Some(id) = block {
            chain.push(id);
            block = buffer.blocks[id.id as usize].parent;
        }
        let mut environment = context.root;
        for id in chain.into_iter().rev() {
            let block = &buffer.blocks[id.id as usize];
            // Declarations execute in their surrounding scope, not in their body.
            if block.opening == command.id && block.kind != BlockKind::Foreach {
                break;
            }
            if let Some(frame) = context.frames.get(&id) {
                environment = frame.environment;
                continue;
            }
            let deferred = matches!(block.kind, BlockKind::Function | BlockKind::Macro);
            let local = deferred || context.local_blocks.contains(&id);
            let conditional = matches!(
                block.kind,
                BlockKind::If | BlockKind::While | BlockKind::Foreach
            );
            if !local && !conditional {
                continue;
            }
            let child = self.fork(environment, deferred);
            if !local {
                self.environments[child.index()].owner =
                    self.environments[environment.index()].owner;
            }
            context.frames.insert(
                id,
                Frame {
                    environment: child,
                    parent: environment,
                    conditional,
                    writes: HashMap::new(),
                    functions: HashMap::new(),
                },
            );
            environment = child;
            if deferred {
                if let Some(function) = context
                    .function_blocks
                    .get(&id)
                    .map(|index| &buffer.functions[index.index()])
                {
                    for parameter in &function.parameters {
                        let variable = self.variable(
                            child,
                            &parameter.name,
                            command.scope,
                            Some(parameter.definition.clone()),
                        );
                        self.record(variable, parameter.definition.clone());
                        self.environments[child.index()].variables.insert(
                            parameter.name.clone(),
                            Binding {
                                variable,
                                value: Value::Unknown,
                                written_at: self.clock,
                            },
                        );
                    }
                }
            }
        }
        environment
    }
    pub fn command(
        &mut self,
        context: &mut FileContext,
        buffer: &SemanticedBuffer,
        source: &str,
        command: &SemanticCommandRepresent,
        invocation: ReferenceInvocationId,
    ) -> ReferenceEnvironmentId {
        let environment = self.environment(context, buffer, command);
        let name = command.name.to_ascii_lowercase();
        if matches!(name.as_str(), "else" | "elseif") {
            if let Some(frame) = command
                .block
                .and_then(|block| context.frames.get_mut(&block))
                .filter(|frame| frame.conditional)
            {
                save_branch(frame, &mut self.environments[environment.index()]);
            }
        }
        for reference in &command.variable_references {
            self.read(reference, environment, invocation);
        }
        if command.closed {
            if let Some(function) = context
                .definitions
                .get(&command.id)
                .map(|index| &buffer.functions[index.index()])
            {
                self.define_function(function, environment);
            } else {
                let functions = self.function(environment, &name);
                let location = ReferenceLocation {
                    file: buffer.id,
                    command: command.id,
                    span: command.span,
                    kind: ReferenceKind::Call,
                    possible_value: None,
                };
                if !functions.is_empty() {
                    // User calls may change caller-visible values; do not evaluate their bodies.
                    self.clock += 1;
                    self.environments[environment.index()].invalidate_after = self.clock;
                    for id in functions {
                        if self.call_seen.insert((id, location.file, location.command)) {
                            self.index.functions[id.index()]
                                .references
                                .push(location.clone());
                        }
                    }
                    if self.resolved_seen.insert((location.file, location.command)) {
                        self.index.resolved_calls.push(location);
                    }
                } else if command.schema.is_none() {
                    if self.environments[environment.index()].deferred {
                        self.deferred_calls.insert((buffer.id, command.id));
                    }
                    self.index.unresolved.push(UnresolvedReference {
                        name: command.name.clone(),
                        target: ReferenceTarget::Function,
                        location,
                    });
                }
            }
            self.writes(buffer, source, command, environment);
        }
        if let Some(block) = command
            .block
            .and_then(|id| buffer.blocks.get(id.id as usize))
            .filter(|block| block.closing == Some(command.id))
        {
            if let Some(frame) = context
                .frames
                .remove(&block.id)
                .filter(|frame| frame.conditional)
            {
                self.merge(frame);
            }
        }
        environment
    }
    fn read(
        &mut self,
        reference: &ArgumentReference,
        environment: ReferenceEnvironmentId,
        invocation: ReferenceInvocationId,
    ) {
        let mut binding = self.lookup(environment, &reference.name);
        if binding.is_none() && self.builtins.contains(&reference.name) {
            let root = context_root(&self.environments, environment);
            let variable = self.variable(root, &reference.name, None, None);
            let value = Binding {
                variable,
                value: Value::Unknown,
                written_at: self.clock,
            };
            self.environments[root.index()]
                .variables
                .insert(reference.name.clone(), value);
            binding = Some(value);
        }
        let warning = match binding {
            Some(binding) => {
                self.record(binding.variable, reference.location.clone());
                if self.environments[environment.index()].deferred
                    && self.index.variables[binding.variable.index()].scope
                        != self.environments[environment.index()].owner
                {
                    None
                } else {
                    match binding.value {
                        Value::Unset => Some(SemanticWarnKind::Unset),
                        Value::Empty => Some(SemanticWarnKind::Empty),
                        Value::MaybeUnset => Some(SemanticWarnKind::PossiblyUnset),
                        _ => None,
                    }
                }
            }
            None => {
                self.index.unresolved.push(UnresolvedReference {
                    name: reference.name.clone(),
                    target: ReferenceTarget::Variable,
                    location: reference.location.clone(),
                });
                Some(SemanticWarnKind::PossiblyUnset)
            }
        };
        if let Some(kind) = warning {
            self.index.warnings.push(ReferenceWarning {
                invocation,
                location: reference.location.clone(),
                warning: Spanned::new(
                    SemanticWarn {
                        variable: reference.name.clone(),
                        kind,
                    },
                    reference.location.span,
                ),
            });
        }
    }
    fn writes(
        &mut self,
        buffer: &SemanticedBuffer,
        source: &str,
        command: &SemanticCommandRepresent,
        environment: ReferenceEnvironmentId,
    ) {
        let native = command.schema.as_ref().is_some_and(|schema| {
            matches!(
                schema.from,
                super::schema::CommandSchemaFrom::Builtin
                    | super::schema::CommandSchemaFrom::KeyWord
            )
        });
        let name = command.name.to_ascii_lowercase();
        let assignment = native && matches!(name.as_str(), "set" | "unset");
        let cache = assignment
            && (name == "unset"
                && command.args.get(1).and_then(SemanticArg::literal_text) == Some("CACHE")
                || name == "set"
                    && command.args.iter().enumerate().skip(1).any(|(index, arg)| {
                        arg.literal_text() == Some("CACHE")
                            && command.args.len() >= index + 3
                            && matches!(
                                command.args[index + 1].literal_text(),
                                Some("BOOL" | "FILEPATH" | "PATH" | "STRING" | "INTERNAL")
                            )
                    }));
        if cache
            || assignment
                && command
                    .args
                    .first()
                    .and_then(SemanticArg::literal_text)
                    .is_some_and(|name| name.starts_with("ENV{") || name.starts_with("CACHE{"))
        {
            return;
        }
        let nonlocal = assignment
            && command.args.last().and_then(SemanticArg::literal_text) == Some("PARENT_SCOPE");
        for (index, arg) in command.args.iter().enumerate() {
            let unset = native
                && index == 0
                && (name == "unset" || (name == "set" && command.args.len() == 1));
            let inplace = native
                && index == 1
                && match name.as_str() {
                    "list" => matches!(
                        command.args.first().and_then(SemanticArg::literal_text),
                        Some(
                            "APPEND"
                                | "PREPEND"
                                | "INSERT"
                                | "POP_BACK"
                                | "POP_FRONT"
                                | "REMOVE_ITEM"
                                | "REMOVE_AT"
                                | "REMOVE_DUPLICATES"
                                | "FILTER"
                                | "TRANSFORM"
                                | "REVERSE"
                                | "SORT"
                        )
                    ),
                    "string" => matches!(
                        command.args.first().and_then(SemanticArg::literal_text),
                        Some("APPEND" | "PREPEND")
                    ),
                    _ => false,
                };
            if !unset && !inplace && !arg.kind.iter().any(|kind| matches!(kind, ArgKind::Out)) {
                continue;
            }
            let Some(variable_name) = arg.literal_text().filter(|name| !name.is_empty()) else {
                continue;
            };
            if nonlocal {
                self.index.unresolved.push(UnresolvedReference {
                    name: variable_name.to_owned(),
                    target: ReferenceTarget::Variable,
                    location: ReferenceLocation {
                        file: buffer.id,
                        command: command.id,
                        span: argument_name_span(source, arg),
                        kind: ReferenceKind::Write,
                        possible_value: None,
                    },
                });
                continue;
            }
            let owner = self.environments[environment.index()].owner;
            let existing = self
                .lookup(environment, variable_name)
                .filter(|binding| self.index.variables[binding.variable.index()].scope == owner);
            let mut location = ReferenceLocation {
                file: buffer.id,
                command: command.id,
                span: argument_name_span(source, arg),
                kind: ReferenceKind::Write,
                possible_value: None,
            };
            let variable = if let Some(binding) = existing {
                binding.variable
            } else {
                self.variable(environment, variable_name, command.scope, None)
            };
            if !unset && self.index.variables[variable.index()].definition.is_none() {
                location.kind = ReferenceKind::Definition;
                self.index.variables[variable.index()].definition = Some(location.clone());
            }
            self.record(variable, location);
            let value = if unset || (native && name == "set" && command.args.len() == 1) {
                Value::Unset
            } else if native && name == "set" && command.args.len() == 2 {
                match command.args[1].literal_text() {
                    Some("") => Value::Empty,
                    Some(_) => Value::NonEmpty,
                    None => Value::Unknown,
                }
            } else {
                Value::Unknown
            };
            let mut binding = Binding {
                variable,
                value,
                written_at: self.clock,
            };
            if unset {
                let mut parent = self.environments[environment.index()].parent;
                while let Some(id) = parent {
                    if self.environments[id.index()].owner != owner {
                        if let Some(inherited) = self.lookup(id, variable_name) {
                            binding = inherited;
                        }
                        break;
                    }
                    parent = self.environments[id.index()].parent;
                }
            }
            self.environments[environment.index()]
                .variables
                .insert(variable_name.to_owned(), binding);
        }
    }
    fn merge(&mut self, mut frame: Frame) {
        let environment = frame.environment;
        save_branch(&mut frame, &mut self.environments[environment.index()]);
        self.environments[frame.parent.index()].invalidate_after = self.environments
            [frame.parent.index()]
        .invalidate_after
        .max(self.environments[environment.index()].invalidate_after);
        for (name, mut binding) in frame.writes {
            binding.value = match self.lookup(frame.parent, &name) {
                Some(old) if old.value == binding.value => old.value,
                Some(_) => Value::Unknown,
                None => Value::MaybeUnset,
            };
            self.environments[frame.parent.index()]
                .variables
                .insert(name, binding);
        }
        for (name, functions) in frame.functions {
            self.environments[frame.parent.index()]
                .functions
                .entry(name)
                .or_default()
                .extend(functions);
        }
    }
}
fn save_branch(frame: &mut Frame, environment: &mut Environment) {
    for (name, mut binding) in std::mem::take(&mut environment.variables) {
        if binding.written_at < environment.invalidate_after {
            binding.value = Value::Unknown;
        }
        frame
            .writes
            .entry(name)
            .and_modify(|old| {
                if old.value != binding.value {
                    old.value = Value::Unknown;
                }
            })
            .or_insert(binding);
    }
    for (name, functions) in std::mem::take(&mut environment.functions) {
        frame.functions.entry(name).or_default().extend(functions);
    }
}
fn context_root(
    environments: &[Environment],
    mut environment: ReferenceEnvironmentId,
) -> ReferenceEnvironmentId {
    while let Some(parent) = environments[environment.index()].parent {
        environment = parent;
    }
    environment
}

impl Semanticer {
    pub(crate) fn collect_references(&self, result: &mut SemanticedBuffer, source: &str) {
        let mut collector = ReferenceCollector::default();
        let root = collector.root();
        let mut context = collector.file(result, root);
        for command in &result.commands {
            collector.command(
                &mut context,
                result,
                source,
                command,
                ReferenceInvocationId::new(0),
            );
        }
        let index = collector.finish();
        result.warnings = index
            .warnings
            .into_iter()
            .map(|warning| warning.warning)
            .collect();
        result.unresolved_references = index.unresolved;
        result.functions = index.functions;
        for variable in &mut result.variables {
            variable.references.clear();
        }
        let mut variables: HashMap<_, _> = result
            .variables
            .iter()
            .enumerate()
            .map(|(id, variable)| {
                (
                    (variable.scope, variable.name.clone()),
                    SnapshotVariableId::new(id),
                )
            })
            .collect();
        for variable in index.variables {
            let key = (variable.local_scope, variable.name.clone());
            let id = variables.get(&key).copied();
            let target = match id {
                Some(id) => id,
                None => {
                    let id = SnapshotVariableId::new(result.variables.len());
                    result.variables.push(SemanticVariable {
                        name: variable.name,
                        builtin: false,
                        first_write: None,
                        scope: variable.local_scope,
                        changing: Vec::new(),
                        references: Vec::new(),
                    });
                    id
                }
            };
            variables.insert(key, target);
            result.variables[target.index()]
                .references
                .extend(variable.references);
        }
        let resolved: HashSet<_> = index
            .resolved_calls
            .iter()
            .map(|location| (location.span.start_byte, location.span.end_byte))
            .collect();
        result.diagnostics.retain(|diagnostic| {
            !matches!(diagnostic.content, SemanticError::CommandNotFound(_))
                || !resolved.contains(&(diagnostic.span.start_byte, diagnostic.span.end_byte))
        });
    }
}
