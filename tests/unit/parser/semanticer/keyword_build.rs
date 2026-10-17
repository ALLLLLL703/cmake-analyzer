//! Private build-command schema tests attached by `schema::builtin::all_build`. Completion probes
//! and structural inspections never run the CMake commands they describe.

// Contains AI-generated documentation edits.
// AI-generated tests.
use super::schemas;
use crate::parser::semanticer::{
    model::{BuiltinCommand, CommandKind},
    parsing::{completion_keywords, completion_roles},
    schema::{ArgSchema, ArgSchemaKind as K, CommandSchema, CommandSchemaFrom, NameRole},
};
use std::collections::BTreeSet;

/// Wrap one versioned command grammar from the enclosing module's schemas in a builtin
/// CommandSchema. Allocates local schema data only; no CMake execution or external side effects.
///
/// # Arguments
/// - `command` `[in]`: Exact command name in this module's versioned catalog.
/// - `version` `[in]`: CMake (major, minor) selecting available grammar nodes.
///
/// # Returns
/// An owned builtin schema with its argument nodes wrapped as ArgSchema entries and generic builtin
/// command kind.
///
/// # Panics
/// If the requested command is absent at that version.
///
/// # Examples
/// Local test-module context:
/// ```text
/// let grammar = schema("add_library", (4, 4));
/// assert!(!grammar.args.is_empty());
/// ```
fn schema(command: &str, version: (u32, u32)) -> CommandSchema {
    let (_, args) = schemas(version)
        .into_iter()
        .find(|(name, _)| *name == command)
        .unwrap_or_else(|| panic!("missing {command} at {version:?}"));
    CommandSchema {
        command_name: command.into(),
        args: args.into_iter().map(|kind| ArgSchema { kind }).collect(),
        from: CommandSchemaFrom::Builtin,
        kind: CommandKind::Builtin(BuiltinCommand::Other),
    }
}

/// Collect owned completion words for a partial argument list using [`schema`]. Cancellation is
/// disabled; only local allocations occur and no represented command is executed.
///
/// # Arguments
/// - `command` `[in]`: Exact command name from the enclosing grammar family.
/// - `version` `[in]`: CMake (major, minor) used to construct the schema.
/// - `prefix` `[in]`: Arguments through the active slot; the final string is the typed prefix,
///   including an empty string for a new slot.
///
/// # Returns
/// Sorted, deduplicated keyword strings returned by the production completion matcher; empty for
/// scalar or unreachable slots.
///
/// # Panics
/// If the command is absent at the requested version.
///
/// # Examples
/// Local test-module context:
/// ```text
/// assert!(keywords("add_library", (4, 4), &["lib", "ST"]).contains(&"STATIC".to_owned()));
/// ```
fn keywords(command: &str, version: (u32, u32), prefix: &[&str]) -> Vec<String> {
    completion_keywords(&schema(command, version), prefix, || false)
        .into_iter()
        .map(str::to_owned)
        .collect()
}

/// Assert that a build-grammar prefix offers the expected literal. Calls [`keywords`] with no
/// external side effects.
///
/// # Arguments
/// - `command` `[in]`: Exact build-family command name.
/// - `version` `[in]`: CMake (major, minor) selecting the schema.
/// - `prefix` `[in]`: Arguments through the active completion slot.
/// - `expected` `[in]`: Word whose presence is required.
///
/// # Returns
/// `()` after the membership assertion succeeds; no output buffer is changed.
///
/// # Panics
/// If schema lookup fails or the expected word is absent; the assertion includes the actual
/// completion list.
///
/// # Examples
/// Local test-module context:
/// ```text
/// has("add_library", (4, 4), &["lib", "ST"], "STATIC");
/// ```
fn has(command: &str, version: (u32, u32), prefix: &[&str], expected: &str) {
    let actual = keywords(command, version, prefix);
    assert!(
        actual.iter().any(|word| word == expected),
        "{command} {version:?} {prefix:?}: missing {expected}, got {actual:?}"
    );
}

/// Assert that a build-grammar prefix excludes a literal from the wrong form or version. Calls
/// [`keywords`] with no external side effects.
///
/// # Arguments
/// - `command` `[in]`: Exact build-family command name.
/// - `version` `[in]`: CMake (major, minor) selecting the schema.
/// - `prefix` `[in]`: Arguments through the active completion slot.
/// - `expected` `[in]`: Word whose absence is required.
///
/// # Returns
/// `()` after the membership assertion succeeds; no output buffer is changed.
///
/// # Panics
/// If schema lookup fails or the expected exclusion is violated; the assertion includes the actual
/// completion list.
///
/// # Examples
/// Local test-module context:
/// ```text
/// lacks("add_library", (4, 4), &["lib", "STATIC", ""], "SYMBOLIC");
/// ```
fn lacks(command: &str, version: (u32, u32), prefix: &[&str], expected: &str) {
    let actual = keywords(command, version, prefix);
    assert!(
        !actual.iter().any(|word| word == expected),
        "{command} {version:?} {prefix:?}: unexpected {expected}, got {actual:?}"
    );
}

/// Walk build argument nodes, appending literal keywords to a sorted set while rejecting
/// unstructured fallback nodes. Mutates only the supplied set, without clearing existing entries or
/// performing I/O.
///
/// # Arguments
/// - `kind` `[in]`: Root schema node, recursively including optional/repeated/intersection groups
///   and choices.
/// - `words` `[in,out]`: Existing keyword set; new owned words are inserted and duplicates retained
///   only once, never cleared.
///
/// # Returns
/// `()` after traversal; accumulated words remain in words. Ordinary typed scalars add nothing.
///
/// # Panics
/// On Unmodeled, Unknown or Action nodes, signaling a grammar coverage gap.
///
/// # Algorithm
/// Visit each group/choice recursively, insert Keyword leaves and ignore modeled scalar leaves.
///
/// # Flow
/// ```text
///       [schema node]
///             |
///      +------+-------+
///      |              |
/// [group/choice]   [leaf kind]
///      |           /    |    \
///  [recurse]   keyword scalar fallback
///      |          |      |       |
///      +----> [insert] [skip]  [panic]
/// ```
///
/// # Examples
/// Local test-module context:
/// ```text
/// let mut words = BTreeSet::new();
/// catalog(&K::Optional(vec![K::Keyword("PUBLIC".into())]), &mut words);
/// assert!(words.contains("PUBLIC"));
/// ```
fn catalog(kind: &K, words: &mut BTreeSet<String>) {
    match kind {
        K::Keyword(word) => {
            words.insert(word.clone());
        }
        K::Optional(args) | K::Repeat { args, .. } | K::AllOf(args) => {
            for arg in args {
                catalog(arg, words);
            }
        }
        K::Choice(forms) => {
            for arg in forms.iter().flatten() {
                catalog(arg, words);
            }
        }
        K::Unmodeled | K::Unknown | K::Action => panic!("unstructured fallback: {kind:?}"),
        _ => {}
    }
}

