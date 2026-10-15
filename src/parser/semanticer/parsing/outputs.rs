// AI-generated implementation.
use crate::parser::semanticer::{
    model::{
        ArgKind, OutputMatch, ScopeId, SemanticArgForm, SemanticVariable, SemanticedBuffer,
        Semanticer, VariableChanging,
    },
    schema::ArgSchemaKind,
};
use std::collections::{BTreeMap, HashMap};

impl Semanticer {
    pub fn track_variable_changes(&self, result: &mut SemanticedBuffer) {
        self.track_variable_changes_cached(result, &std::collections::HashSet::new());
    }
    // AI-authored cache path. Scope changes rebuild write lists, but do not change argument matching.
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
                    vec![OutputMatch {
                        position: 0,
                        indices: Vec::new(),
                        keywords: 0,
                    }],
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
fn contains_out(kind: &ArgSchemaKind) -> bool {
    match kind {
        ArgSchemaKind::Out => true,
        ArgSchemaKind::Optional(args)
        | ArgSchemaKind::AllOf(args)
        | ArgSchemaKind::Repeat { args, .. } => args.iter().any(contains_out),
        ArgSchemaKind::Choice(choices) => choices.iter().flatten().any(contains_out),
        _ => false,
    }
}
#[derive(Clone, Copy)]
enum MatchRole {
    Out,
    Variable,
    Path(crate::parser::semanticer::model::PathKind),
}
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
        if let Some(state) = sequence(
            &roots,
            vec![OutputMatch {
                position: 0,
                indices: Vec::new(),
                keywords: 0,
            }],
            &arguments,
            role,
        )
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
fn contains_role(kind: &ArgSchemaKind, role: MatchRole) -> bool {
    if matches_role(kind, role) {
        return true;
    }
    match kind {
        ArgSchemaKind::Optional(args)
        | ArgSchemaKind::AllOf(args)
        | ArgSchemaKind::Repeat { args, .. } => args.iter().any(|kind| contains_role(kind, role)),
        ArgSchemaKind::Choice(choices) => choices
            .iter()
            .flatten()
            .any(|kind| contains_role(kind, role)),
        _ => false,
    }
}
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
        sequence(
            &roots,
            vec![OutputMatch {
                position: 0,
                indices: Vec::new(),
                keywords: 0,
            }],
            &arguments,
            role,
        )
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
fn contains_variable(kind: &ArgSchemaKind) -> bool {
    match kind {
        ArgSchemaKind::Variable => true,
        ArgSchemaKind::Optional(args)
        | ArgSchemaKind::AllOf(args)
        | ArgSchemaKind::Repeat { args, .. } => args.iter().any(contains_variable),
        ArgSchemaKind::Choice(choices) => choices.iter().flatten().any(contains_variable),
        _ => false,
    }
}
fn matches_role(kind: &ArgSchemaKind, role: MatchRole) -> bool {
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
    matches!(
        (kind, role),
        (ArgSchemaKind::Out, MatchRole::Out) | (ArgSchemaKind::Variable, MatchRole::Variable)
    )
}
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
                .flat_map(|state| step(kind, state, arguments, role))
                .collect(),
        );
        if states.is_empty() {
            break;
        }
    }
    states
}
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
fn step(
    kind: &ArgSchemaKind,
    state: OutputMatch,
    arguments: &[&str],
    role: MatchRole,
) -> Vec<OutputMatch> {
    match kind {
        ArgSchemaKind::Optional(args) => {
            let mut states = group(args, state.clone(), arguments, role);
            states.push(state);
            merge(states)
        }
        ArgSchemaKind::Choice(choices) => merge(
            choices
                .iter()
                .flat_map(|choice| group(choice, state.clone(), arguments, role))
                .collect(),
        ),
        ArgSchemaKind::Repeat { args, min } => {
            let mut frontier = vec![state];
            let mut matches = Vec::new();
            for count in 0..=arguments.len() {
                if count >= *min {
                    matches.extend(frontier.iter().cloned());
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
                frontier = merge(next);
                if frontier.is_empty() {
                    break;
                }
            }
            merge(matches)
        }
        ArgSchemaKind::Unmodeled => (state.position..=arguments.len())
            .map(|position| OutputMatch {
                position,
                ..state.clone()
            })
            .collect(),
        ArgSchemaKind::Keyword(word) => {
            if arguments.get(state.position).copied() != Some(word.as_str()) {
                return Vec::new();
            }
            vec![OutputMatch {
                position: state.position + 1,
                keywords: state.keywords + 1,
                ..state
            }]
        }
        ArgSchemaKind::AllOf(kinds) => {
            if state.position >= arguments.len() {
                return Vec::new();
            }
            if kinds.iter().any(|kind|matches!(kind,ArgSchemaKind::Keyword(word) if arguments[state.position]!=word)){return Vec::new()}
            let mut next = state;
            if kinds.iter().any(|kind| match role {
                MatchRole::Out => contains_out(kind),
                MatchRole::Variable => contains_variable(kind),
                MatchRole::Path(_) => contains_role(kind, role),
            }) {
                next.indices.push(next.position);
            }
            next.position += 1;
            vec![next]
        }
        _ => {
            if state.position >= arguments.len() {
                return Vec::new();
            }
            let mut next = state;
            if matches_role(kind, role) {
                next.indices.push(next.position);
            }
            next.position += 1;
            vec![next]
        }
    }
}
fn merge(states: Vec<OutputMatch>) -> Vec<OutputMatch> {
    let mut positions: BTreeMap<usize, OutputMatch> = BTreeMap::new();
    for state in states {
        match positions.get_mut(&state.position) {
            Some(old) if old.keywords < state.keywords => *old = state,
            Some(old) if old.keywords == state.keywords => {
                old.indices.retain(|index| state.indices.contains(index))
            }
            Some(_) => {}
            None => {
                positions.insert(state.position, state);
            }
        }
    }
    positions.into_values().collect()
}
