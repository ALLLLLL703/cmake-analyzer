//! Bind references in source order with conservative branch and deferred-body state.
//!
//! Workspace callers interleave resolved include/subdirectory visits through this collector.
//! Function bodies are indexed as deferred source, not invoked. Calls invalidate value-state
//! facts; conditional writes are merged as possibilities rather than selected by a condition.
//! No filesystem/process access occurs here.
// Contains AI-generated documentation additions.
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
    /// Read the variable identity owner of an environment. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: collector and its current generation arenas, unchanged.
    /// - `environment` `[in]`: valid environment ID from this collector generation.
    ///
    /// # Returns
    /// Owner environment ID; conditional frames can share their parent owner.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    ///
    /// # Examples
    /// ```text
    /// let owner = collector.owner(root);
    /// ```
    pub fn owner(&self, environment: ReferenceEnvironmentId) -> ReferenceEnvironmentId {
        self.environments[environment.index()].owner
    }
    /// Read the next surrounding visibility environment. No side effects.
    ///
    /// # Arguments
    /// - `self` `[in]`: collector and its current generation arenas, unchanged.
    /// - `environment` `[in]`: valid environment ID from this collector generation.
    ///
    /// # Returns
    /// Some parent ID for a child; None for a root.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    ///
    /// # Examples
    /// ```text
    /// assert!(collector.parent(root).is_none());
    /// ```
    pub fn parent(&self, environment: ReferenceEnvironmentId) -> Option<ReferenceEnvironmentId> {
        self.environments[environment.index()].parent
    }
    /// Consume the collector and attach unresolved deferred calls to any same-name source definitions collected later. This records static candidates, not body execution; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: consumed collector; appends candidate call references and resolved-call locations, retaining unmatched uses.
    ///
    /// # Returns
    /// Completed aggregate index; mutable environments and deduplication maps are discarded.
    ///
    /// # Algorithm
    /// Group all source-defined functions by lowercase name. Revisit unresolved function calls marked deferred, attaching them to every same-name candidate and deduplicating function/location pairs. Leave other unresolved uses unchanged.
    ///
    /// # Flow
    /// ```text
    ///  [collected definitions]     [unresolved uses]
    ///             |                       |
    ///        [group by name]              |
    ///             +-----------+-----------+
    ///                         v
    ///               <deferred matching call?>
    ///                   /             \
    ///            [append candidates] [retain unresolved]
    ///                   |             |
    ///                   +------+------+
    ///                          v
    ///                    [return index]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let index = collector.finish();
    /// ```
    pub fn finish(mut self) -> ReferenceIndex {
        // Deferred bodies can call declarations reached later; these remain static candidates.
        let mut functions: HashMap<String, Vec<IndexedFunctionId>> = HashMap::new();
        for (id, function) in self
            .index
            .functions
            .iter()
            .enumerate()
            .filter(|(_, function)| function.definition.is_some())
        {
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
    /// Append an independent root visibility environment. Mutates collector arena only; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    ///
    /// # Returns
    /// New environment ID owning its own variable identities, with no parent or deferred state.
    ///
    /// # Examples
    /// ```text
    /// let mut collector = ReferenceCollector::default();
    /// let root = collector.root();
    /// ```
    pub fn root(&mut self) -> ReferenceEnvironmentId {
        let id = ReferenceEnvironmentId::new(self.environments.len());
        self.environments.push(Environment {
            owner: id,
            ..Default::default()
        });
        id
    }
    /// Append a child visibility environment with its own variable owner. Inherits deferred status but not copied binding maps; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    /// - `parent` `[in]`: valid surrounding environment in this collector.
    /// - `deferred` `[in]`: whether this child belongs to deferred source even if its parent does not.
    ///
    /// # Returns
    /// New child ID whose parent is supplied and owner initially equals itself.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    ///
    /// # Examples
    /// ```text
    /// let child = collector.fork(root, false);
    /// assert_eq!(collector.parent(child), Some(root));
    /// ```
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
    /// Initialize per-visit structural maps and register builtin variable names. Does not create environments or execute the file; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    /// - `buffer` `[in]`: immutable file snapshot providing scopes and declaration arenas.
    /// - `root` `[in]`: caller-selected entry environment ID.
    ///
    /// # Returns
    /// File context with empty lazy frames and snapshot-local definition lookup maps.
    ///
    /// # Examples
    /// ```text
    /// let context = collector.file(&buffer, root);
    /// ```
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
                .filter_map(|(id, function)| {
                    function
                        .definition
                        .as_ref()
                        .map(|definition| (definition.command, SnapshotFunctionId::new(id)))
                })
                .collect(),
            function_blocks: buffer
                .functions
                .iter()
                .enumerate()
                .filter_map(|(id, function)| {
                    function
                        .block
                        .map(|block| (block, SnapshotFunctionId::new(id)))
                })
                .collect(),
        }
    }
    /// Find the nearest visible variable binding, weakening warning state after user-call invalidation. No mutation or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: collector and its current generation arenas, unchanged.
    /// - `environment` `[in]`: valid environment ID from this collector generation.
    /// - `name` `[in]`: exact case-sensitive variable spelling.
    ///
    /// # Returns
    /// Some copied binding with Unknown state when its write clock predates any traversed invalidation; None when no ancestor binds the name.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    ///
    /// # Examples
    /// ```text
    /// let binding = collector.lookup(root, "answer");
    /// ```
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
    /// Find the nearest visible candidate function list. No mutation or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: collector and its current generation arenas, unchanged.
    /// - `environment` `[in]`: valid environment ID from this collector generation.
    /// - `name` `[in]`: normalized lowercase command name.
    ///
    /// # Returns
    /// Cloned candidate aggregate IDs at the first binding; empty if absent throughout the parent chain.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    ///
    /// # Examples
    /// ```text
    /// let candidates = collector.function(root, "helper");
    /// ```
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
    /// Find or allocate an aggregate variable by owner environment and exact name. Existing records retain their original definition/scope; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    /// - `environment` `[in]`: valid environment ID from this collector generation.
    /// - `name` `[in]`: case-sensitive name copied only for a new record.
    /// - `scope` `[in]`: optional file-local scope stored for a new aggregate.
    /// - `definition` `[in]`: optional source declaration consumed for a new aggregate.
    ///
    /// # Returns
    /// Existing or newly appended aggregate variable ID, not a snapshot-local index.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    ///
    /// # Examples
    /// ```text
    /// let id = collector.variable(root, "answer", None, None);
    /// ```
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
    /// Append a source reference only once per variable/file/command/byte-range/kind key. No I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    /// - `variable` `[in]`: valid aggregate variable index.
    /// - `location` `[in]`: consumed reference; duplicate keys are ignored.
    ///
    /// # Returns
    /// () after inserting into the deduplication set and, if new, the variable reference list.
    ///
    /// # Panics
    /// Panics for a new reference with an out-of-range aggregate variable ID.
    ///
    /// # Examples
    /// ```text
    /// collector.record(variable_id, location);
    /// ```
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
    /// Replay only top-level unconditional source declarations in a functions-only dependency visit. Preserves declaration order without variable or body effects; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    /// - `context` `[in]`: per-visit definition map and entry environment.
    /// - `buffer` `[in]`: immutable declaration/block arenas.
    /// - `command` `[in]`: source command being visited.
    ///
    /// # Returns
    /// The context root environment, whether or not a definition was registered.
    ///
    /// # Examples
    /// ```text
    /// let environment = collector.function_command(&context, &buffer, command);
    /// ```
    pub(crate) fn function_command(
        &mut self,
        context: &FileContext,
        buffer: &SemanticedBuffer,
        command: &SemanticCommandRepresent,
    ) -> ReferenceEnvironmentId {
        if let Some(function) = context
            .definitions
            .get(&command.id)
            .filter(|_| command.closed)
            .and_then(|id| buffer.functions.get(id.index()))
            .filter(|function| {
                function
                    .block
                    .and_then(|block| buffer.blocks.get(block.id as usize))
                    .is_some_and(|block| block.parent.is_none())
            })
        {
            self.define_function(function, context.root);
        }
        context.root
    }
    /// Register a source definition as the current candidate for its lowercase name in one environment. Reuses aggregate identity across visits and initializes new references from the definition; no execution or I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    /// - `function` `[in]`: source function/macro; builtin records without a definition are ignored.
    /// - `environment` `[in]`: valid environment ID from this collector generation.
    ///
    /// # Returns
    /// () after overwriting this environment name binding with one candidate ID.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    ///
    /// # Examples
    /// ```text
    /// collector.define_function(&buffer.functions[0], root);
    /// ```
    fn define_function(
        &mut self,
        function: &SemanticFunction,
        environment: ReferenceEnvironmentId,
    ) {
        let Some(definition) = function.definition.as_ref() else {
            return;
        };
        let key = (definition.file, definition.command);
        let id = if let Some(&id) = self.functions.get(&key) {
            id
        } else {
            let id = IndexedFunctionId::new(self.index.functions.len());
            let mut function = function.clone();
            function.references = vec![definition.clone()];
            self.index.functions.push(function);
            self.functions.insert(key, id);
            id
        };
        self.environments[environment.index()]
            .functions
            .insert(function.name.to_ascii_lowercase(), vec![id]);
    }
    /// Find/create an encountered builtin command identity and append a deduplicated call location. No execution or I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    /// - `buffer` `[in]`: owning file snapshot identity.
    /// - `command` `[in]`: command-name span and command ID to record.
    /// - `name` `[in]`: normalized builtin spelling copied for a new record.
    ///
    /// # Returns
    /// () after ensuring a definition-free builtin function record exists.
    ///
    /// # Examples
    /// ```text
    /// collector.builtin_command(&buffer, command, "set");
    /// ```
    fn builtin_command(
        &mut self,
        buffer: &SemanticedBuffer,
        command: &SemanticCommandRepresent,
        name: &str,
    ) {
        let id = if let Some(&id) = self.builtin_functions.get(name) {
            id
        } else {
            let id = IndexedFunctionId::new(self.index.functions.len());
            self.index.functions.push(SemanticFunction {
                documentation: None,
                name: name.to_owned(),
                kind: FunctionKind::Builtin,
                block: None,
                scope: None,
                definition: None,
                parameters: Vec::new(),
                references: Vec::new(),
            });
            self.builtin_functions.insert(name.to_owned(), id);
            id
        };
        if self.call_seen.insert((id, buffer.id, command.id)) {
            self.index.functions[id.index()]
                .references
                .push(ReferenceLocation {
                    file: buffer.id,
                    command: command.id,
                    span: command.span,
                    kind: ReferenceKind::Call,
                    possible_value: None,
                });
        }
    }
    /// Find or lazily create the reference environment for one source command. May allocate body/branch frames and unknown-valued parameter bindings; does not run a body. No I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    /// - `context` `[in,out]`: appends missing block frames.
    /// - `buffer` `[in]`: immutable block/function/scope arenas.
    /// - `command` `[in]`: source location whose enclosing body is entered.
    ///
    /// # Returns
    /// Current environment ID. Declaration openers use their surrounding environment except foreach, whose opener enters its loop frame.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    ///
    /// # Algorithm
    /// Walk the command block ancestry from outermost to innermost, reusing existing frames. Fork for deferred, local or conditional bodies. Conditional nonlocal frames share parent variable identity ownership. For deferred function/macro bodies, seed literal parameters as unknown-valued definitions.
    ///
    /// # Flow
    /// ```text
    ///  [command ancestry]      [existing frames]
    ///           |                      |
    ///           +----------+-----------+
    ///                      v
    ///           <enter body, not opener?>
    ///                 /             \
    ///          [surrounding]    <frame exists?>
    ///                            /         \
    ///                         [reuse]    [fork]
    ///                            |         |
    ///                            |   [owner/deferred rules]
    ///                            |         |
    ///                            |   [seed parameters]
    ///                            +----+----+
    ///                                 v
    ///                          [environment ID]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let environment = collector.environment(&mut context, &buffer, command);
    /// ```
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
    /// Index one source command in the selected environment, recording reads, candidate calls and possible writes. Merges conditional closers without evaluating conditions or executing user bodies; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    /// - `context` `[in,out]`: creates/removes block frames and accumulates branch alternatives.
    /// - `buffer` `[in]`: immutable file snapshot and arenas.
    /// - `source` `[in]`: matching text for delimiter-free variable-name spans.
    /// - `command` `[in]`: command visited in source/dependency order.
    /// - `invocation` `[in]`: dependency visit identity attached to warnings.
    ///
    /// # Returns
    /// Environment used for this command, including when a closing conditional frame has just been merged.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    /// User-call clock overflow can also panic with overflow checks.
    ///
    /// # Algorithm
    /// Select the environment; save/reset branch state at else/elseif. Read arguments before writes. Record native or user calls and declarations, invalidating older value-state facts for user calls. Apply closed-command writes, then merge a conditional frame on its matched closer.
    ///
    /// # Flow
    /// ```text
    ///  [command]             [visit context]
    ///       |                       |
    ///       +-----------+-----------+
    ///                   v
    ///         [environment / branch reset]
    ///                   |
    ///                   v
    ///              [read bindings]
    ///                   |
    ///             <closed command?>
    ///               /          \
    ///            [skip]   [declaration/call + writes]
    ///               |          |
    ///               +-----+----+
    ///                     v
    ///             <conditional closer?>
    ///                /            \
    ///            [merge]        [keep frame]
    ///                |            |
    ///                +------+-----+
    ///                       v
    ///               [environment ID]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// let environment = collector.command(&mut context, &buffer, source, command, ReferenceInvocationId::new(0));
    /// ```
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
        let functions = self.function(environment, &name);
        // Keep native calls separate from user bindings, including declaration keywords.
        if functions.is_empty()
            && command.schema.as_ref().is_some_and(|schema| {
                matches!(
                    schema.from,
                    super::schema::CommandSchemaFrom::Builtin
                        | super::schema::CommandSchemaFrom::KeyWord
                )
            })
        {
            self.builtin_command(buffer, command, &name);
        }
        if command.closed {
            if let Some(function) = context
                .definitions
                .get(&command.id)
                .map(|index| &buffer.functions[index.index()])
            {
                self.define_function(function, environment);
            } else {
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
    /// Bind an extracted variable read, lazily create builtin roots and append unset/empty warnings. Deferred outer values suppress unsafe warnings; no I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    /// - `reference` `[in]`: extracted name and source read location.
    /// - `environment` `[in]`: valid environment ID from this collector generation.
    /// - `invocation` `[in]`: dependency visit ID for any warning.
    ///
    /// # Returns
    /// () after recording a bound read or appending an unresolved use; warnings are not deduplicated by navigation keys.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    ///
    /// # Algorithm
    /// Look up the nearest binding, creating unknown-valued builtin bindings at the context root when needed. Record the read. Emit unset/empty/possibly-unset state warnings except for outer bindings read by deferred bodies; missing names stay unresolved.
    ///
    /// # Flow
    /// ```text
    ///  [read name]          [visible bindings/builtins]
    ///       |                         |
    ///       +------------+------------+
    ///                    v
    ///               <binding found?>
    ///                 /         \
    ///          [record read] [unresolved + possible warning]
    ///                 |
    ///        <deferred outer value?>
    ///             /           \
    ///        [suppress]    [state warning if known]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// collector.read(&command.variable_references[0], root, ReferenceInvocationId::new(0));
    /// ```
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
    /// Record possible local variable writes and update unset/empty warning states. Cache/environment writes are ignored; PARENT_SCOPE writes stay unresolved rather than mutating parent state. No execution or I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    /// - `buffer` `[in]`: owning file identity.
    /// - `source` `[in]`: source matching argument spans.
    /// - `command` `[in]`: closed command with schema/output classifications.
    /// - `environment` `[in]`: valid environment ID from this collector generation.
    ///
    /// # Returns
    /// () after appending definitions/writes and overwriting local binding entries. An unset may reveal a binding from a different owner.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    ///
    /// # Algorithm
    /// Recognize native assignments and in-place mutations, reject cache/environment effects, then inspect literal output names. Nonlocal writes become unresolved locations. Local writes reuse/create owner-qualified identities; first non-unset writes establish definitions. Infer only simple empty/nonempty set states and let unset expose an inherited owner binding when available.
    ///
    /// # Flow
    /// ```text
    ///  [schema Out / native mutation slots]
    ///                    |
    ///           <cache/environment write?>
    ///                /              \
    ///             [skip]      <PARENT_SCOPE?>
    ///                          /         \
    ///                    [unresolved] [local identity]
    ///                                     |
    ///                           [definition/write location]
    ///                                     |
    ///                               <unset?>
    ///                               /      \
    ///                     [reveal parent] [infer simple state]
    ///                               |      |
    ///                               +--+---+
    ///                                  v
    ///                          [overwrite local map]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// collector.writes(&buffer, source, command, root);
    /// ```
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
    /// Merge conditional alternatives into the surrounding environment conservatively. Appends candidate functions and weakens conflicting variable states; no branch selection or I/O.
    ///
    /// # Arguments
    /// - `self` `[in,out]`: collector state updated as described; document snapshots remain unchanged.
    /// - `frame` `[in]`: consumed conditional frame with accumulated writes/functions.
    ///
    /// # Returns
    /// () after saving the final branch, propagating invalidation and overwriting possible parent bindings.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    ///
    /// # Algorithm
    /// Drain the final branch into accumulated alternatives. Propagate the latest call invalidation. Preserve a write state only when it agrees with a visible parent binding; conflicting states become Unknown, missing parent names MaybeUnset. Extend parent function candidates.
    ///
    /// # Flow
    /// ```text
    ///  [prior branches]        [final branch]
    ///          |                     |
    ///          +----------+----------+
    ///                     v
    ///           [save/merge alternatives]
    ///                     |
    ///             <parent binding?>
    ///              /             \
    ///        [compare states]  [MaybeUnset]
    ///           /       \          |
    ///       [same]   [Unknown]      |
    ///           +-------+----------+
    ///                   v
    ///        [parent maps + invalidation]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// collector.merge(frame);
    /// ```
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
/// Drain one branch into accumulated frame alternatives and reset its local maps. Conflicting write states become Unknown; no I/O.
///
/// # Arguments
/// - `frame` `[in,out]`: appends/merges writes and extends function candidate lists.
/// - `environment` `[in,out]`: drains variables/functions; parent/owner/clock fields are unchanged.
///
/// # Returns
/// () after merging; old writes before invalidation are downgraded to Unknown.
///
/// # Examples
/// ```text
/// save_branch(&mut frame, &mut environment);
/// ```
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
/// Follow parent links to the topmost visibility environment. No side effects.
///
/// # Arguments
/// - `environments` `[in]`: owning collector arena.
/// - `environment` `[in]`: valid starting ID.
///
/// # Returns
/// Root ancestor ID, or the input when already a root.
///
/// # Panics
/// Invalid IDs panic; parent chains must be acyclic.
///
/// # Examples
/// ```text
/// let root = context_root(&collector.environments, child);
/// ```
fn context_root(
    environments: &[Environment],
    mut environment: ReferenceEnvironmentId,
) -> ReferenceEnvironmentId {
    while let Some(parent) = environments[environment.index()].parent {
        environment = parent;
    }
    environment
}

/// Build a standalone file reference index without a dependency graph. Uses one synthetic visit ID and preserves collector environment identities; no I/O or execution.
///
/// # Arguments
/// - `buffer` `[in]`: immutable semantic snapshot.
/// - `source` `[in]`: matching source text for name spans.
///
/// # Returns
/// Completed index for source-order local commands only; external dependency declarations are not replayed.
///
/// # Panics
/// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
///
/// # Examples
/// ```text
/// let index = local_index(&buffer, source);
/// ```
pub(crate) fn local_index(buffer: &SemanticedBuffer, source: &str) -> ReferenceIndex {
    let mut collector = ReferenceCollector::default();
    let root = collector.root();
    let mut context = collector.file(buffer, root);
    for command in &buffer.commands {
        collector.command(
            &mut context,
            buffer,
            source,
            command,
            ReferenceInvocationId::new(0),
        );
    }
    collector.finish()
}
impl Semanticer {
    /// Overwrite local warnings/functions/unresolved uses and rebuild variable reference lists from a standalone index. Removes unknown-command diagnostics for bound user calls; no I/O or execution.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer, unchanged.
    /// - `result` `[in,out]`: replaces warnings/functions/unresolved references, clears and extends variable references, appends newly discovered variables and filters resolved-call diagnostics.
    /// - `source` `[in]`: text matching all source spans.
    ///
    /// # Returns
    /// () after merging aggregate references back into file-local scope/name records; existing possible-write lists are retained.
    ///
    /// # Panics
    /// Panics if an environment or source arena ID does not belong to its owning generation; parent chains must be acyclic.
    ///
    /// # Algorithm
    /// Build the local index, transfer warning/function/unresolved outputs, map aggregate variables back to snapshot-local scope/name keys, then remove CommandNotFound diagnostics whose exact name spans match resolved user calls.
    ///
    /// # Flow
    /// ```text
    ///  [local reference index]       [existing scoped variables]
    ///            |                              |
    ///    [replace warning/function lists]       |
    ///            |                              |
    ///            +---------------+--------------+
    ///                            v
    ///                     <scope/name exists?>
    ///                        /           \
    ///                     [reuse]      [append]
    ///                        |           |
    ///                        +-----+-----+
    ///                              v
    ///                   [extend variable references]
    ///                              |
    ///                              v
    ///                   [filter resolved-call errors]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// analyzer.collect_references(&mut buffer, source);
    /// ```
    pub(crate) fn collect_references(&self, result: &mut SemanticedBuffer, source: &str) {
        let index = local_index(result, source);
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
