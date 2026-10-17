//! Collect schema-gated argument names, not proven reference bindings.
//! Variables, targets, languages and discovered names remain plain-text candidates. No I/O occurs here.

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

/// Case-sensitive label/kind keys borrow source names while values own rendered candidates.
/// Example: `let names: Names<'_> = BTreeMap::new();` creates an empty collection.
type Names<'a> = BTreeMap<(&'a str, Kind), Candidate>;
/// Insert a safe case-sensitive argument spelling if absent; no I/O.
///
/// # Arguments
/// - `names` `[in,out]`: Candidate map; appends a missing (label, kind) entry without overwriting an existing one.
/// - `name` `[in]`: Borrowed spelling; empty, whitespace, expression, list or delimiter syntax is rejected.
/// - `prefix` `[in]`: Case-sensitive typed prefix.
/// - `kind` `[in]`: Presentation category, also part of the deduplication key.
///
/// # Returns
/// `()`: matching safe spellings become plain-text, non-source-defined candidates.
///
/// # Examples
/// ```text
/// add(&mut names, "MY_VAR", "MY_", Kind::Variable);
/// ```
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
/// Test lexical variable-scope ancestry within one snapshot; no side effects.
///
/// # Arguments
/// - `snapshot` `[in]`: Scope arena containing both IDs.
/// - `current` `[in]`: Active scope to walk upward; local traversal does not change the caller's value.
/// - `wanted` `[in]`: Candidate scope; None denotes globally visible descriptors.
///
/// # Returns
/// `true` for globals or a matching current/ancestor scope; `false` for unrelated or unavailable scopes.
///
/// # Examples
/// ```text
/// assert!(scope_visible(&snapshot, command.scope, None));
/// ```
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
/// Test whether a function/macro body encloses the cursor's block; no side effects.
///
/// # Arguments
/// - `snapshot` `[in]`: Block arena qualifying both IDs.
/// - `current` `[in]`: Active block whose parent chain is read; caller state is unchanged.
/// - `wanted` `[in]`: Function/macro body block sought in that chain.
///
/// # Returns
/// `true` if wanted is current or ancestral; `false` if the chain ends or an ID is unavailable.
///
/// # Examples
/// ```text
/// let inside_body = block_visible(&snapshot, command.block, body);
/// ```
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
/// Append possible variable and enclosing formal names; no I/O or binding changes.
///
/// # Arguments
/// - `input` `[in]`: Owner snapshot plus directory-qualified dependency sources.
/// - `command` `[in]`: Active command providing lexical scope/block.
/// - `prefix` `[in]`: Case-sensitive variable-name prefix.
/// - `out` `[in,out]`: Map receiving missing variable candidates; existing entries remain unchanged.
/// - `cancel` `[in]`: Stop signal checked while walking variable inventories.
///
/// # Returns
/// `Ok(())` after adding visible candidates; descriptors are possible names, not proof of initialization.
///
/// # Errors
/// `Err` reports cancellation; entries appended before cancellation remain in out.
///
/// # Algorithm
/// Use current/ancestor lexical scopes, then formal parameters from enclosing function/macro blocks. Add only global-scope variables from distinct dependency snapshots marked directory-visible.
///
/// # Flow
/// ```text
///  [Local variables]         [Enclosing function formals]
///         |                             |
///  [scope ancestry]              [block ancestry]
///         +-------------+---------------+
///                       v
///                 [Candidate map] <---- [Ancestor/include globals]
///                       |
///                       v
///              [Possible variable names]
/// ```
///
/// # Examples
/// ```text
/// variables(input, slot.command, "MY_", &mut names, &cancel)?;
/// ```
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
/// Append literal target-name candidates supported by active schema roles.
/// No I/O or reference binding occurs.
///
/// # Arguments
/// - `input` `[in]`: Owner context used to reject source-shadowed declaration commands.
/// - `snapshot` `[in]`: Source target facts; candidate spellings borrow from this snapshot.
/// - `visible` `[in]`: Whether nonglobal imports from this source directory may be offered.
/// - `accepted` `[in]`: Generic or executable/library/custom-target schema roles.
/// - `prefix` `[in]`: Case-sensitive target prefix.
/// - `out` `[in,out]`: Map receiving missing EnumMember candidates; never clears existing entries.
/// - `cancel` `[in]`: Stop signal checked per source fact.
///
/// # Returns
/// `Ok(())` after considering literal declarations, including aliases under their declaration class.
///
/// # Errors
/// `Err` reports cancellation; already appended candidates remain in out.
///
/// # Algorithm
/// Skip dynamic names, invisible nonglobal imports and declarations whose native command is shadowed. Classify executable/library aliases by their declaration command, then filter target kind against schema roles. This is candidate discovery, not the stricter unique-name binding used for navigation.
///
/// # Flow
/// ```text
///  [Source build facts]
///          |
///          v
///  <literal + visible + native?> --no--> [Skip]
///          | yes
///          v
///  [Classify declaration/alias]
///          |
///  <role accepts class?> --no--> [Skip]
///          | yes
///          v
///  [Deduplicated EnumMember candidate]
/// ```
///
/// # Examples
/// ```text
/// targets(input, &input.snapshot, true, &accepted, "app", &mut names, &cancel)?;
/// ```
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
            NameRole::Variable | NameRole::LanguageType | NameRole::Module | NameRole::Package => {
                false
            }
        }) {
            add(out, &target.name.text, prefix, Kind::EnumMember);
        }
    }
    Ok(())
}
/// Collect plain-text argument choices allowed by schemas or ordinary variable expansions.
/// No I/O and no reference/index mutation.
///
/// # Arguments
/// - `inputs` `[in]`: Owner-qualified snapshots and preloaded discovery catalogues.
/// - `offset` `[in]`: Validated byte cursor shared by snapshots of the queried open text.
/// - `prefix` `[in]`: Case-sensitive active spelling prefix.
/// - `expansion` `[in]`: True for ordinary ${...} name mode: variables only, never schema keywords.
/// - `cancel` `[in]`: Stop signal checked per owner, inventories and before returning.
///
/// # Returns
/// `Ok(candidates)` sorted/deduplicated by case-sensitive (label, kind), with no snippets; empty when no safe role/site is available.
///
/// # Errors
/// `Err` reports cooperative cancellation; partial local results are discarded.
///
/// # Algorithm
/// Relocate each owner's argument. Expansion mode supplies only Variable; otherwise probe schema choices. Merge keywords, scoped variables, discovered module/package names, version-supported languages (plus project NONE), and role-filtered targets from local, supplemental and graph snapshots. Static/runtime visibility is not proven by these suggestions.
///
/// # Flow
/// ```text
///  [Owner argument site]
///           |
///      <ordinary expansion?>
///       /                \
///  [Variable role]    [Schema choices]
///       |                |
///       +-------+--------+
///               v
///        [Role-gated inventories]
///          /         |          \
///  [variables] [keywords/names] [targets]
///          +---------+----------+
///                    v
///          [Case-sensitive label/kind map]
///                    |
///                    v
///            [Plain-text candidates]
/// ```
///
/// # Examples
/// ```text
/// let choices = names::collect(&inputs, offset, "MY_", true, &cancel)?;
/// ```
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
        if accepted.contains(&NameRole::Module) {
            for name in input.catalog.modules.iter() {
                add(&mut out, name, prefix, Kind::Module);
            }
        }
        if accepted.contains(&NameRole::Package) {
            for name in input.catalog.packages.iter() {
                add(&mut out, name, prefix, Kind::Module);
            }
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
        if accepted.iter().any(|role| {
            matches!(
                role,
                NameRole::Target
                    | NameRole::Executable
                    | NameRole::Library
                    | NameRole::CustomTarget
            )
        }) {
            targets(
                input,
                &input.snapshot,
                true,
                &accepted,
                prefix,
                &mut out,
                cancel,
            )?;
            for source in &input.catalog.targets {
                targets(
                    input,
                    &source.snapshot,
                    source.visible,
                    &accepted,
                    prefix,
                    &mut out,
                    cancel,
                )?;
            }
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
