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
fn matches(name: &str, prefix: &str) -> bool {
    identifier(name)
        && name
            .get(..prefix.len())
            .is_some_and(|name| name.eq_ignore_ascii_case(prefix))
}
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