/// Enumerate build schemas at 3.0, 3.31, 4.0 and 4.4, requiring unique names and no fallback nodes.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Check target_sources begins at 3.1 and precompiled-header support at 3.16.
///
/// # Flow
/// ```text
///   [versioned build schemas]
///    |
///   [walk every argument tree]----------> [command introduction cases]
///                                        |
///                                       [check names/counts/no fallback]
///                                        |
///                                       < checks hold? >
///                                        /         \
///                                      [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_all_owned_commands_are_structured();
/// ```
#[test]
fn keyword_build_all_owned_commands_are_structured() {
    for version in [(3, 0), (3, 31), (4, 0), (4, 4)] {
        let entries = schemas(version);
        assert_eq!(entries.len(), if version == (3, 0) { 17 } else { 19 });
        let names: BTreeSet<_> = entries.iter().map(|(name, _)| *name).collect();
        assert_eq!(names.len(), entries.len());
        for (_, args) in entries {
            for arg in args {
                catalog(&arg, &mut BTreeSet::new());
            }
        }
    }
    for (version, command, present) in [
        ((3, 0), "target_sources", false),
        ((3, 1), "target_sources", true),
        ((3, 15), "target_precompile_headers", false),
        ((3, 16), "target_precompile_headers", true),
    ] {
        assert_eq!(
            schemas(version).iter().any(|(name, _)| *name == command),
            present
        );
    }
}

/// Probe OUTPUT versus TARGET custom-command prefixes at 4.4.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Require event and COMMAND ordering, output-only options and empty suggestions inside required
/// scalar operands.
///
/// # Flow
/// ```text
///   [OUTPUT / TARGET prefixes]
///    |
///   [probe mode-specific keywords]----------> [required scalar prefixes]
///                                            |
///                                           [require empty completion]
///                                            |
///                                           < checks hold? >
///                                            /         \
///                                          [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_custom_command_modes_and_scalar_slots();
/// ```
#[test]
fn keyword_build_custom_command_modes_and_scalar_slots() {
    let v = (4, 4);
    assert_eq!(
        keywords("add_custom_command", v, &[""]),
        ["OUTPUT", "TARGET"]
    );
    assert!(keywords("add_custom_command", v, &["OUTPUT", ""]).is_empty());
    assert!(keywords("add_custom_command", v, &["TARGET", ""]).is_empty());
    assert_eq!(
        keywords("add_custom_command", v, &["TARGET", "app", ""]),
        ["POST_BUILD", "PRE_BUILD", "PRE_LINK"]
    );
    assert_eq!(
        keywords(
            "add_custom_command",
            v,
            &["TARGET", "app", "POST_BUILD", ""]
        ),
        ["COMMAND"]
    );
    assert!(
        keywords(
            "add_custom_command",
            v,
            &["TARGET", "app", "POST_BUILD", "COMMAND", ""]
        )
        .is_empty()
    );
    let output = ["OUTPUT", "generated.cpp", ""];
    for word in [
        "COMMAND",
        "MAIN_DEPENDENCY",
        "DEPENDS",
        "BYPRODUCTS",
        "IMPLICIT_DEPENDS",
        "WORKING_DIRECTORY",
        "COMMENT",
        "DEPFILE",
        "JOB_POOL",
        "JOB_SERVER_AWARE",
        "VERBATIM",
        "APPEND",
        "USES_TERMINAL",
        "CODEGEN",
        "COMMAND_EXPAND_LISTS",
        "DEPENDS_EXPLICIT_ONLY",
    ] {
        has("add_custom_command", v, &output, word);
    }
    has(
        "add_custom_command",
        v,
        &["OUTPUT", "out", "COMMAND", "tool", ""],
        "ARGS",
    );
    for word in [
        "OUTPUT",
        "MAIN_DEPENDENCY",
        "DEPENDS",
        "IMPLICIT_DEPENDS",
        "DEPFILE",
        "JOB_POOL",
        "JOB_SERVER_AWARE",
        "APPEND",
        "CODEGEN",
        "DEPENDS_EXPLICIT_ONLY",
    ] {
        lacks(
            "add_custom_command",
            v,
            &["TARGET", "app", "POST_BUILD", "COMMAND", "tool", ""],
            word,
        );
    }
    for prefix in [
        vec!["OUTPUT", "out", "COMMENT", ""],
        vec!["OUTPUT", "out", "MAIN_DEPENDENCY", ""],
        vec!["OUTPUT", "out", "JOB_SERVER_AWARE", ""],
        vec!["OUTPUT", "out", "IMPLICIT_DEPENDS", "CXX", ""],
    ] {
        assert!(
            keywords("add_custom_command", v, &prefix).is_empty(),
            "{prefix:?}"
        );
    }
}

/// Probe custom-command and custom-target option introductions immediately before/at their
/// versions, from BYPRODUCTS/USES_TERMINAL in 3.2 through CODEGEN in 3.31.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Probe custom-command and custom-target option introductions immediately before/at their
/// versions, from BYPRODUCTS/USES_TERMINAL in 3.2 through CODEGEN in 3.31.
///
/// # Flow
/// ```text
///   [before/introduced versions]
///    |
///   [probe custom command options]----------> [custom target options]
///                                            |
///                                           [compare absent/present words]
///                                            |
///                                           < checks hold? >
///                                            /         \
///                                          [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_custom_command_version_boundaries();
/// ```
#[test]
fn keyword_build_custom_command_version_boundaries() {
    for (word, before, after) in [
        ("BYPRODUCTS", (3, 1), (3, 2)),
        ("USES_TERMINAL", (3, 1), (3, 2)),
        ("DEPFILE", (3, 6), (3, 7)),
        ("COMMAND_EXPAND_LISTS", (3, 7), (3, 8)),
        ("JOB_POOL", (3, 14), (3, 15)),
        ("DEPENDS_EXPLICIT_ONLY", (3, 26), (3, 27)),
        ("JOB_SERVER_AWARE", (3, 27), (3, 28)),
        ("CODEGEN", (3, 30), (3, 31)),
    ] {
        lacks("add_custom_command", before, &["OUTPUT", "out", ""], word);
        has("add_custom_command", after, &["OUTPUT", "out", ""], word);
    }
    for (word, before, after) in [
        ("BYPRODUCTS", (3, 1), (3, 2)),
        ("USES_TERMINAL", (3, 1), (3, 2)),
        ("COMMAND_EXPAND_LISTS", (3, 7), (3, 8)),
        ("JOB_POOL", (3, 14), (3, 15)),
        ("JOB_SERVER_AWARE", (3, 27), (3, 28)),
    ] {
        lacks("add_custom_target", before, &["generate", ""], word);
        has("add_custom_target", after, &["generate", ""], word);
    }
}

/// Probe a custom target before/after ALL and implicit executable arguments.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Require shared command options, but suppress ALL once an explicit COMMAND starts and never offer
/// OUTPUT.
///
/// # Flow
/// ```text
///   [implicit / explicit commands]
///    |
///   [probe options after target]------------> [ALL and OUTPUT exclusions]
///                                            |
///                                           [check mode-specific suggestions]
///                                            |
///                                           < checks hold? >
///                                            /         \
///                                          [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_custom_target_implicit_and_explicit_commands();
/// ```
#[test]
fn keyword_build_custom_target_implicit_and_explicit_commands() {
    let v = (4, 4);
    assert!(keywords("add_custom_target", v, &[""]).is_empty());
    for prefix in [
        vec!["gen", ""],
        vec!["gen", "ALL", ""],
        vec!["gen", "tool", "arg", ""],
    ] {
        for word in [
            "COMMAND",
            "DEPENDS",
            "BYPRODUCTS",
            "WORKING_DIRECTORY",
            "COMMENT",
            "JOB_POOL",
            "JOB_SERVER_AWARE",
            "VERBATIM",
            "USES_TERMINAL",
            "COMMAND_EXPAND_LISTS",
            "SOURCES",
        ] {
            has("add_custom_target", v, &prefix, word);
        }
    }
    has("add_custom_target", v, &["gen", ""], "ALL");
    lacks(
        "add_custom_target",
        v,
        &["gen", "COMMAND", "tool", ""],
        "ALL",
    );
    lacks(
        "add_custom_target",
        v,
        &["gen", "COMMAND", "tool", ""],
        "OUTPUT",
    );
}

