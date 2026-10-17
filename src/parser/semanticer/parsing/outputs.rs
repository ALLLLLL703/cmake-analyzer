//! Match schema argument grammars for writes, reads, paths and completion roles.
//!
//! Matching tracks consumed positions and literal-keyword specificity. Equally specific
//! full-call branches retain only agreed bindings; completion combines alternatives.
//! This classifies source slots, never project execution.
// Contains AI-generated documentation additions.
// AI-generated implementation.
mod keywords;
mod trails;
pub(crate) use keywords::completion_keywords;
use trails::TargetTrails;
/// Full-call target-binding intersection tests loaded from the source-named test file. No production behavior.
#[cfg(test)]
#[path = "../../../../tests/unit/parser/semanticer/target_arguments.rs"]
mod target_tests;
use crate::parser::semanticer::{
    model::{
        ArgKind, OutputMatch, ScopeId, SemanticArgForm, SemanticVariable, SemanticedBuffer,
        Semanticer, VariableChanging,
    },
    schema::ArgSchemaKind,
};
use std::collections::{BTreeMap, HashMap};

impl Semanticer {
    /// Record possible schema output writes from scratch matching. Appends missing variable/change records and Out flags without clearing existing lists; no execution or I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer, unchanged.
    /// - `result` `[in,out]`: adds scoped variables, initializes first_write when absent, appends deduplicated changes and Out role flags.
    ///
    /// # Returns
    /// () after matching closed commands with output-bearing schemas.
    ///
    /// # Examples
    /// ```
    /// use cmake_analyzer::parser::semanticer::model::*;
    /// use cmake_analyzer::parser::nparser::model::NParsedBuffer;
    /// let analyzer = Semanticer::new(NParsedBuffer::default(), Default::default());
    /// let mut buffer = SemanticedBuffer::default();
    /// analyzer.track_variable_changes(&mut buffer);
    /// assert!(buffer.variables.is_empty());
    /// ```
    pub fn track_variable_changes(&self, result: &mut SemanticedBuffer) {
        self.track_variable_changes_cached(result, &std::collections::HashSet::new());
    }
    // AI-authored cache path. Scope changes rebuild write lists, but do not change argument matching.
    /// Record possible writes, reusing output-slot classifications for unchanged commands. Scope-dependent variable lists are rebuilt by callers; this pass only appends/initializes missing facts. No I/O.
    ///
    /// # Arguments
    /// - `self` `[in]`: analyzer, unchanged.
    /// - `result` `[in,out]`: appends variables/changes, initializes absent first_write and adds missing Out flags.
    /// - `reused` `[in]`: IDs whose prior Out flags may be reused instead of rematching.
    ///
    /// # Returns
    /// () after filtering dynamic names and arguments that could destroy source-slot correspondence.
    ///
    /// # Panics
    /// Cached and newly matched output indices must index the corresponding command arguments.
    ///
    /// # Algorithm
    /// Build a scoped name lookup, select closed output-bearing commands and either reuse Out indices or match the full argument grammar. Reject unstable names/prior unquoted expansion slots. Add each variable once per scope and deduplicate write locations.
    ///
    /// # Flow
    /// ```text
    ///  [closed command]        [reused IDs]
    ///         |                      |
    ///         +----------+-----------+
    ///                    v
    ///              <cached slots?>
    ///                /        \
    ///           [Out flags] [full match]
    ///                |        |
    ///                +----+---+
    ///                     v
    ///            <literal stable name?>
    ///               /             \
    ///            [skip]       [find/create variable]
    ///                               |
    ///                               v
    ///                   [first write + change + Out]
    /// ```
    ///
    /// # Examples
    /// ```text
    /// analyzer.track_variable_changes_cached(&mut buffer, &reused);
    /// ```
    pub(crate) fn track_variable_changes_cached(
        &self,
        result: &mut SemanticedBuffer,
        reused: &std::collections::HashSet<crate::parser::semanticer::model::CommandId>,
    ) {
        let mut variables: HashMap<Option<ScopeId>, HashMap<String, usize>> = HashMap::new();
        for (id, variable) in result.variables.iter().enumerate() {
            variables
                .entry(variable.scope)
                .or_default()
                .insert(variable.name.clone(), id);
        }
        for command in &mut result.commands {
            if !command.closed {
                continue;
            }
            let Some(schema) = &command.schema else {
                continue;
            };
            if !schema.args.iter().any(|arg| contains_out(&arg.kind)) {
                continue;
            }
            let indices = if reused.contains(&command.id) {
                Some(
                    command
                        .args
                        .iter()
                        .enumerate()
                        .filter_map(|(i, arg)| {
                            arg.kind
                                .iter()
                                .any(|k| matches!(k, ArgKind::Out))
                                .then_some(i)
                        })
                        .collect(),
                )
            } else {
                let arguments: Vec<&str> = command
                    .args
                    .iter()
                    .map(|arg| arg.content.as_str())
                    .collect();
                let roots: Vec<&ArgSchemaKind> = schema.args.iter().map(|arg| &arg.kind).collect();
                sequence(
                    &roots,
                    vec![OutputMatch::default()],
                    &arguments,
                    MatchRole::Out,
                )
                .into_iter()
                .find(|state| state.position == arguments.len())
                .map(|state| state.indices)
            };
            let Some(indices) = indices else {
                continue;
            };
            let variables = variables.entry(command.scope).or_default();
            for index in indices {
                let argument = &command.args[index];
                let name = argument.content.as_str();
                if name.is_empty() {
                    continue;
                }
                if argument.form != SemanticArgForm::Bracket
                    && (name.contains('$')
                        || name.contains('\\')
                        || (argument.form == SemanticArgForm::Unquoted && name.contains(';')))
                {
                    continue;
                }
                if command.args[..index].iter().any(|arg| {
                    arg.form == SemanticArgForm::Unquoted
                        && (arg.content.contains('$') || arg.content.contains(';'))
                }) {
                    continue;
                }
                let variable = match variables.get(name).copied() {
                    Some(id) => id,
                    None => {
                        let id = result.variables.len();
                        result.variables.push(SemanticVariable {
                            name: name.to_owned(),
                            builtin: false,
                            first_write: None,
                            scope: command.scope,
                            changing: Vec::new(),
                            references: Vec::new(),
                        });
                        variables.insert(name.to_owned(), id);
                        id
                    }
                };
                let variable = &mut result.variables[variable];
                variable.first_write.get_or_insert(argument.span);
                if !variable
                    .changing
                    .iter()
                    .any(|change| change.command == command.id && change.span == argument.span)
                {
                    variable.changing.push(VariableChanging {
                        documentation: command.documentation.clone(),
                        command: command.id,
                        span: argument.span,
                    });
                }
                if let Some(arg) = command.args.get_mut(index) {
                    if !arg.kind.iter().any(|kind| matches!(kind, ArgKind::Out)) {
                        arg.kind.push(ArgKind::Out);
                    }
                }
            }
        }
    }
}
/// Check nested schema groups for an output-variable leaf. No side effects.
///
/// # Arguments
/// - `kind` `[in]`: schema node recursively inspected.
///
/// # Returns
/// True when the requested leaf occurs anywhere below this node.
///
/// # Examples
/// ```text
/// assert!(contains_out(&ArgSchemaKind::Out));
/// ```
fn contains_out(kind: &ArgSchemaKind) -> bool {
    match kind {
        ArgSchemaKind::Out => true,
        ArgSchemaKind::Optional(args)
        | ArgSchemaKind::AllOf(args)
        | ArgSchemaKind::Repeat { args, .. }
        | ArgSchemaKind::Grammar { args, .. } => args.iter().any(contains_out),
        ArgSchemaKind::Choice(choices) => choices.iter().flatten().any(contains_out),
        _ => false,
    }
}
/// Which schema slots to retain while advancing grammar states. Full-call roles intersect ambiguous bindings; completion roles union alternatives. Creating a role has no side effects, though target matching appends to its borrowed trail arena.
///
/// [`OutputMatch`] stores consumed position and retained bindings.
///
/// # Examples
/// ```text
/// let role = MatchRole::Out;
/// assert!(!prefix_mode(role));
/// ```
#[derive(Clone, Copy)]
enum MatchRole<'a> {
    /// Schema output-variable slot.
    /// Example: `MatchRole::Out`.
    Out,
    /// Explicit variable-read slot.
    /// Example: `MatchRole::Variable`.
    Variable,
    /// Filesystem role for full-call classification.
    /// Example: `MatchRole::Path(PathKind::Input)`.
    Path(
        /// Required path role. Example: `MatchRole::Path(PathKind::Input)`.
        crate::parser::semanticer::model::PathKind,
    ),
    /// Name-completion role at the active slot.
    /// Example: `MatchRole::Name(NameRole::Target)`.
    Name(
        /// Required completion role. Example: `MatchRole::Name(NameRole::Target)`.
        crate::parser::semanticer::schema::NameRole,
    ),
    /// Proposed literal keyword at the active slot.
    /// Example: `MatchRole::Keyword("PUBLIC")`.
    Keyword(
        /// Candidate literal spelling. Example: `MatchRole::Keyword("PUBLIC")`.
        &'a str,
    ),
    /// Completed-prefix specificity probe without retained slots.
    /// Example: `MatchRole::CompletedPrefix`.
    CompletedPrefix,
    /// Full-call target role with shared history.
    /// Example: `MatchRole::Target(NameRole::Target, &trails)`.
    Target(
        /// Target family to retain. Example: `MatchRole::Target(NameRole::Target, &trails)`.
        crate::parser::semanticer::schema::NameRole,
        /// Arena receiving shared histories. Example: `MatchRole::Target(NameRole::Target, &trails)`.
        &'a TargetTrails,
    ),
}
/// Find name roles reachable at the active unfinished argument slot. Missing required suffixes do not reject an editing prefix; no side effects.
///
/// # Arguments
/// - `schema` `[in]`: command grammar.
/// - `arguments` `[in]`: source-slot payloads including the active prefix as the last item.
///
/// # Returns
/// Supported roles whose strongest matching states include the active slot; empty when there is no active argument.
///
/// # Algorithm
/// Probe each supported name role present in the grammar with prefix matching and collect roles whose completed prefix state records the last source slot.
///
/// # Flow
/// ```text
///  [schema roles]         [argument prefix]
///         |                       |
///         +-----------+-----------+
///                     v
///               [probe each role]
///                     |
///            <active slot retained?>
///                 /          \
///              [offer]      [omit]
///                 |          |
///                 +-----+----+
///                       v
///                  [role list]
/// ```
///
/// # Examples
/// ```text
/// let roles = completion_roles(schema, &["my_"]);
/// ```
pub(crate) fn completion_roles(
    schema: &crate::parser::semanticer::schema::CommandSchema,
    arguments: &[&str],
) -> Vec<crate::parser::semanticer::schema::NameRole> {
    use crate::parser::semanticer::schema::NameRole;
    let Some(position) = arguments.len().checked_sub(1) else {
        return Vec::new();
    };
    let roots: Vec<_> = schema.args.iter().map(|argument| &argument.kind).collect();
    [
        NameRole::Variable,
        NameRole::LanguageType,
        NameRole::Module,
        NameRole::Package,
        NameRole::Target,
        NameRole::Executable,
        NameRole::Library,
        NameRole::CustomTarget,
    ]
    .into_iter()
    .filter(|role| {
        if !roots
            .iter()
            .any(|kind| contains_role(kind, MatchRole::Name(*role)))
        {
            return false;
        }
        sequence(
            &roots,
            vec![OutputMatch::default()],
            arguments,
            MatchRole::Name(*role),
        )
        .iter()
        .any(|state| state.position == arguments.len() && state.indices.contains(&position))
    })
    .collect()
}
/// Check whether a schema can contain any target-name role. No side effects.
///
/// # Arguments
/// - `schema` `[in]`: command grammar to inspect recursively.
///
/// # Returns
/// True for Target, Executable, Library or CustomTarget roles, including nested groups.
///
/// # Examples
/// ```text
/// let relevant = has_target_arguments(schema);
/// ```
pub(crate) fn has_target_arguments(
    schema: &crate::parser::semanticer::schema::CommandSchema,
) -> bool {
    use crate::parser::semanticer::schema::NameRole;
    [
        NameRole::Target,
        NameRole::Executable,
        NameRole::Library,
        NameRole::CustomTarget,
    ]
    .into_iter()
    .any(|role| {
        schema
            .args
            .iter()
            .any(|arg| contains_role(&arg.kind, MatchRole::Name(role)))
    })
}
/// Locate target-role slots agreed by the strongest complete grammar matches. Requires source-slot correspondence; does not expand lists or execute commands.
///
/// # Arguments
/// - `command` `[in]`: command supplying the selected schema.
/// - `arguments` `[in]`: matched text with exactly one item per original source slot.
///
/// # Returns
/// Pairs of zero-based source argument index and target NameRole; empty for absent schema or no full match.
///
/// # Algorithm
/// For each target role, run the full grammar with shared target histories. Equal-specificity states intersect histories at each consumed position; emit the final complete state slots.
///
/// # Flow
/// ```text
///  [target roles]      [schema + source slots]
///        |                       |
///        +-----------+-----------+
///                    v
///            [full grammar match]
///                    |
///           <equal-specificity paths?>
///              /              \
///        [single trail]   [intersect trails]
///              |              |
///              +-------+------+
///                      v
///             [agreed slot/role pairs]
/// ```
///
/// # Examples
/// ```text
/// let slots = target_arguments(command, &["app", "dep"]);
/// ```
pub(crate) fn target_arguments(
    command: &crate::parser::semanticer::model::SemanticCommandRepresent,
    arguments: &[&str],
) -> Vec<(usize, crate::parser::semanticer::schema::NameRole)> {
    use crate::parser::semanticer::schema::NameRole;
    let Some(schema) = &command.schema else {
        return Vec::new();
    };
    let roots: Vec<_> = schema.args.iter().map(|argument| &argument.kind).collect();
    let mut result = Vec::new();
    for kind in [
        NameRole::Target,
        NameRole::Executable,
        NameRole::Library,
        NameRole::CustomTarget,
    ] {
        let trails = TargetTrails::default();
        let role = MatchRole::Target(kind, &trails);
        if !roots.iter().any(|root| contains_role(root, role)) {
            continue;
        }
        if let Some(state) = sequence(&roots, vec![OutputMatch::default()], arguments, role)
            .into_iter()
            .find(|state| state.position == arguments.len())
        {
            result.extend(
                trails
                    .indices(state.trail)
                    .into_iter()
                    .map(|argument| (argument, kind)),
            );
        }
    }
    result
}
/// Classify filesystem argument slots agreed by complete schema matches. No filesystem checks or other I/O.
///
/// # Arguments
/// - `command` `[in]`: schema and original argument payloads.
///
/// # Returns
/// PathArgument entries for File/Directory/Input/Module/Output roles; empty if schema absent or matching fails.
///
/// # Algorithm
/// Probe each supported filesystem role present in the schema. Match the entire source argument list and retain only indices surviving strongest-branch intersection; attach the probed role to each retained slot.
///
/// # Flow
/// ```text
///  [path roles]          [schema + source slots]
///       |                         |
///       +------------+------------+
///                    v
///           <role occurs in schema?>
///               /              \
///            [skip]        [full match]
///                              |
///                     <all slots consumed?>
///                         /          \
///                     [omit]     [emit path slots]
/// ```
///
/// # Examples
/// ```text
/// let paths = path_arguments(command);
/// ```
pub(crate) fn path_arguments(
    command: &crate::parser::semanticer::model::SemanticCommandRepresent,
) -> Vec<crate::parser::semanticer::model::PathArgument> {
    use crate::parser::semanticer::model::{PathArgument, PathKind};
    let Some(schema) = &command.schema else {
        return Vec::new();
    };
    let arguments: Vec<_> = command
        .args
        .iter()
        .map(|arg| arg.content.as_str())
        .collect();
    let roots: Vec<_> = schema.args.iter().map(|arg| &arg.kind).collect();
    let mut result = Vec::new();
    for kind in [
        PathKind::File,
        PathKind::Directory,
        PathKind::Input,
        PathKind::Module,
        PathKind::Output,
    ] {
        let role = MatchRole::Path(kind);
        if !roots.iter().any(|root| contains_role(root, role)) {
            continue;
        }
        if let Some(state) = sequence(&roots, vec![OutputMatch::default()], &arguments, role)
            .into_iter()
            .find(|state| state.position == arguments.len())
        {
            result.extend(
                state
                    .indices
                    .into_iter()
                    .map(|argument| PathArgument { argument, kind }),
            );
        }
    }
    result
}
/// Check whether matching tolerates missing suffixes and records only the active slot. No side effects.
///
/// # Arguments
/// - `role` `[in]`: matching mode.
///
/// # Returns
/// True for Name, Keyword and CompletedPrefix modes; false for full-call binding modes.
///
/// # Examples
/// ```text
/// assert!(prefix_mode(MatchRole::Keyword("PUBLIC")));
/// ```
fn prefix_mode(role: MatchRole<'_>) -> bool {
    matches!(
        role,
        MatchRole::Name(_) | MatchRole::Keyword(_) | MatchRole::CompletedPrefix
    )
}
/// Check whether a schema subtree contains the requested role. No side effects.
///
/// # Arguments
/// - `kind` `[in]`: schema subtree.
/// - `role` `[in]`: requested matching mode.
///
/// # Returns
/// True if this leaf or any nested grammar/group/choice leaf matches the role.
///
/// # Examples
/// ```text
/// assert!(contains_role(&ArgSchemaKind::Out, MatchRole::Out));
/// ```
fn contains_role(kind: &ArgSchemaKind, role: MatchRole) -> bool {
    if matches_role(kind, role) {
        return true;
    }
    match kind {
        ArgSchemaKind::Optional(args)
        | ArgSchemaKind::AllOf(args)
        | ArgSchemaKind::Repeat { args, .. }
        | ArgSchemaKind::Grammar { args, .. } => args.iter().any(|kind| contains_role(kind, role)),
        ArgSchemaKind::Choice(choices) => choices
            .iter()
            .flatten()
            .any(|kind| contains_role(kind, role)),
        _ => false,
    }
}
/// Find explicit variable-read slots, excluding slots also matched as outputs. No side effects.
///
/// # Arguments
/// - `command` `[in]`: source payloads and selected schema.
///
/// # Returns
/// Zero-based input-variable argument slots from a complete match; empty when absent, unmatched or output-only.
///
/// # Algorithm
/// Match the full grammar separately for explicit variable reads and output writes, then remove output positions from the read list. Missing schemas or missing variable roles return immediately.
///
/// # Flow
/// ```text
///  [schema + source slots]
///              |
///     <explicit variable role?>
///         /             \
///      [empty]      [full matching]
///                     /      \
///              [read slots] [output slots]
///                     |      |
///                     +---+--+
///                         v
///                 [reads minus outputs]
/// ```
///
/// # Examples
/// ```text
/// let reads = variable_arguments(command);
/// ```
pub(super) fn variable_arguments(
    command: &crate::parser::semanticer::model::SemanticCommandRepresent,
) -> Vec<usize> {
    let Some(schema) = &command.schema else {
        return Vec::new();
    };
    if !schema.args.iter().any(|arg| contains_variable(&arg.kind)) {
        return Vec::new();
    }
    let arguments: Vec<_> = command
        .args
        .iter()
        .map(|arg| arg.content.as_str())
        .collect();
    let roots: Vec<_> = schema.args.iter().map(|arg| &arg.kind).collect();
    let matched = |role| {
        sequence(&roots, vec![OutputMatch::default()], &arguments, role)
            .into_iter()
            .find(|state| state.position == arguments.len())
            .map_or_else(Vec::new, |state| state.indices)
    };
    let outputs = matched(MatchRole::Out);
    matched(MatchRole::Variable)
        .into_iter()
        .filter(|index| !outputs.contains(index))
        .collect()
}
/// Check nested schema groups for an explicit variable-read leaf. No side effects.
///
/// # Arguments
/// - `kind` `[in]`: schema node recursively inspected.
///
/// # Returns
/// True when the requested leaf occurs anywhere below this node.
///
/// # Examples
/// ```text
/// assert!(contains_variable(&ArgSchemaKind::Variable));
/// ```
fn contains_variable(kind: &ArgSchemaKind) -> bool {
    match kind {
        ArgSchemaKind::Variable => true,
        ArgSchemaKind::Optional(args)
        | ArgSchemaKind::AllOf(args)
        | ArgSchemaKind::Repeat { args, .. }
        | ArgSchemaKind::Grammar { args, .. } => args.iter().any(contains_variable),
        ArgSchemaKind::Choice(choices) => choices.iter().flatten().any(contains_variable),
        _ => false,
    }
}
/// Compare one schema leaf against an output/read/path/name/keyword role. No side effects.
///
/// # Arguments
/// - `kind` `[in]`: schema node; nested groups are not traversed here.
/// - `role` `[in]`: requested role, including literal keyword spelling.
///
/// # Returns
/// True for the supported leaf/role mapping; CompletedPrefix never retains a leaf.
///
/// # Examples
/// ```text
/// assert!(matches_role(&ArgSchemaKind::Out, MatchRole::Out));
/// ```
fn matches_role(kind: &ArgSchemaKind, role: MatchRole) -> bool {
    if let MatchRole::Keyword(candidate) = role {
        return matches!(kind,ArgSchemaKind::Keyword(word) if word==candidate);
    }
    use crate::parser::semanticer::model::PathKind;
    if let MatchRole::Path(path) = role {
        return matches!(
            (kind, path),
            (ArgSchemaKind::InputFile, PathKind::File)
                | (ArgSchemaKind::InputDirectory, PathKind::Directory)
                | (ArgSchemaKind::InputPath, PathKind::Input)
                | (ArgSchemaKind::ModulePath, PathKind::Module)
                | (ArgSchemaKind::OutputPath, PathKind::Output)
        );
    }
    if let MatchRole::Name(role) | MatchRole::Target(role, _) = role {
        use crate::parser::semanticer::schema::NameRole;
        return matches!(
            (kind, role),
            (
                ArgSchemaKind::Variable | ArgSchemaKind::Out | ArgSchemaKind::Unset,
                NameRole::Variable
            ) | (ArgSchemaKind::Target, NameRole::Target)
                | (ArgSchemaKind::TargetExe, NameRole::Executable)
                | (ArgSchemaKind::TargetLib, NameRole::Library)
                | (ArgSchemaKind::CustomTarget(_), NameRole::CustomTarget)
                | (ArgSchemaKind::LanguageType, NameRole::LanguageType)
                | (ArgSchemaKind::ModulePath, NameRole::Module)
                | (ArgSchemaKind::PackageName, NameRole::Package)
        );
    }
    matches!(
        (kind, role),
        (ArgSchemaKind::Out, MatchRole::Out) | (ArgSchemaKind::Variable, MatchRole::Variable)
    )
}
/// Advance match frontiers through a sequence, merging ambiguity after each node. Target mode may append match-local trail nodes; no external effects.
///
/// # Arguments
/// - `kinds` `[in]`: ordered borrowed grammar nodes.
/// - `states` `[in]`: consumed frontier states.
/// - `arguments` `[in]`: source-slot payloads.
/// - `role` `[in]`: role retention and prefix policy; Target borrows a mutable-through-cell arena.
///
/// # Returns
/// Ranked frontier states after the sequence, or empty if every path fails.
///
/// # Algorithm
/// Expand each frontier with step, preserve completed editing prefixes even when suffix grammar remains, merge by position/specificity and stop as soon as no state survives.
///
/// # Flow
/// ```text
///  [grammar node]         [frontier]
///        |                    |
///        +---------+----------+
///                  v
///         <prefix already consumed?>
///              /           \
///           [keep]         [step]
///              |             |
///              +------+------+
///                     v
///              [merge by position]
///                     |
///               <states remain?>
///                  /       \
///            [next node]  [empty return]
/// ```
///
/// # Examples
/// ```text
/// let states = sequence(&[&ArgSchemaKind::Out], vec![OutputMatch::default()], &["result"], MatchRole::Out);
/// ```
fn sequence(
    kinds: &[&ArgSchemaKind],
    mut states: Vec<OutputMatch>,
    arguments: &[&str],
    role: MatchRole,
) -> Vec<OutputMatch> {
    for kind in kinds {
        states = merge(
            states
                .into_iter()
                .flat_map(|state| {
                    // Missing required suffixes do not invalidate an editing prefix.
                    if prefix_mode(role) && state.position == arguments.len() {
                        vec![state]
                    } else {
                        step(kind, state, arguments, role)
                    }
                })
                .collect(),
            role,
        );
        if states.is_empty() {
            break;
        }
    }
    states
}
/// Match an owned-node grammar slice from one frontier state. Target histories may grow locally; no I/O.
///
/// # Arguments
/// - `kinds` `[in]`: ordered grammar node slice.
/// - `state` `[in]`: starting frontier consumed by matching.
/// - `arguments` `[in]`: source-slot payloads.
/// - `role` `[in]`: retention/prefix mode and optional trail arena.
///
/// # Returns
/// Merged surviving frontier states after matching the group.
///
/// # Examples
/// ```text
/// let states = group(&[ArgSchemaKind::Out], OutputMatch::default(), &["result"], MatchRole::Out);
/// ```
fn group(
    kinds: &[ArgSchemaKind],
    state: OutputMatch,
    arguments: &[&str],
    role: MatchRole,
) -> Vec<OutputMatch> {
    sequence(
        &kinds.iter().collect::<Vec<_>>(),
        vec![state],
        arguments,
        role,
    )
}
/// Expand one grammar node into possible match states. Optional/choice/repeat branches preserve alternatives; target histories mutate only their match-local arena.
///
/// # Arguments
/// - `kind` `[in]`: grammar node to consume.
/// - `state` `[in]`: incoming frontier, moved into alternatives.
/// - `arguments` `[in]`: source-slot payloads.
/// - `role` `[in]`: requested role and editing/full-call policy.
///
/// # Returns
/// Surviving states; empty for missing/mismatched required arguments. Repeats only retain paths that consume input.
///
/// # Algorithm
/// Dispatch groups recursively. Optional/choice merge alternatives; repeat grows a bounded frontier with strict position progress. Opaque grammar may consume any remaining length. Keyword/cache selectors add specificity evidence; ordinary leaves consume one slot and record the selected role.
///
/// # Flow
/// ```text
///  [schema node]        [incoming state]
///        |                     |
///        +----------+----------+
///                   v
///              <node family?>
///            /       |         \
///        [group]  [repeat]    [leaf]
///           |        |       /    \
///       [recurse] [progress] [fail] [consume]
///           |        |              |
///           +--------+--------------+
///                    v
///         [merged alternatives / slots]
/// ```
///
/// # Examples
/// ```text
/// let states = step(&ArgSchemaKind::Out, OutputMatch::default(), &["result"], MatchRole::Out);
/// ```
fn step(
    kind: &ArgSchemaKind,
    state: OutputMatch,
    arguments: &[&str],
    role: MatchRole,
) -> Vec<OutputMatch> {
    match kind {
        ArgSchemaKind::Grammar { args, .. } => group(args, state, arguments, role),
        ArgSchemaKind::Optional(args) => {
            let mut states = group(args, state.clone(), arguments, role);
            states.push(state);
            merge(states, role)
        }
        ArgSchemaKind::Choice(choices) => merge(
            choices
                .iter()
                .flat_map(|choice| group(choice, state.clone(), arguments, role))
                .collect(),
            role,
        ),
        ArgSchemaKind::Repeat { args, min } => {
            let mut frontier = vec![state];
            let mut matches = Vec::new();
            for count in 0..=arguments.len() {
                if count >= *min {
                    matches.extend(frontier.iter().cloned());
                } else if prefix_mode(role) {
                    matches.extend(
                        frontier
                            .iter()
                            .filter(|state| state.position == arguments.len())
                            .cloned(),
                    );
                }
                let next = frontier
                    .into_iter()
                    .flat_map(|state| {
                        let position = state.position;
                        group(args, state, arguments, role)
                            .into_iter()
                            .filter(move |next| next.position > position)
                    })
                    .collect();
                frontier = merge(next, role);
                if frontier.is_empty() {
                    break;
                }
            }
            merge(matches, role)
        }
        ArgSchemaKind::Unmodeled => (state.position..=arguments.len())
            .map(|position| OutputMatch {
                position,
                ..state.clone()
            })
            .collect(),
        ArgSchemaKind::Keyword(word) => {
            if let MatchRole::Keyword(candidate) = role {
                if state.position + 1 == arguments.len() {
                    if word != candidate {
                        return Vec::new();
                    }
                    let mut next = state;
                    next.indices.push(next.position);
                    next.position += 1;
                    // Only completed arguments rank branches; a proposed keyword is not evidence.
                    return vec![next];
                }
            }
            let current = arguments.get(state.position).copied();
            let active =
                matches!(role, MatchRole::Name(_)) && state.position + 1 == arguments.len();
            if !current
                .is_some_and(|argument| argument == word || active && word.starts_with(argument))
            {
                return Vec::new();
            }
            vec![OutputMatch {
                position: state.position + 1,
                keywords: state.keywords + usize::from(current == Some(word.as_str())),
                ..state
            }]
        }
        ArgSchemaKind::AllOf(kinds) => {
            if state.position >= arguments.len() {
                return Vec::new();
            }
            let current = if state.position + 1 == arguments.len() {
                if let MatchRole::Keyword(candidate) = role {
                    candidate
                } else {
                    arguments[state.position]
                }
            } else {
                arguments[state.position]
            };
            if kinds
                .iter()
                .any(|kind| matches!(kind,ArgSchemaKind::Keyword(word) if current!=word))
            {
                return Vec::new();
            }
            let mut next = state;
            if (!prefix_mode(role) || next.position + 1 == arguments.len())
                && kinds.iter().any(|kind| match role {
                    MatchRole::Out => contains_out(kind),
                    MatchRole::Variable => contains_variable(kind),
                    MatchRole::Path(_)
                    | MatchRole::Name(_)
                    | MatchRole::Target(_, _)
                    | MatchRole::Keyword(_) => contains_role(kind, role),
                    MatchRole::CompletedPrefix => false,
                })
            {
                record(&mut next, role);
            }
            next.position += 1;
            vec![next]
        }
        _ => {
            if state.position >= arguments.len() {
                return Vec::new();
            }
            if matches!(kind, ArgSchemaKind::CacheEntry)
                && !arguments[state.position]
                    .strip_prefix("CACHE{")
                    .is_some_and(|tail| tail.ends_with('}') && tail.len() > 1)
            {
                return Vec::new();
            }
            let mut next = state;
            if matches!(kind, ArgSchemaKind::CacheEntry) {
                next.keywords += 1;
            }
            if matches_role(kind, role)
                && (!prefix_mode(role) || next.position + 1 == arguments.len())
            {
                record(&mut next, role);
            }
            next.position += 1;
            vec![next]
        }
    }
}
/// Retain the current argument slot as a role binding. Target mode appends a trail node; other modes append to state.indices. No I/O.
///
/// # Arguments
/// - `state` `[in,out]`: overwrites trail or appends indices; position is unchanged.
/// - `role` `[in]`: selects shared target history or direct index storage.
///
/// # Returns
/// () after retaining the current slot.
///
/// # Examples
/// ```text
/// let mut state = OutputMatch::default();
/// record(&mut state, MatchRole::Out);
/// assert_eq!(state.indices, vec![0]);
/// ```
fn record(state: &mut OutputMatch, role: MatchRole<'_>) {
    if let MatchRole::Target(_, trails) = role {
        state.trail = trails.push(state.trail, state.position);
    } else {
        state.indices.push(state.position);
    }
}
/// Rank states per consumed position and combine equally specific bindings. Target intersections may append match-local trail nodes; no I/O.
///
/// # Arguments
/// - `states` `[in]`: competing frontier states consumed by merging.
/// - `role` `[in]`: prefix modes union slots; full binding modes intersect slots or trails.
///
/// # Returns
/// One state per position in ascending order, favoring more exact keyword evidence.
///
/// # Algorithm
/// Keep the greatest keyword count at each consumed position. On ties union active completion alternatives, intersect full-call target histories or intersect ordinary full-call indices.
///
/// # Flow
/// ```text
///  [competing states]
///          |
///  [group by consumed position]
///          |
///    <keyword evidence?>
///     /        |        \
///  [higher]  [equal]   [lower]
///     |        |         |
///  [replace] <prefix?> [discard]
///            /      \
///        [union] [intersection]
///            |      |
///            +--+---+
///               v
///         [ordered frontier]
/// ```
///
/// # Examples
/// ```text
/// let states = merge(vec![OutputMatch::default()], MatchRole::Out);
/// assert_eq!(states.len(), 1);
/// ```
fn merge(states: Vec<OutputMatch>, role: MatchRole) -> Vec<OutputMatch> {
    let mut positions: BTreeMap<usize, OutputMatch> = BTreeMap::new();
    for state in states {
        match positions.get_mut(&state.position) {
            Some(old) if old.keywords < state.keywords => *old = state,
            Some(old) if old.keywords == state.keywords => {
                // Completion offers alternatives; binding requires agreement instead.
                if prefix_mode(role) {
                    for index in state.indices {
                        if !old.indices.contains(&index) {
                            old.indices.push(index);
                        }
                    }
                } else if let MatchRole::Target(_, trails) = role {
                    old.trail = trails.intersect(old.trail, state.trail);
                } else {
                    old.indices.retain(|index| state.indices.contains(index))
                }
            }
            Some(_) => {}
            None => {
                positions.insert(state.position, state);
            }
        }
    }
    positions.into_values().collect()
}
