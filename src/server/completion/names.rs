use super::{
    model::{ArgumentChoices, Candidate, Input, Kind},
    roles,
};
use crate::{
    parser::{
        build_struct::model::{BuildEffect, TargetKind},
        semanticer::{
            model::{BlockId, ScopeId, SemanticCommandRepresent},
            schema::NameRole,
        },
    },
    workspace::model::DocumentSnapshot,
};
use lspally::server::model::{Cancellation, ServerError, ServerResult};
use std::collections::BTreeMap;

type Names<'a> = BTreeMap<(&'a str, Kind), Candidate>;
fn add<'a>(names: &mut Names<'a>, name: &'a str, prefix: &str, kind: Kind) {
    if !name.is_empty()
        && name.starts_with(prefix)
        && !name.chars().any(|character| {
            character.is_whitespace()
                || matches!(
                    character,
                    '$' | '{' | '}' | '\\' | ';' | '"' | '<' | '>' | '(' | ')'
                )
        })
    {
        names.entry((name, kind)).or_insert_with(|| Candidate {
            name: name.to_owned(),
            kind,
            parameters: None,
            source_defined: false,
        });
    }
}
fn scope_visible(
    snapshot: &DocumentSnapshot,
    mut current: Option<ScopeId>,
    wanted: Option<ScopeId>,
) -> bool {
    let Some(wanted) = wanted else { return true };
    while let Some(scope) = current {
        if scope == wanted {
            return true;
        }
        current = snapshot
            .semantics
            .scopes
            .get(scope.id as usize)
            .and_then(|scope| scope.parent);
    }
    false
}
fn block_visible(
    snapshot: &DocumentSnapshot,
    mut current: Option<BlockId>,
    wanted: BlockId,
) -> bool {
    while let Some(block) = current {
        if block == wanted {
            return true;
        }
        current = snapshot
            .semantics
            .blocks
            .get(block.id as usize)
            .and_then(|block| block.parent);
    }
    false
}
fn variables<'a>(
    input: &'a Input,
    command: &SemanticCommandRepresent,
    prefix: &str,
    out: &mut Names<'a>,
    cancel: &Cancellation,
) -> ServerResult<()> {
    for variable in &input.snapshot.semantics.variables {
        if cancel.is_cancelled() {
            return Err(ServerError::cancelled());
        }
        if scope_visible(&input.snapshot, command.scope, variable.scope) {
            add(out, &variable.name, prefix, Kind::Variable);
        }
    }
    for function in &input.snapshot.semantics.functions {
        if function
            .block
            .is_some_and(|block| block_visible(&input.snapshot, command.block, block))
        {
            for parameter in &function.parameters {
                add(out, &parameter.name, prefix, Kind::Variable);
            }
        }
    }
    for source in input.sources.iter().filter(|source| {
        source.variables_visible && source.snapshot.semantics.id != input.snapshot.semantics.id
    }) {
        // Specialized builtin descriptors can also carry ordinary source writes.
        for variable in &source.snapshot.semantics.variables {
            if cancel.is_cancelled() {
                return Err(ServerError::cancelled());
            }
            if variable.scope.is_none() {
                add(out, &variable.name, prefix, Kind::Variable);
            }
        }
    }
    Ok(())
}
fn targets<'a>(
    input: &Input,
    snapshot: &'a DocumentSnapshot,
    visible: bool,
    accepted: &[NameRole],
    prefix: &str,
    out: &mut Names<'a>,
    cancel: &Cancellation,
) -> ServerResult<()> {
    for fact in &snapshot.build.facts {
        if cancel.is_cancelled() {
            return Err(ServerError::cancelled());
        }
        let BuildEffect::Target(target) = &fact.effect else {
            continue;
        };
        if !target.name.literal
            || target.imported && !target.global && !visible
            || roles::shadowed(input, fact.source.command, snapshot.semantics.id)
        {
            continue;
        }
        let kind = if target.kind == TargetKind::Alias {
            if snapshot
                .semantics
                .commands
                .get(fact.source.command.id as usize)
                .is_some_and(|command| command.name.eq_ignore_ascii_case("add_executable"))
            {
                TargetKind::Executable
            } else {
                TargetKind::Library
            }
        } else {
            target.kind
        };
        if accepted.iter().any(|role| match role {
            NameRole::Target => true,
            NameRole::Executable => kind == TargetKind::Executable,
            NameRole::CustomTarget => kind == TargetKind::Custom,
            NameRole::Library => matches!(
                kind,
                TargetKind::Library
                    | TargetKind::StaticLibrary
                    | TargetKind::SharedLibrary
                    | TargetKind::ModuleLibrary
                    | TargetKind::ObjectLibrary
                    | TargetKind::InterfaceLibrary
                    | TargetKind::UnknownLibrary
            ),
            NameRole::Variable | NameRole::LanguageType => false,
        }) {
            add(out, &target.name.text, prefix, Kind::EnumMember);
        }
    }
    Ok(())
}
pub(super) fn collect(
    inputs: &[Input],
    offset: usize,
    prefix: &str,
    expansion: bool,
    cancel: &Cancellation,
) -> ServerResult<Vec<Candidate>> {
    let mut out = BTreeMap::new();
    for input in inputs {
        if cancel.is_cancelled() {
            return Err(ServerError::cancelled());
        }
        let Some(argument) = super::argument_context::argument(&input.snapshot, offset) else {
            continue;
        };
        let choices = if expansion {
            ArgumentChoices {
                roles: vec![NameRole::Variable],
                keywords: Vec::new(),
            }
        } else {
            roles::choices(input, &argument, prefix, cancel)
        };
        for keyword in choices.keywords {
            add(&mut out, keyword, prefix, Kind::Keyword);
        }
        let accepted = choices.roles;
        if accepted.contains(&NameRole::Variable) {
            variables(input, argument.command, prefix, &mut out, cancel)?;
        }
        if accepted.contains(&NameRole::LanguageType) {
            for language in crate::parser::semanticer::schema::languages::supported(
                input.snapshot.semantics.version,
            ) {
                add(&mut out, language, prefix, Kind::EnumMember);
            }
            if argument.command.name.eq_ignore_ascii_case("project") {
                add(&mut out, "NONE", prefix, Kind::EnumMember);
            }
        }
        if accepted
            .iter()
            .any(|role| !matches!(role, NameRole::Variable | NameRole::LanguageType))
        {
            targets(
                input,
                &input.snapshot,
                true,
                &accepted,
                prefix,
                &mut out,
                cancel,
            )?;
            for source in &input.sources {
                if source.snapshot.semantics.id != input.snapshot.semantics.id {
                    targets(
                        input,
                        &source.snapshot,
                        source.variables_visible,
                        &accepted,
                        prefix,
                        &mut out,
                        cancel,
                    )?;
                }
            }
        }
    }
    if cancel.is_cancelled() {
        return Err(ServerError::cancelled());
    }
    Ok(out.into_values().collect())
}