/// Probe executable/library names, alias operands, imported GLOBAL, object imports and
/// interface-only options.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Check version gates and ensure SYMBOLIC never leaks into STATIC library forms.
///
/// # Flow
/// ```text
///   [executable / library forms]
///    |
///   [probe alias/import/interface]----------> [versioned special options]
///                                            |
///                                           [check gates and non-leakage]
///                                            |
///                                           < checks hold? >
///                                            /         \
///                                          [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_target_creation_forms();
/// ```
#[test]
fn keyword_build_target_creation_forms() {
    for command in ["add_executable", "add_library"] {
        assert!(keywords(command, (4, 4), &[""]).is_empty());
        has(command, (3, 0), &["app", ""], "ALIAS");
        lacks(command, (3, 0), &["app", ""], "GLOBAL");
        assert!(keywords(command, (4, 4), &["app", "ALIAS", ""]).is_empty());
    }
    for word in ["WIN32", "MACOSX_BUNDLE", "EXCLUDE_FROM_ALL", "IMPORTED"] {
        has("add_executable", (3, 0), &["app", ""], word);
    }
    has("add_executable", (3, 0), &["app", "IMPORTED", ""], "GLOBAL");
    has(
        "add_library",
        (3, 0),
        &["lib", "STATIC", "IMPORTED", ""],
        "GLOBAL",
    );
    lacks("add_library", (3, 8), &["lib", "OBJECT", ""], "IMPORTED");
    has("add_library", (3, 9), &["lib", "OBJECT", ""], "IMPORTED");
    lacks(
        "add_library",
        (3, 18),
        &["lib", "INTERFACE", ""],
        "EXCLUDE_FROM_ALL",
    );
    has(
        "add_library",
        (3, 19),
        &["lib", "INTERFACE", ""],
        "EXCLUDE_FROM_ALL",
    );
    lacks("add_library", (4, 1), &["lib", "INTERFACE", ""], "SYMBOLIC");
    has("add_library", (4, 2), &["lib", "INTERFACE", ""], "SYMBOLIC");
    lacks("add_library", (4, 4), &["lib", "STATIC", ""], "SYMBOLIC");
}

/// Contrast named tests with legacy positional tests.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Require NAME/COMMAND scalar ordering, named-only options and introduction gates for command-list
/// expansion and build dependencies.
///
/// # Flow
/// ```text
///   [NAME / legacy prefixes]
///    |
///   [probe test option slots]----------> [before/introduced versions]
///                                       |
///                                      [check ordering + version gates]
///                                       |
///                                      < checks hold? >
///                                       /         \
///                                     [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_add_test_named_vs_legacy();
/// ```
#[test]
fn keyword_build_add_test_named_vs_legacy() {
    let v = (4, 4);
    has("add_test", v, &[""], "NAME");
    assert!(keywords("add_test", v, &["NAME", ""]).is_empty());
    assert_eq!(keywords("add_test", v, &["NAME", "smoke", ""]), ["COMMAND"]);
    assert!(keywords("add_test", v, &["NAME", "smoke", "COMMAND", ""]).is_empty());
    for word in [
        "CONFIGURATIONS",
        "WORKING_DIRECTORY",
        "COMMAND_EXPAND_LISTS",
        "BUILD_DEPENDS",
    ] {
        has(
            "add_test",
            v,
            &["NAME", "smoke", "COMMAND", "app", ""],
            word,
        );
        lacks("add_test", v, &["smoke", "app", ""], word);
    }
    lacks(
        "add_test",
        (3, 15),
        &["NAME", "smoke", "COMMAND", "app", ""],
        "COMMAND_EXPAND_LISTS",
    );
    has(
        "add_test",
        (3, 16),
        &["NAME", "smoke", "COMMAND", "app", ""],
        "COMMAND_EXPAND_LISTS",
    );
    lacks(
        "add_test",
        (4, 3),
        &["NAME", "smoke", "COMMAND", "app", ""],
        "BUILD_DEPENDS",
    );
}

/// Probe subdirectories, build-command outputs, external projects and legacy install/subdirs forms.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Require supported options, SYSTEM/PARALLEL_LEVEL version gates and no keywords in required
/// target/project-file operands.
///
/// # Flow
/// ```text
///   [small/legacy build forms]
///    |
///   [probe each option prefix]----------> [SYSTEM / PARALLEL_LEVEL gates]
///                                        |
///                                       [check scalars stay uncompleted]
///                                        |
///                                       < checks hold? >
///                                        /         \
///                                      [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_small_commands_and_legacy_signatures();
/// ```
#[test]
fn keyword_build_small_commands_and_legacy_signatures() {
    for (command, prefix, word) in [
        ("add_subdirectory", vec!["src", ""], "EXCLUDE_FROM_ALL"),
        ("add_subdirectory", vec!["src", "binary", ""], "SYSTEM"),
        ("build_command", vec!["cmd", ""], "CONFIGURATION"),
        ("build_command", vec!["cmd", ""], "TARGET"),
        ("build_command", vec!["cmd", ""], "PROJECT_NAME"),
        (
            "include_external_msproject",
            vec!["external", "file.csproj", ""],
            "TYPE",
        ),
        (
            "include_external_msproject",
            vec!["external", "file.csproj", ""],
            "GUID",
        ),
        (
            "include_external_msproject",
            vec!["external", "file.csproj", ""],
            "PLATFORM",
        ),
        ("install_files", vec!["include", ""], "FILES"),
        ("install_programs", vec!["bin", ""], "FILES"),
        ("install_targets", vec!["lib", ""], "RUNTIME_DIRECTORY"),
        ("subdirs", vec!["src", ""], "EXCLUDE_FROM_ALL"),
        ("subdirs", vec!["src", ""], "PREORDER"),
    ] {
        has(command, (4, 4), &prefix, word);
    }
    lacks("add_subdirectory", (3, 24), &["src", ""], "SYSTEM");
    has("add_subdirectory", (3, 25), &["src", ""], "SYSTEM");
    lacks("build_command", (3, 20), &["cmd", ""], "PARALLEL_LEVEL");
    has("build_command", (3, 21), &["cmd", ""], "PARALLEL_LEVEL");
    assert!(keywords("build_command", (4, 4), &["cmd", "TARGET", ""]).is_empty());
    assert!(keywords("include_external_msproject", (4, 4), &["external", ""]).is_empty());
}

