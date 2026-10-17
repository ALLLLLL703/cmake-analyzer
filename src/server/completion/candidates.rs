//! Merge versioned command schemas and source declarations into possible command names.
//! Source shadows win; candidates do not establish bindings. No I/O occurs.

use super::{
    arguments,
    context::identifier,
    model::{Candidate, Input, Kind, Parameter},
};
use crate::parser::semanticer::{
    model::SemanticFunction,
    schema::{CommandSchema, CommandSchemaFrom, builtin},
};
use lspally::server::model::{Cancellation, ServerError, ServerResult};
use std::collections::{BTreeMap, btree_map::Entry};

/// Classify a command schema for display; no side effects.
///
/// # Arguments
/// - `schema` `[in]`: Provenance and command spelling; build-command classification applies only to native schemas.
///
/// # Returns
/// Keyword for control schemas, Module for selected native build commands, Function otherwise.
///
/// # Examples
/// ```text
/// let category = kind(schema);
/// ```
fn kind(schema: &CommandSchema) -> Kind {
    if matches!(schema.from, CommandSchemaFrom::KeyWord) {
        return Kind::Keyword;
    }
    if matches!(schema.from, CommandSchemaFrom::Builtin) && build_command(&schema.command_name) {
        Kind::Module
    } else {
        Kind::Function
    }
}
/// Classify native build/module commands using spelling conventions and an explicit list.
/// No side effects; this is presentation, not semantic command dispatch.
///
/// # Arguments
/// - `name` `[in]`: Canonical native command spelling; matching is case-sensitive.
///
/// # Returns
/// `true` for recognized build/module families and listed names; `false` for other names.
///
/// # Examples
/// ```text
/// assert!(build_command("target_sources"));
/// assert!(!build_command("message"));
/// ```
fn build_command(name: &str) -> bool {
    name.starts_with("add_")
        || name.starts_with("target_")
        || name.starts_with("ctest_")
        || name.starts_with("find_")
        || name.starts_with("install_")
        || name.starts_with("export_")
        || matches!(
            name,
            "project"
                | "enable_language"
                | "enable_testing"
                | "include"
                | "include_directories"
                | "include_external_msproject"
                | "include_regular_expression"
                | "cmake_file_api"
                | "define_property"
                | "get_source_file_property"
                | "get_target_property"
                | "get_test_property"
                | "load_cache"
                | "fltk_wrap_ui"
                | "qt_wrap_cpp"
                | "qt_wrap_ui"
                | "output_required_files"
                | "use_mangled_mesa"
                | "utility_source"
                | "link_directories"
                | "link_libraries"
                | "remove_definitions"
                | "install"
                | "export"
                | "configure_file"
                | "aux_source_directory"
                | "create_test_sourcelist"
                | "build_command"
                | "build_name"
                | "try_compile"
                | "try_run"
                | "set_directory_properties"
                | "set_source_files_properties"
                | "set_target_properties"
                | "set_tests_properties"
                | "source_group"
                | "subdirs"
                | "subdir_depends"
        )
}
/// Check command identifier validity and ASCII-case-insensitive prefix; no side effects.
///
/// # Arguments
/// - `name` `[in]`: Full command spelling.
/// - `prefix` `[in]`: Typed command prefix, possibly empty.
///
/// # Returns
/// `true` for a valid identifier starting with the prefix ignoring ASCII case; `false` otherwise.
///
/// # Examples
/// ```text
/// assert!(matches("My_Command", "my_"));
/// ```
fn matches(name: &str, prefix: &str) -> bool {
    identifier(name)
        && name
            .get(..prefix.len())
            .is_some_and(|name| name.eq_ignore_ascii_case(prefix))
}
/// Insert a matching schema candidate under a lowercase key; no I/O.
///
/// # Arguments
/// - `schema` `[in]`: Native or external schema supplying spelling, kind and required parameters.
/// - `prefix` `[in]`: Case-insensitive command prefix.
/// - `snippets` `[in]`: Whether to compute required parameter pieces.
/// - `out` `[in,out]`: Map receiving/overwriting this lowercase command entry; nonmatching entries are unchanged.
///
/// # Returns
/// `()`: a matching candidate was inserted, or the map was left unchanged.
///
/// # Examples
/// ```text
/// schema(command_schema, "mes", false, &mut out);
/// ```
fn schema(
    schema: &CommandSchema,
    prefix: &str,
    snippets: bool,
    out: &mut BTreeMap<String, Candidate>,
) {
    if !matches(&schema.command_name, prefix) {
        return;
    }
    out.insert(
        schema.command_name.to_ascii_lowercase(),
        Candidate {
            name: schema.command_name.clone(),
            kind: kind(schema),
            parameters: snippets.then(|| arguments::parameters(schema)),
            source_defined: false,
        },
    );
}
/// Merge a source declaration, preferring it over native schemas; no I/O.
///
/// # Arguments
/// - `function` `[in]`: Declaration with source definition and formal parameter names; builtin-only records are ignored.
/// - `prefix` `[in]`: Case-insensitive command prefix.
/// - `snippets` `[in]`: Whether formals become placeholders.
/// - `out` `[in,out]`: Inserts/overwrites native entries; existing source entries retain spelling and lose parameters if signatures disagree.
///
/// # Returns
/// `()`: the candidate map is updated only for matching source-defined functions/macros.
///
/// # Algorithm
/// Ignore records without source definitions. Build formal placeholders only when
/// snippets are enabled. A first source declaration replaces any native entry;
/// a later source declaration keeps the original spelling, but clears parameters
/// when its signature differs so insertion cannot pick an arbitrary overload.
///
/// # Flow
/// ```text
/// [Matching source declaration]
///              |
///        <existing source?>
///          /           \
///         no           yes
///          |            |
/// [Insert/replace] <same parameters?>
///                    /         \
///                  yes         no
///                   |           |
///                [Keep]   [Clear parameters]
/// ```
///
/// # Examples
/// ```text
/// function(declaration, "my_", true, &mut out);
/// ```
fn function(
    function: &SemanticFunction,
    prefix: &str,
    snippets: bool,
    out: &mut BTreeMap<String, Candidate>,
) {
    if function.definition.is_none() || !matches(&function.name, prefix) {
        return;
    }
    let candidate = Candidate {
        name: function.name.clone(),
        kind: Kind::Function,
        parameters: snippets.then(|| {
            function
                .parameters
                .iter()
                .map(|parameter| Parameter::Placeholder(parameter.name.clone()))
                .collect()
        }),
        source_defined: true,
    };
    match out.entry(function.name.to_ascii_lowercase()) {
        Entry::Occupied(mut entry) if entry.get().source_defined => {
            if entry.get().parameters != candidate.parameters {
                entry.get_mut().parameters = None;
            }
        }
        entry => {
            entry.insert_entry(candidate);
        }
    }
}
/// Merge possible command spellings across owners without creating reference bindings.
/// Read-only analysis; no I/O.
///
/// # Arguments
/// - `inputs` `[in]`: Owner-local queried snapshots and optional reference indexes.
/// - `prefix` `[in]`: Case-insensitive command-name prefix.
/// - `snippets` `[in]`: Whether to collect safe required-call parameters.
/// - `cancel` `[in]`: Stop signal checked per owner, command and declaration.
///
/// # Returns
/// `Ok(candidates)` is sorted/deduplicated by lowercase spelling; source declarations win and conflicting source signatures use name-only insertion.
///
/// # Errors
/// `Err` reports cooperative cancellation; no analysis is mutated.
///
/// # Algorithm
/// Borrow effective version layers, falling back to the builtin minimum for unknown/unsupported versions. Overlay observed external schemas, then source declarations from owner indexes and local semantics so user shadows always win.
///
/// # Flow
/// ```text
///  [Owner snapshots]          [Versioned builtin layers]
///          |                            |
///          +-------------+--------------+
///                        v
///                [Lowercase-key map]
///                        ^
///                        |
///              [Observed external schemas]
///                        |
///                        v
///             [Source declarations override]
///                        |
///                        v
///           [Sorted possible command candidates]
/// ```
///
/// # Examples
/// ```text
/// let choices = candidates::collect(&inputs, "add_", false, &cancel)?;
/// ```
pub(super) fn collect(
    inputs: &[Input],
    prefix: &str,
    snippets: bool,
    cancel: &Cancellation,
) -> ServerResult<Vec<Candidate>> {
    let mut out = BTreeMap::new();
    for input in inputs {
        if cancel.is_cancelled() {
            return Err(ServerError::cancelled());
        }
        let version = input
            .snapshot
            .semantics
            .version
            .map(|version| (version.major, version.minor))
            .unwrap_or(builtin::MIN_VERSION);
        for native in builtin::schema_layers(version.0, version.1)
            .or_else(|| builtin::schema_layers(builtin::MIN_VERSION.0, builtin::MIN_VERSION.1))
            .into_iter()
            .flatten()
        {
            schema(native, prefix, snippets, &mut out);
        }
        for command in &input.snapshot.semantics.commands {
            if cancel.is_cancelled() {
                return Err(ServerError::cancelled());
            }
            if let Some(external) = command
                .schema
                .as_ref()
                .filter(|schema| matches!(schema.from, CommandSchemaFrom::External))
            {
                schema(external, prefix, snippets, &mut out);
            }
        }
    }
    // Source declarations win over native names, independent of owner traversal order.
    for input in inputs {
        for declaration in input
            .index
            .iter()
            .flat_map(|index| &index.functions)
            .chain(&input.snapshot.semantics.functions)
        {
            if cancel.is_cancelled() {
                return Err(ServerError::cancelled());
            }
            function(declaration, prefix, snippets, &mut out);
        }
    }
    Ok(out.into_values().collect())
}