/// Require two configure_file path operands before options, restrict newline spellings to
/// NEWLINE_STYLE and check permission-option introductions and OWNER_READ completion.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Require two configure_file path operands before options, restrict newline spellings to
/// NEWLINE_STYLE and check permission-option introductions and OWNER_READ completion.
///
/// # Flow
/// ```text
///   [two path operands]
///    |
///   [probe configure_file options]----------> [newline / permission slots]
///                                            |
///                                           [check nested choices + gates]
///                                            |
///                                           < checks hold? >
///                                            /         \
///                                          [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_configure_file_permissions_and_newlines();
/// ```
#[test]
fn keyword_build_configure_file_permissions_and_newlines() {
    assert!(keywords("configure_file", (4, 4), &[""]).is_empty());
    assert!(keywords("configure_file", (4, 4), &["in", ""]).is_empty());
    for word in ["COPYONLY", "ESCAPE_QUOTES", "@ONLY", "NEWLINE_STYLE"] {
        has("configure_file", (3, 0), &["in", "out", ""], word);
    }
    for word in ["UNIX", "DOS", "WIN32", "LF", "CRLF"] {
        has(
            "configure_file",
            (3, 0),
            &["in", "out", "NEWLINE_STYLE", ""],
            word,
        );
        lacks("configure_file", (3, 0), &["in", "out", ""], word);
    }
    for (word, before, after) in [
        ("NO_SOURCE_PERMISSIONS", (3, 18), (3, 19)),
        ("USE_SOURCE_PERMISSIONS", (3, 19), (3, 20)),
        ("FILE_PERMISSIONS", (3, 19), (3, 20)),
    ] {
        lacks("configure_file", before, &["in", "out", ""], word);
        has("configure_file", after, &["in", "out", ""], word);
    }
    has(
        "configure_file",
        (3, 20),
        &["in", "out", "FILE_PERMISSIONS", ""],
        "OWNER_READ",
    );
}

/// Probe visibility groups, file-set name/type/options and precompiled-header reuse.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Check FILE_SET, CXX_MODULES and SOURCES version gates, while preventing file-set options and
/// REUSE_FROM from escaping their forms.
///
/// # Flow
/// ```text
///   [visibility + FILE_SET prefixes]
///    |
///   [probe names/types/options]---------------> [precompiled-header REUSE_FROM]
///                                              |
///                                             [check scope + introductions]
///                                              |
///                                             < checks hold? >
///                                              /         \
///                                            [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_target_sources_file_sets_and_reuse();
/// ```
#[test]
fn keyword_build_target_sources_file_sets_and_reuse() {
    let v = (4, 4);
    for command in ["target_sources", "target_precompile_headers"] {
        assert!(keywords(command, v, &[""]).is_empty());
        for word in ["PRIVATE", "PUBLIC", "INTERFACE"] {
            has(command, v, &["app", ""], word);
        }
    }
    lacks(
        "target_sources",
        (3, 22),
        &["app", "PUBLIC", ""],
        "FILE_SET",
    );
    has(
        "target_sources",
        (3, 23),
        &["app", "PUBLIC", ""],
        "FILE_SET",
    );
    for word in ["TYPE", "BASE_DIRS", "FILES"] {
        has(
            "target_sources",
            v,
            &["app", "PUBLIC", "FILE_SET", "headers", ""],
            word,
        );
        lacks("target_sources", v, &["app", "PUBLIC", ""], word);
    }
    has(
        "target_sources",
        (3, 23),
        &["app", "PUBLIC", "FILE_SET", "headers", "TYPE", ""],
        "HEADERS",
    );
    lacks(
        "target_sources",
        (3, 27),
        &["app", "PUBLIC", "FILE_SET", "modules", "TYPE", ""],
        "CXX_MODULES",
    );
    has(
        "target_sources",
        (3, 28),
        &["app", "PUBLIC", "FILE_SET", "modules", "TYPE", ""],
        "CXX_MODULES",
    );
    lacks(
        "target_sources",
        (4, 3),
        &["app", "PUBLIC", "FILE_SET", "sources", "TYPE", ""],
        "SOURCES",
    );
    has(
        "target_sources",
        (4, 4),
        &["app", "PUBLIC", "FILE_SET", "sources", "TYPE", ""],
        "SOURCES",
    );
    has(
        "target_precompile_headers",
        (3, 16),
        &["app", ""],
        "REUSE_FROM",
    );
    lacks(
        "target_precompile_headers",
        v,
        &["app", "PRIVATE", "<vector>", ""],
        "REUSE_FROM",
    );
    assert!(keywords("target_precompile_headers", v, &["app", "REUSE_FROM", ""]).is_empty());
}

/// Probe export root modes and their nested options, including SETUP dependencies/targets and AUTO.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Check Android/module/dependency/SBOM version gates and required scalar suppression without
/// creating export files.
///
/// # Flow
/// ```text
///   [export mode prefixes]
///    |
///   [probe mode-specific options]----------> [SETUP nested dependency/target]
///                                           |
///                                          [check gates + scalar suppression]
///                                           |
///                                          < checks hold? >
///                                           /         \
///                                         [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_export_modes();
/// ```
#[test]
fn keyword_build_export_modes() {
    let v = (4, 4);
    assert_eq!(
        keywords("export", v, &[""]),
        [
            "EXPORT",
            "PACKAGE",
            "PACKAGE_INFO",
            "SBOM",
            "SETUP",
            "TARGETS"
        ]
    );
    assert!(keywords("export", v, &["PACKAGE", ""]).is_empty());
    for word in [
        "NAMESPACE",
        "APPEND",
        "FILE",
        "EXPORT_LINK_INTERFACE_LIBRARIES",
        "CXX_MODULES_DIRECTORY",
        "ANDROID_MK",
    ] {
        has("export", v, &["TARGETS", "app", ""], word);
    }
    for word in [
        "NAMESPACE",
        "FILE",
        "CXX_MODULES_DIRECTORY",
        "EXPORT_PACKAGE_DEPENDENCIES",
    ] {
        has("export", v, &["EXPORT", "set", ""], word);
    }
    for (word, before, after, prefix) in [
        ("ANDROID_MK", (3, 6), (3, 7), vec!["TARGETS", "app", ""]),
        (
            "CXX_MODULES_DIRECTORY",
            (3, 27),
            (3, 28),
            vec!["EXPORT", "set", ""],
        ),
        (
            "EXPORT_PACKAGE_DEPENDENCIES",
            (3, 28),
            (3, 29),
            vec!["EXPORT", "set", ""],
        ),
        ("SETUP", (3, 28), (3, 29), vec![""]),
        ("SBOM", (4, 2), (4, 3), vec![""]),
    ] {
        lacks("export", before, &prefix, word);
        has("export", after, &prefix, word);
    }
    has("export", v, &["SETUP", "set", ""], "PACKAGE_DEPENDENCY");
    has("export", v, &["SETUP", "set", ""], "TARGET");
    has(
        "export",
        v,
        &["SETUP", "set", "PACKAGE_DEPENDENCY", "Dep", ""],
        "ENABLED",
    );
    has(
        "export",
        v,
        &["SETUP", "set", "PACKAGE_DEPENDENCY", "Dep", ""],
        "EXTRA_ARGS",
    );
    has(
        "export",
        v,
        &["SETUP", "set", "PACKAGE_DEPENDENCY", "Dep", "ENABLED", ""],
        "AUTO",
    );
    has(
        "export",
        v,
        &["SETUP", "set", "TARGET", "lib", ""],
        "XCFRAMEWORK_LOCATION",
    );
    lacks("export", v, &["SETUP", "set", ""], "XCFRAMEWORK_LOCATION");
    assert!(keywords("export", v, &["TARGETS", "app", "NAMESPACE", ""]).is_empty());
}

/// Probe install root forms, target artifact sections, file sets and runtime dependency filters.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Require INCLUDES DESTINATION ordering and prevent runtime-only filters or ARCHIVE from appearing
/// in unrelated forms.
///
/// # Flow
/// ```text
///   [install root/artifact forms]
///    |
///   [probe target sections]----------------> [runtime vs unrelated forms]
///                                           |
///                                          [check allowed and excluded filters]
///                                           |
///                                          < checks hold? >
///                                           /         \
///                                         [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_install_artifact_and_runtime_modes();
/// ```
#[test]
fn keyword_build_install_artifact_and_runtime_modes() {
    let v = (4, 4);
    for word in [
        "TARGETS",
        "FILES",
        "PROGRAMS",
        "DIRECTORY",
        "SCRIPT",
        "CODE",
        "EXPORT",
        "EXPORT_ANDROID_MK",
        "IMPORTED_RUNTIME_ARTIFACTS",
        "RUNTIME_DEPENDENCY_SET",
        "PACKAGE_INFO",
        "SBOM",
    ] {
        has("install", v, &[""], word);
    }
    for word in [
        "EXPORT",
        "DESTINATION",
        "PERMISSIONS",
        "CONFIGURATIONS",
        "COMPONENT",
        "NAMELINK_COMPONENT",
        "OPTIONAL",
        "EXCLUDE_FROM_ALL",
        "NAMELINK_ONLY",
        "NAMELINK_SKIP",
        "ARCHIVE",
        "LIBRARY",
        "RUNTIME",
        "FRAMEWORK",
        "BUNDLE",
        "PRIVATE_HEADER",
        "PUBLIC_HEADER",
        "RESOURCE",
        "OBJECTS",
        "FILE_SET",
        "CXX_MODULES_BMI",
        "INCLUDES",
        "RUNTIME_DEPENDENCY_SET",
        "RUNTIME_DEPENDENCIES",
    ] {
        has("install", v, &["TARGETS", "lib", ""], word);
    }
    assert_eq!(
        keywords("install", v, &["TARGETS", "lib", "INCLUDES", ""]),
        ["DESTINATION"]
    );
    assert!(keywords("install", v, &["TARGETS", "lib", "FILE_SET", ""]).is_empty());
    has(
        "install",
        v,
        &["TARGETS", "lib", "FILE_SET", "headers", ""],
        "DESTINATION",
    );
    for word in [
        "DIRECTORIES",
        "PRE_INCLUDE_REGEXES",
        "PRE_EXCLUDE_REGEXES",
        "POST_INCLUDE_REGEXES",
        "POST_EXCLUDE_REGEXES",
        "POST_INCLUDE_FILES",
        "POST_EXCLUDE_FILES",
    ] {
        has("install", v, &["RUNTIME_DEPENDENCY_SET", "deps", ""], word);
        has(
            "install",
            v,
            &["TARGETS", "app", "RUNTIME_DEPENDENCIES", ""],
            word,
        );
        lacks(
            "install",
            v,
            &["FILES", "file", "DESTINATION", "bin", ""],
            word,
        );
    }
    for word in [
        "LIBRARY",
        "RUNTIME",
        "FRAMEWORK",
        "BUNDLE",
        "RUNTIME_DEPENDENCY_SET",
    ] {
        has(
            "install",
            v,
            &["IMPORTED_RUNTIME_ARTIFACTS", "app", ""],
            word,
        );
    }
    lacks(
        "install",
        v,
        &["IMPORTED_RUNTIME_ARTIFACTS", "app", ""],
        "ARCHIVE",
    );
}

/// Probe file destination/type alternatives, directory match filters, script/code bodies and export
/// options.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Require scalar destination/body slots and keep PATTERN-only suboptions and DESTINATION scoped to
/// the correct forms.
///
/// # Flow
/// ```text
///   [files/directory/script/export]
///    |
///   [probe destination/body slots]-----------> [PATTERN nested options]
///                                             |
///                                            [check exact option scope]
///                                             |
///                                            < checks hold? >
///                                             /         \
///                                           [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_install_files_directories_scripts_and_exports();
/// ```
#[test]
fn keyword_build_install_files_directories_scripts_and_exports() {
    let v = (4, 4);
    has("install", v, &["FILES", "header.h", ""], "TYPE");
    has("install", v, &["FILES", "header.h", ""], "DESTINATION");
    lacks("install", v, &["FILES", "header.h", ""], "RENAME");
    assert!(keywords("install", v, &["FILES", "header.h", "DESTINATION", ""]).is_empty());
    for word in [
        "PERMISSIONS",
        "CONFIGURATIONS",
        "COMPONENT",
        "RENAME",
        "OPTIONAL",
        "EXCLUDE_FROM_ALL",
    ] {
        has(
            "install",
            v,
            &["FILES", "header.h", "DESTINATION", "include", ""],
            word,
        );
    }
    for word in [
        "BIN",
        "SBIN",
        "LIB",
        "INCLUDE",
        "SYSCONF",
        "SHAREDSTATE",
        "LOCALSTATE",
        "RUNSTATE",
        "DATA",
        "INFO",
        "LOCALE",
        "MAN",
        "DOC",
        "LIBEXEC",
    ] {
        has("install", v, &["FILES", "file", "TYPE", ""], word);
    }
    lacks("install", v, &["FILES", "file", "TYPE", ""], "DATAROOT");
    for word in [
        "FILE_PERMISSIONS",
        "DIRECTORY_PERMISSIONS",
        "USE_SOURCE_PERMISSIONS",
        "OPTIONAL",
        "MESSAGE_NEVER",
        "FILES_MATCHING",
        "PATTERN",
        "REGEX",
    ] {
        has(
            "install",
            v,
            &["DIRECTORY", "src", "DESTINATION", "share", ""],
            word,
        );
    }
    for word in ["EXCLUDE", "PERMISSIONS"] {
        has(
            "install",
            v,
            &[
                "DIRECTORY",
                "src",
                "DESTINATION",
                "share",
                "PATTERN",
                "*.txt",
                "",
            ],
            word,
        );
        lacks(
            "install",
            v,
            &["DIRECTORY", "src", "DESTINATION", "share", ""],
            word,
        );
    }
    for command in ["SCRIPT", "CODE"] {
        assert!(keywords("install", v, &[command, ""]).is_empty());
        has("install", v, &[command, "opaque", ""], "COMPONENT");
        has("install", v, &[command, "opaque", ""], "ALL_COMPONENTS");
        lacks("install", v, &[command, "opaque", ""], "DESTINATION");
    }
    for word in [
        "NAMESPACE",
        "FILE",
        "PERMISSIONS",
        "CONFIGURATIONS",
        "CXX_MODULES_DIRECTORY",
        "EXPORT_LINK_INTERFACE_LIBRARIES",
        "COMPONENT",
        "EXCLUDE_FROM_ALL",
        "EXPORT_PACKAGE_DEPENDENCIES",
    ] {
        has(
            "install",
            v,
            &["EXPORT", "set", "DESTINATION", "lib/cmake", ""],
            word,
        );
    }
}

/// Probe install options immediately before and at each introduction, from MESSAGE_NEVER in 3.1
/// through SBOM in 4.3, including artifact, file-set and package metadata forms.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Probe install options immediately before and at each introduction, from MESSAGE_NEVER in 3.1
/// through SBOM in 4.3, including artifact, file-set and package metadata forms.
///
/// # Flow
/// ```text
///   [old/new install versions]
///    |
///   [probe contextual prefix]-----------> [introduced option word]
///                                        |
///                                       [check absent then present]
///                                        |
///                                       < checks hold? >
///                                        /         \
///                                      [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_install_version_boundaries();
/// ```
#[test]
fn keyword_build_install_version_boundaries() {
    for (word, before, after, prefix) in [
        (
            "MESSAGE_NEVER",
            (3, 0),
            (3, 1),
            vec!["DIRECTORY", "src", "DESTINATION", "share", ""],
        ),
        (
            "EXCLUDE_FROM_ALL",
            (3, 5),
            (3, 6),
            vec!["FILES", "f", "DESTINATION", "share", ""],
        ),
        ("EXPORT_ANDROID_MK", (3, 6), (3, 7), vec![""]),
        ("OBJECTS", (3, 8), (3, 9), vec!["TARGETS", "obj", ""]),
        (
            "NAMELINK_COMPONENT",
            (3, 11),
            (3, 12),
            vec!["TARGETS", "lib", "LIBRARY", ""],
        ),
        ("TYPE", (3, 13), (3, 14), vec!["FILES", "f", ""]),
        ("IMPORTED_RUNTIME_ARTIFACTS", (3, 20), (3, 21), vec![""]),
        ("RUNTIME_DEPENDENCY_SET", (3, 20), (3, 21), vec![""]),
        (
            "RUNTIME_DEPENDENCIES",
            (3, 20),
            (3, 21),
            vec!["TARGETS", "app", ""],
        ),
        ("ALL_COMPONENTS", (3, 20), (3, 21), vec!["CODE", "body", ""]),
        ("FILE_SET", (3, 22), (3, 23), vec!["TARGETS", "lib", ""]),
        (
            "CXX_MODULES_BMI",
            (3, 27),
            (3, 28),
            vec!["TARGETS", "lib", ""],
        ),
        (
            "CXX_MODULES_DIRECTORY",
            (3, 27),
            (3, 28),
            vec!["EXPORT", "set", "DESTINATION", "lib/cmake", ""],
        ),
        (
            "EXPORT_PACKAGE_DEPENDENCIES",
            (3, 28),
            (3, 29),
            vec!["EXPORT", "set", "DESTINATION", "lib/cmake", ""],
        ),
        (
            "LIBEXEC",
            (3, 30),
            (3, 31),
            vec!["FILES", "file", "TYPE", ""],
        ),
        ("PACKAGE_INFO", (3, 30), (3, 31), vec![""]),
        ("SBOM", (4, 2), (4, 3), vec![""]),
    ] {
        lacks("install", before, &prefix, word);
        has("install", after, &prefix, word);
    }
}

/// Check PACKAGE_INFO moves from nested EXPORT syntax to a root form in 4.3.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Probe version-dependent metadata, VERSION-only compatibility options, APPENDIX exclusions and
/// required SBOM name/EXPORTS ordering; SBOM means software bill of materials.
///
/// # Flow
/// ```text
///   [historical PACKAGE_INFO forms]
///    |
///   [probe versioned metadata]---------------> [VERSION / APPENDIX / SBOM]
///                                             |
///                                            [check dependent options]
///                                             |
///                                            < checks hold? >
///                                             /         \
///                                           [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_package_info_metadata_historical_forms_and_sbom();
/// ```
#[test]
fn keyword_build_package_info_metadata_historical_forms_and_sbom() {
    lacks("export", (4, 0), &["EXPORT", "set", ""], "PACKAGE_INFO");
    has("export", (4, 1), &["EXPORT", "set", ""], "PACKAGE_INFO");
    lacks("export", (4, 2), &[""], "PACKAGE_INFO");
    has("export", (4, 3), &[""], "PACKAGE_INFO");
    lacks("export", (4, 3), &["EXPORT", "set", ""], "PACKAGE_INFO");
    for (command, prefix) in [
        ("install", vec!["PACKAGE_INFO", "Pkg", "EXPORT", "set", ""]),
        ("export", vec!["PACKAGE_INFO", "Pkg", "EXPORT", "set", ""]),
    ] {
        for word in [
            "PROJECT",
            "NO_PROJECT_METADATA",
            "APPENDIX",
            "LOWER_CASE_FILE",
            "VERSION",
            "DEFAULT_TARGETS",
            "DEFAULT_CONFIGURATIONS",
            "LICENSE",
            "DEFAULT_LICENSE",
            "DESCRIPTION",
            "HOMEPAGE_URL",
            "CXX_MODULES_DIRECTORY",
        ] {
            has(command, (4, 4), &prefix, word);
        }
        lacks(command, (4, 4), &prefix, "COMPAT_VERSION");
        let mut version = prefix[..prefix.len() - 1].to_vec();
        version.extend(["VERSION", "1.0", ""]);
        has(command, (4, 4), &version, "COMPAT_VERSION");
        has(command, (4, 4), &version, "VERSION_SCHEMA");
        let mut appendix = prefix[..prefix.len() - 1].to_vec();
        appendix.extend(["APPENDIX", "extra", ""]);
        lacks(command, (4, 4), &appendix, "VERSION");
        lacks(command, (4, 4), &appendix, "DEFAULT_TARGETS");
        lacks(command, (4, 4), &appendix, "PROJECT");
    }
    for (word, before, after) in [
        ("DESCRIPTION", (4, 0), (4, 1)),
        ("HOMEPAGE_URL", (4, 0), (4, 1)),
        ("LICENSE", (4, 1), (4, 2)),
        ("DEFAULT_LICENSE", (4, 1), (4, 2)),
    ] {
        lacks(
            "install",
            before,
            &["PACKAGE_INFO", "Pkg", "EXPORT", "set", ""],
            word,
        );
        has(
            "install",
            after,
            &["PACKAGE_INFO", "Pkg", "EXPORT", "set", ""],
            word,
        );
    }
    for command in ["install", "export"] {
        assert!(keywords(command, (4, 3), &["SBOM", ""]).is_empty());
        assert_eq!(
            keywords(command, (4, 3), &["SBOM", "name", ""]),
            ["EXPORTS"]
        );
        for word in [
            "FORMAT",
            "PROJECT",
            "NO_PROJECT_METADATA",
            "VERSION",
            "LICENSE",
            "DESCRIPTION",
            "HOMEPAGE_URL",
            "PACKAGE_URL",
        ] {
            has(
                command,
                (4, 3),
                &["SBOM", "name", "EXPORTS", "set", ""],
                word,
            );
        }
    }
}

/// Contrast modern PROJECT/source-generation try_compile forms with legacy binary/source operands.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Require source/project-specific options, generated-source scalar slots and COPY_FILE_ERROR only
/// after COPY_FILE.
///
/// # Flow
/// ```text
///   [PROJECT / sources / legacy]
///    |
///   [probe try_compile prefixes]----------> [generated-source / COPY_FILE]
///                                          |
///                                         [check required and nested slots]
///                                          |
///                                         < checks hold? >
///                                          /         \
///                                        [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_try_compile_source_project_and_legacy_modes();
/// ```
#[test]
fn keyword_build_try_compile_source_project_and_legacy_modes() {
    let v = (4, 4);
    for word in [
        "PROJECT",
        "SOURCES",
        "SOURCE_FROM_CONTENT",
        "SOURCE_FROM_VAR",
        "SOURCE_FROM_FILE",
        "SOURCES_TYPE",
    ] {
        has("try_compile", v, &["result", ""], word);
    }
    assert!(keywords("try_compile", v, &["result", "PROJECT", ""]).is_empty());
    assert_eq!(
        keywords("try_compile", v, &["result", "PROJECT", "proj", ""]),
        ["SOURCE_DIR"]
    );
    for word in [
        "BINARY_DIR",
        "TARGET",
        "LOG_DESCRIPTION",
        "NO_CACHE",
        "NO_LOG",
        "CMAKE_FLAGS",
        "OUTPUT_VARIABLE",
    ] {
        has(
            "try_compile",
            v,
            &["result", "PROJECT", "proj", "SOURCE_DIR", "src", ""],
            word,
        );
    }
    lacks(
        "try_compile",
        v,
        &["result", "PROJECT", "proj", "SOURCE_DIR", "src", ""],
        "COMPILE_DEFINITIONS",
    );
    for word in [
        "LOG_DESCRIPTION",
        "NO_CACHE",
        "NO_LOG",
        "CMAKE_FLAGS",
        "COMPILE_DEFINITIONS",
        "LINK_OPTIONS",
        "LINK_LIBRARIES",
        "LINKER_LANGUAGE",
        "OUTPUT_VARIABLE",
        "COPY_FILE",
        "C_STANDARD",
        "CXX_STANDARD_REQUIRED",
        "CUDA_EXTENSIONS",
        "OBJC_STANDARD",
        "OBJCXX_EXTENSIONS",
    ] {
        has("try_compile", v, &["result", "SOURCES", "test.c", ""], word);
    }
    assert!(
        keywords(
            "try_compile",
            v,
            &["result", "SOURCE_FROM_CONTENT", "test.c", ""]
        )
        .is_empty()
    );
    has(
        "try_compile",
        v,
        &[
            "result",
            "SOURCE_FROM_CONTENT",
            "test.c",
            "int main() {}",
            "",
        ],
        "NO_CACHE",
    );
    has(
        "try_compile",
        v,
        &["result", "SOURCES", "test.c", "COPY_FILE", "test.exe", ""],
        "COPY_FILE_ERROR",
    );
    lacks(
        "try_compile",
        v,
        &["result", "SOURCES", "test.c", ""],
        "COPY_FILE_ERROR",
    );
    for word in ["CMAKE_FLAGS", "OUTPUT_VARIABLE"] {
        has(
            "try_compile",
            (3, 0),
            &["result", "binary", "src", "project", ""],
            word,
        );
    }
    has(
        "try_compile",
        (3, 0),
        &["result", "binary", "test.c", ""],
        "COPY_FILE",
    );
    lacks("try_compile", (3, 24), &["result", ""], "PROJECT");
    has("try_compile", (3, 25), &["result", ""], "PROJECT");
}

/// Probe try_compile/try_run source-generation, logging, language and linking introductions.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Build prefixes with one or two result variables as appropriate, then check older versus
/// introduced schemas.
///
/// # Flow
/// ```text
///   [one/two result prefixes]
///    |
///   [append versioned option case]----------> [before/introduced schemas]
///                                            |
///                                           [check keyword availability]
///                                            |
///                                           < checks hold? >
///                                            /         \
///                                          [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_try_compile_and_run_version_boundaries();
/// ```
#[test]
fn keyword_build_try_compile_and_run_version_boundaries() {
    for command in ["try_compile", "try_run"] {
        let initial: Vec<_> = if command == "try_compile" {
            vec!["compile"]
        } else {
            vec!["run", "compile"]
        };
        for (word, before, after, suffix) in [
            ("SOURCE_FROM_CONTENT", (3, 24), (3, 25), vec![""]),
            ("SOURCE_FROM_FILE", (3, 24), (3, 25), vec![""]),
            ("SOURCE_FROM_VAR", (3, 24), (3, 25), vec![""]),
            ("SOURCES_TYPE", (3, 27), (3, 28), vec![""]),
            ("NO_CACHE", (3, 24), (3, 25), vec!["SOURCES", "test.c", ""]),
            ("NO_LOG", (3, 25), (3, 26), vec!["SOURCES", "test.c", ""]),
            (
                "LOG_DESCRIPTION",
                (3, 25),
                (3, 26),
                vec!["SOURCES", "test.c", ""],
            ),
            (
                "LINK_OPTIONS",
                (3, 13),
                (3, 14),
                vec!["binary", "test.c", ""],
            ),
            (
                "LINKER_LANGUAGE",
                (3, 28),
                (3, 29),
                vec!["binary", "test.c", ""],
            ),
            ("CXX_STANDARD", (3, 7), (3, 8), vec!["binary", "test.c", ""]),
            (
                "OBJC_STANDARD",
                (3, 15),
                (3, 16),
                vec!["binary", "test.m", ""],
            ),
        ] {
            let mut prefix = initial.clone();
            prefix.extend(suffix);
            lacks(command, before, &prefix, word);
            has(command, after, &prefix, word);
        }
    }
    lacks(
        "try_run",
        (3, 1),
        &["run", "compile", "binary", "test.c", ""],
        "LINK_LIBRARIES",
    );
    has(
        "try_run",
        (3, 2),
        &["run", "compile", "binary", "test.c", ""],
        "LINK_LIBRARIES",
    );
    lacks(
        "try_run",
        (3, 19),
        &["run", "compile", "binary", "test.c", ""],
        "WORKING_DIRECTORY",
    );
    has(
        "try_run",
        (3, 20),
        &["run", "compile", "binary", "test.c", ""],
        "WORKING_DIRECTORY",
    );
}

/// Require both try_run result variables before options.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Contrast modern split stdout/stderr outputs with legacy OUTPUT_VARIABLE and require no keyword
/// completion inside the trailing ARGS payload.
///
/// # Flow
/// ```text
///   [modern / legacy try_run]
///    |
///   [probe output options]-------------> [trailing ARGS payload]
///                                       |
///                                      [check no keyword suggestions]
///                                       |
///                                      < checks hold? >
///                                       /         \
///                                     [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_try_run_order_outputs_and_opaque_args();
/// ```
#[test]
fn keyword_build_try_run_order_outputs_and_opaque_args() {
    let v = (4, 4);
    assert!(keywords("try_run", v, &[""]).is_empty());
    assert!(keywords("try_run", v, &["run", ""]).is_empty());
    for word in [
        "COMPILE_OUTPUT_VARIABLE",
        "RUN_OUTPUT_VARIABLE",
        "RUN_OUTPUT_STDOUT_VARIABLE",
        "RUN_OUTPUT_STDERR_VARIABLE",
        "WORKING_DIRECTORY",
        "ARGS",
    ] {
        has(
            "try_run",
            v,
            &["run", "compile", "SOURCES", "test.c", ""],
            word,
        );
    }
    lacks(
        "try_run",
        v,
        &["run", "compile", "SOURCES", "test.c", ""],
        "OUTPUT_VARIABLE",
    );
    has(
        "try_run",
        v,
        &["run", "compile", "binary", "test.c", ""],
        "OUTPUT_VARIABLE",
    );
    lacks(
        "try_run",
        v,
        &["run", "compile", "binary", "test.c", ""],
        "RUN_OUTPUT_STDOUT_VARIABLE",
    );
    assert!(
        keywords(
            "try_run",
            v,
            &[
                "run",
                "compile",
                "SOURCES",
                "test.c",
                "ARGS",
                "arbitrary",
                ""
            ]
        )
        .is_empty()
    );
}

/// Feed partial prefixes to build schemas and require variable, generic target, executable or
/// library roles in result, alias, reuse, install and export positions.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails.
///
/// # Algorithm
/// Feed partial prefixes to build schemas and require variable, generic target, executable or
/// library roles in result, alias, reuse, install and export positions.
///
/// # Flow
/// ```text
///   [build typed-slot prefixes]
///    |
///   [infer completion roles]-------------> [expected variable/target kind]
///                                         |
///                                        [check role membership]
///                                         |
///                                        < checks hold? >
///                                         /         \
///                                       [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_name_roles_are_preserved();
/// ```
#[test]
fn keyword_build_name_roles_are_preserved() {
    for (command, args, role) in [
        ("try_compile", vec![""], NameRole::Variable),
        ("try_run", vec!["run", ""], NameRole::Variable),
        (
            "try_compile",
            vec!["result", "SOURCES", "test.c", "OUTPUT_VARIABLE", ""],
            NameRole::Variable,
        ),
        (
            "try_compile",
            vec!["result", "SOURCE_FROM_VAR", "test.c", ""],
            NameRole::Variable,
        ),
        (
            "try_run",
            vec![
                "run",
                "compile",
                "SOURCES",
                "test.c",
                "RUN_OUTPUT_STDERR_VARIABLE",
                "",
            ],
            NameRole::Variable,
        ),
        ("build_command", vec![""], NameRole::Variable),
        ("build_command", vec!["cmd", "TARGET", ""], NameRole::Target),
        (
            "add_executable",
            vec!["alias", "ALIAS", ""],
            NameRole::Executable,
        ),
        ("add_library", vec!["alias", "ALIAS", ""], NameRole::Library),
        ("target_sources", vec![""], NameRole::Target),
        (
            "target_precompile_headers",
            vec!["app", "REUSE_FROM", ""],
            NameRole::Target,
        ),
        (
            "include_external_msproject",
            vec!["external", "file.csproj", "TYPE", "guid", ""],
            NameRole::Target,
        ),
        ("install", vec!["TARGETS", ""], NameRole::Target),
        (
            "export",
            vec!["SETUP", "set", "TARGET", ""],
            NameRole::Target,
        ),
    ] {
        assert!(
            completion_roles(&schema(command, (4, 4)), &args).contains(&role),
            "{command} {args:?}: missing {role:?}"
        );
    }
}

/// Search depth-first for the first matching Keyword and borrow its following sibling slice. Reads
/// schema nodes only, with no side effects.
///
/// # Arguments
/// - `kinds` `[in]`: Argument sequence searched recursively through optional, repeated,
///   intersection and choice groups.
/// - `word` `[in]`: Exact keyword spelling to locate.
///
/// # Returns
/// Some following siblings from the keyword's own containing sequence, possibly empty; None when no
/// keyword matches. The borrow is tied to kinds, not word.
///
/// # Algorithm
/// Search siblings in order, returning immediately for a literal match. Recursively search each
/// group or choice before continuing later siblings; the first nested match wins.
///
/// # Flow
/// ```text
///   [next sibling]
///         |
///  < matching literal? >
///     /          \
/// [return tail] [group/choice?]
///                    |
///             [search children]
///                    |
///               < found? >
///                /      \
///            [return] [next sibling]
///                         |
///                   [None at end]
/// ```
///
/// # Examples
/// Local test-module context:
/// ```text
/// let kinds = [K::Keyword("COPY_FILE_ERROR".into()), K::Out];
/// assert!(matches!(keyword_suffix(&kinds, "COPY_FILE_ERROR"), Some([K::Out])));
/// ```
fn keyword_suffix<'a>(kinds: &'a [K], word: &str) -> Option<&'a [K]> {
    for (index, kind) in kinds.iter().enumerate() {
        match kind {
            K::Keyword(keyword) if keyword == word => return Some(&kinds[index + 1..]),
            K::Optional(group) | K::Repeat { args: group, .. } | K::AllOf(group) => {
                if let Some(suffix) = keyword_suffix(group, word) {
                    return Some(suffix);
                }
            }
            K::Choice(forms) => {
                for form in forms {
                    if let Some(suffix) = keyword_suffix(form, word) {
                        return Some(suffix);
                    }
                }
            }
            _ => {}
        }
    }
    None
}

/// Inspect schema trees rather than only completions: generated-source operand types, copy
/// output/error slots, pre-3.11 source minima, configure/subdirectory input/output paths and
/// generic runtime search directories must retain their specific roles.
///
/// Only local values and analysis buffers are created or changed; no file, network or process side
/// effects.
///
/// # Arguments
/// None. The test builds its own fixture and expected results.
///
/// # Returns
/// `()` after all checks pass; no data or output parameter is returned.
///
/// # Panics
/// If a stated invariant fails or an unwrapped fixture/setup operation fails.
///
/// # Algorithm
/// Inspect schema trees rather than only completions: generated-source operand types, copy
/// output/error slots, pre-3.11 source minima, configure/subdirectory input/output paths and
/// generic runtime search directories must retain their specific roles.
///
/// # Flow
/// ```text
///   [build schema trees]
///    |
///   [find keyword sibling suffix]----------> [typed path/source operands]
///                                           |
///                                          [check exact roles + minima]
///                                           |
///                                          < checks hold? >
///                                           /         \
///                                         [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_build_required_operands_and_path_roles_are_structural();
/// ```
#[test]
fn keyword_build_required_operands_and_path_roles_are_structural() {
    for command in ["try_compile", "try_run"] {
        let kinds: Vec<_> = schema(command, (4, 4))
            .args
            .into_iter()
            .map(|arg| arg.kind)
            .collect();
        assert!(matches!(kinds[0], K::Out));
        if command == "try_run" {
            assert!(matches!(kinds[1], K::Out));
        }
        assert!(matches!(
            keyword_suffix(&kinds, "SOURCE_FROM_VAR").unwrap(),
            [K::String, K::Variable]
        ));
        assert!(matches!(
            keyword_suffix(&kinds, "SOURCE_FROM_FILE").unwrap(),
            [K::String, K::InputFile]
        ));
        assert!(matches!(
            keyword_suffix(&kinds, "SOURCE_FROM_CONTENT").unwrap(),
            [K::String, K::String]
        ));
        assert!(matches!(
            keyword_suffix(&kinds, "COPY_FILE").unwrap(),
            [K::OutputPath, K::Optional(_)]
        ));
        assert!(matches!(
            keyword_suffix(&kinds, "COPY_FILE_ERROR").unwrap(),
            [K::Out]
        ));
    }
    for command in ["add_executable", "add_library"] {
        let old = format!("{:?}", schema(command, (3, 10)).args);
        let new = format!("{:?}", schema(command, (3, 11)).args);
        assert!(old.contains("Repeat { args: [InputFile], min: 1 }"));
        assert!(new.contains("Repeat { args: [InputFile], min: 0 }"));
    }
    let configure: Vec<_> = schema("configure_file", (4, 4))
        .args
        .into_iter()
        .map(|arg| arg.kind)
        .collect();
    assert!(matches!(&configure[..2], [K::InputFile, K::OutputPath]));
    let directory: Vec<_> = schema("add_subdirectory", (4, 4))
        .args
        .into_iter()
        .map(|arg| arg.kind)
        .collect();
    assert!(matches!(&directory[0], K::InputDirectory));
    assert!(matches!(&directory[1], K::Optional(group) if matches!(&group[..], [K::OutputPath])));
    let install: Vec<_> = schema("install", (4, 4))
        .args
        .into_iter()
        .map(|arg| arg.kind)
        .collect();
    assert!(
        matches!(keyword_suffix(&install, "DIRECTORIES").unwrap(), [K::Repeat { args, min: 0 }] if matches!(&args[..], [K::Path]))
    );
    let custom: Vec<_> = schema("add_custom_command", (4, 4))
        .args
        .into_iter()
        .map(|arg| arg.kind)
        .collect();
    assert!(matches!(
        keyword_suffix(&custom, "DEPFILE").unwrap(),
        [K::OutputPath]
    ));
    for command in ["install_files", "install_programs", "install_targets"] {
        assert!(matches!(
            schema(command, (3, 0)).args[0].kind,
            K::OutputPath
        ));
    }
}
