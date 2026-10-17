//! Private process/CTest schema tests attached by `schema::builtin::all_process`. In-memory parsing
//! proves completion, output-variable and path roles; process names in fixtures are never executed.

// Contains AI-generated documentation edits.
// AI-generated tests.
use super::schemas;
use crate::parser::{
    lexer::model::{Lexer, RawBuffer},
    nparser::model::NParser,
    semanticer::{
        model::{FileId, PathKind, Semanticer},
        parsing::{completion_keywords, completion_roles},
        schema::{ArgSchemaKind as K, CommandSchema, NameRole},
    },
};
use std::collections::{BTreeSet, HashMap};

/// Find one enclosing-family grammar and wrap it with the shared builtin schema constructor.
/// Allocates schema data only; does not inspect the host or run CMake.
///
/// # Arguments
/// - `version` `[in]`: CMake (major, minor) selecting the grammar catalog.
/// - `name` `[in]`: Exact command name expected in that catalog.
///
/// # Returns
/// An owned CommandSchema from the shared make_schema helper, retaining the selected typed
/// arguments.
///
/// # Panics
/// If the requested command is absent at that version.
///
/// # Examples
/// Local test-module context:
/// ```text
/// let grammar = schema((4, 4), "execute_process");
/// assert!(!grammar.args.is_empty());
/// ```
fn schema(version: (u32, u32), name: &str) -> CommandSchema {
    let (_, args) = schemas(version)
        .into_iter()
        .find(|(word, _)| *word == name)
        .unwrap();
    super::super::make_schema(name, args)
}

/// Probe argument keyword completions with cancellation disabled and own the returned words. Calls
/// [`schema`] and the production matcher; no process or file side effects.
///
/// # Arguments
/// - `version` `[in]`: CMake (major, minor) selecting the schema.
/// - `name` `[in]`: Exact command name in this grammar family.
/// - `args` `[in]`: Argument prefix through the active final slot; an empty final string means no
///   letters typed there.
///
/// # Returns
/// Sorted, deduplicated owned keyword strings; empty when no literal fits the active slot.
///
/// # Panics
/// If the requested command is absent at that version.
///
/// # Examples
/// Local test-module context:
/// ```text
/// assert_eq!(keywords((4, 4), "execute_process", &["COM"]), ["COMMAND"]);
/// ```
fn keywords(version: (u32, u32), name: &str, args: &[&str]) -> Vec<String> {
    completion_keywords(&schema(version, name), args, || false)
        .into_iter()
        .map(str::to_owned)
        .collect()
}

/// Collect all explicit process-grammar literals into a sorted unique set. Traverses modeled groups
/// without executing the process names represented by the schema.
///
/// # Arguments
/// - `args` `[in]`: Argument sequence, recursively including optional/repeated/intersection groups
///   and choices.
///
/// # Returns
/// A new BTreeSet of owned keyword spellings; scalar roles contribute no words.
///
/// # Panics
/// If an Unmodeled node appears; other non-keyword leaves are ignored.
///
/// # Algorithm
/// Insert literal leaves, union recursively collected group/choice words and ignore typed scalars.
/// Unmodeled nodes fail the test immediately.
///
/// # Flow
/// ```text
///       [argument node]
///               |
///      +--------+--------+
///      |                 |
/// [group/choice]      [leaf]
///      |             /  |   \
///  [recurse]    keyword scalar unmodeled
///      |           |     |       |
///      +------> [insert] [skip] [panic]
///                  |
///           [sorted unique set]
/// ```
///
/// # Examples
/// Local test-module context:
/// ```text
/// assert!(literal_words(&[K::Keyword("COMMAND".into())]).contains("COMMAND"));
/// ```
fn literal_words(args: &[K]) -> BTreeSet<String> {
    let mut words = BTreeSet::new();
    for arg in args {
        match arg {
            K::Keyword(word) => {
                words.insert(word.clone());
            }
            K::Optional(args) | K::Repeat { args, .. } | K::AllOf(args) => {
                words.extend(literal_words(args))
            }
            K::Choice(forms) => {
                for form in forms {
                    words.extend(literal_words(form));
                }
            }
            K::Unmodeled => panic!("process grammars must not have unmodeled tails"),
            _ => {}
        }
    }
    words
}

/// Build representative required operands for process-grammar witnesses. Omits optional groups and
/// uses each repeat minimum and first choice; no external side effects.
///
/// # Arguments
/// - `args` `[in]`: Process-schema argument sequence to approximate with representative text.
///
/// # Returns
/// Owned strings containing literal spellings and operand placeholders. This is a witness helper,
/// not a general validator or exhaustive signature generator.
///
/// # Algorithm
/// Keep literal nodes, omit optionals, repeat representatives min times and choose the first
/// branch. Other nodes become one operand placeholder.
///
/// # Flow
/// ```text
///       [argument node]
///                |
///       +--------+---------+
///       |                  |
/// [group/repeat/choice]  [leaf/optional]
///       |                  |
/// [recurse as required] [literal/operand/omit]
///       |                  |
///       +--------+---------+
///                v
///       [concatenate arguments]
/// ```
///
/// # Examples
/// Local test-module context:
/// ```text
/// assert_eq!(minimum(&[K::Keyword("COMMAND".into()), K::Value]), ["COMMAND", "operand"]);
/// ```
fn minimum(args: &[K]) -> Vec<String> {
    args.iter()
        .flat_map(|arg| match arg {
            K::Keyword(word) => vec![word.clone()],
            K::Optional(_) => vec![],
            K::Repeat { args, min } => (0..*min).flat_map(|_| minimum(args)).collect(),
            K::Choice(forms) => forms.first().map_or_else(Vec::new, |form| minimum(form)),
            _ => vec!["operand".to_owned()],
        })
        .collect()
}

// Derive literal witnesses from structure, then check the real completion matcher.
/// Append literal-position witnesses by walking structured grammar branches. Each witness pairs
/// prior representative arguments with the next keyword; no I/O or input mutation.
///
/// # Arguments
/// - `args` `[in]`: Argument sequence walked recursively through groups and alternatives.
/// - `prefix` `[in]`: Owned preceding argument strings; extended locally as each node is passed.
/// - `result` `[in,out]`: Existing witness vector; append (preceding arguments, keyword) pairs
///   without clearing earlier entries.
///
/// # Returns
/// `()` after appending witnesses. Prefixes exclude the active keyword; callers append that word
/// before probing completion.
///
/// # Algorithm
/// For each node, record a literal or visit every branch with a cloned prefix. Then extend the
/// local prefix with minimum representatives before visiting following siblings.
///
/// # Flow
/// ```text
///       [node + prior prefix]
///                 |
///        +--------+--------+
///        |                 |
///    [keyword]        [group/choice]
///        |                 |
/// [append witness]  [visit branches]
///        |                 |
///        +--------+--------+
///                 v
///      [extend prefix with minimum]
///                 |
///           [next sibling]
/// ```
///
/// # Examples
/// Local test-module context:
/// ```text
/// let mut examples = Vec::new();
/// witnesses(&[K::Keyword("NAME".into())], vec![], &mut examples);
/// assert_eq!(examples, vec![(vec![], "NAME".into())]);
/// ```
fn witnesses(args: &[K], prefix: Vec<String>, result: &mut Vec<(Vec<String>, String)>) {
    let mut prefix = prefix;
    for arg in args {
        match arg {
            K::Keyword(word) => {
                result.push((prefix.clone(), word.clone()));
            }
            K::Optional(args) | K::Repeat { args, .. } => witnesses(args, prefix.clone(), result),
            K::Choice(forms) => {
                for form in forms {
                    witnesses(form, prefix.clone(), result);
                }
            }
            _ => {}
        }
        prefix.extend(minimum(std::slice::from_ref(arg)));
    }
}

/// Build minimal argument-prefix witnesses for each process schema literal across CMake 3.0–3.31
/// and 4.0–4.4.
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
/// Require completion to rediscover each literal at its own structured position.
///
/// # Flow
/// ```text
///   [all supported process schemas]
///    |
///   [generate literal witnesses]-------------> [production prefix matcher]
///                                             |
///                                            [require witnessed keyword]
///                                             |
///                                            < checks hold? >
///                                             /         \
///                                           [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_process_every_literal_is_reachable_in_its_structured_slot();
/// ```
#[test]
fn keyword_process_every_literal_is_reachable_in_its_structured_slot() {
    for version in (0..=31)
        .map(|minor| (3, minor))
        .chain((0..=4).map(|minor| (4, minor)))
    {
        for (name, args) in schemas(version) {
            let mut examples = Vec::new();
            witnesses(&args, vec![], &mut examples);
            let schema = super::super::make_schema(name, args);
            for (mut prefix, word) in examples {
                prefix.push(word.clone());
                let arguments: Vec<_> = prefix.iter().map(String::as_str).collect();
                assert!(
                    completion_keywords(&schema, &arguments, || false).contains(&word.as_str()),
                    "{version:?} {name} {prefix:?}: {word}"
                );
            }
        }
    }
}

/// Compare schemas just before/at process, CTest, host-query and instrumentation introductions.
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
/// Inspect literal catalogs and targeted PRESET/QUIET/CAPTURE_CMAKE_ERROR probes so newer words
/// cannot leak into older versions.
///
/// # Flow
/// ```text
///   [command/option gate tables]
///    |
///   [compare old/new literal catalogs]----------> [contextual preset/capture probes]
///                                                |
///                                               [check introductions]
///                                                |
///                                               < checks hold? >
///                                                /         \
///                                              [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_process_command_and_keyword_introduction_boundaries();
/// ```
#[test]
fn keyword_process_command_and_keyword_introduction_boundaries() {
    for (introduced, name) in [
        ((3, 18), "cmake_language"),
        ((3, 27), "cmake_file_api"),
        ((3, 31), "cmake_pkg_config"),
        ((4, 3), "cmake_instrumentation"),
        ((4, 4), "discover_tests"),
        ((4, 4), "cmake_diagnostic"),
    ] {
        let before = if introduced.1 == 0 {
            (3, 31)
        } else {
            (introduced.0, introduced.1 - 1)
        };
        assert!(!schemas(before).iter().any(|(word, _)| *word == name));
        assert!(schemas(introduced).iter().any(|(word, _)| *word == name));
    }
    for (name, word, introduced) in [
        ("execute_process", "ENCODING", (3, 8)),
        ("execute_process", "RESULTS_VARIABLE", (3, 10)),
        ("execute_process", "UTF-8", (3, 11)),
        ("execute_process", "COMMAND_ECHO", (3, 15)),
        ("execute_process", "ECHO_OUTPUT_VARIABLE", (3, 18)),
        ("execute_process", "ECHO_ERROR_VARIABLE", (3, 18)),
        ("execute_process", "COMMAND_ERROR_IS_FATAL", (3, 19)),
        ("execute_process", "ENVIRONMENT", (4, 4)),
        ("execute_process", "ENVIRONMENT_MODIFICATION", (4, 4)),
        ("ctest_build", "PARALLEL_LEVEL", (3, 21)),
        ("ctest_memcheck", "DEFECT_COUNT", (3, 8)),
        ("ctest_test", "TEST_LOAD", (3, 4)),
        ("ctest_test", "EXCLUDE_FIXTURE", (3, 7)),
        ("ctest_test", "RESOURCE_SPEC_FILE", (3, 16)),
        ("ctest_test", "REPEAT", (3, 17)),
        ("ctest_test", "STOP_ON_FAILURE", (3, 18)),
        ("ctest_test", "OUTPUT_JUNIT", (3, 21)),
        ("ctest_test", "INCLUDE_FROM_FILE", (3, 29)),
        ("ctest_memcheck", "EXCLUDE_FROM_FILE", (3, 29)),
        ("ctest_test", "COVERAGE_TOOL", (4, 4)),
        ("ctest_start", "GROUP", (3, 16)),
        ("ctest_submit", "CDASH_UPLOAD", (3, 2)),
        ("ctest_submit", "HTTPHEADER", (3, 9)),
        ("ctest_submit", "SUBMIT_URL", (3, 14)),
        ("ctest_submit", "Done", (3, 14)),
        ("ctest_submit", "BUILD_ID", (3, 15)),
        ("ctest_update", "VERSION_ONLY", (4, 4)),
        ("ctest_update", "VERSION_OVERRIDE", (4, 4)),
        ("cmake_host_system_information", "IS_64BIT", (3, 10)),
        ("cmake_host_system_information", "DISTRIB_INFO", (3, 22)),
        ("cmake_host_system_information", "WINDOWS_REGISTRY", (3, 24)),
        ("cmake_host_system_information", "MSYSTEM_PREFIX", (3, 28)),
        ("cmake_host_system_information", "LOCALE_CHARSET", (4, 3)),
        ("cmake_host_system_information", "HAS_APIC", (4, 3)),
        ("cmake_language", "DEFER", (3, 19)),
        ("cmake_language", "SET_DEPENDENCY_PROVIDER", (3, 24)),
        ("cmake_language", "GET_MESSAGE_LOG_LEVEL", (3, 25)),
        ("cmake_language", "EXIT", (3, 29)),
        ("cmake_language", "TRACE", (4, 2)),
        ("cmake_pkg_config", "POPULATE", (4, 1)),
        ("cmake_pkg_config", "IMPORT", (4, 1)),
        ("cmake_instrumentation", "captureOutput", (4, 4)),
        ("cmake_instrumentation", "compileTrace", (4, 4)),
    ] {
        let before = (introduced.0, introduced.1 - 1);
        let old = schema(before, name);
        let new = schema(introduced, name);
        let old: Vec<_> = old.args.into_iter().map(|arg| arg.kind).collect();
        let new: Vec<_> = new.args.into_iter().map(|arg| arg.kind).collect();
        assert!(
            !literal_words(&old).contains(word),
            "{name} {word} before {introduced:?}"
        );
        assert!(
            literal_words(&new).contains(word),
            "{name} {word} at {introduced:?}"
        );
    }
    for name in [
        "ctest_build",
        "ctest_configure",
        "ctest_test",
        "ctest_memcheck",
    ] {
        for word in ["PRESET", "PRESETS_FILE"] {
            assert!(!keywords((4, 3), name, &[word]).contains(&word.to_owned()));
            assert!(keywords((4, 4), name, &[word]).contains(&word.to_owned()));
        }
    }
    for name in [
        "ctest_build",
        "ctest_configure",
        "ctest_coverage",
        "ctest_memcheck",
        "ctest_start",
        "ctest_submit",
        "ctest_test",
        "ctest_update",
    ] {
        let before = schema((3, 2), name);
        assert!(
            !literal_words(
                &before
                    .args
                    .into_iter()
                    .map(|arg| arg.kind)
                    .collect::<Vec<_>>()
            )
            .contains("QUIET")
        );
    }
    for (name, minor) in [
        ("ctest_build", 7),
        ("ctest_configure", 7),
        ("ctest_coverage", 7),
        ("ctest_memcheck", 7),
        ("ctest_test", 7),
        ("ctest_upload", 7),
        ("ctest_submit", 13),
        ("ctest_update", 13),
    ] {
        for (version, expected) in [((3, minor - 1), false), ((3, minor), true)] {
            let schema = schema(version, name);
            assert_eq!(
                literal_words(
                    &schema
                        .args
                        .into_iter()
                        .map(|arg| arg.kind)
                        .collect::<Vec<_>>()
                )
                .contains("CAPTURE_CMAKE_ERROR"),
                expected,
                "{name} {version:?}"
            );
        }
    }
}

/// Probe command/script/registry/package required operands and cmake_language modes.
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
/// Require no keywords inside opaque values, mode-specific options and correct
/// variable/executable/target completion roles.
///
/// # Flow
/// ```text
///   [process mode/scalar prefixes]
///    |
///   [probe keywords + name roles]-----------> [opaque required values]
///                                            |
///                                           [check empty or typed result]
///                                            |
///                                           < checks hold? >
///                                            /         \
///                                          [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_process_required_operands_modes_and_opaque_values();
/// ```
#[test]
fn keyword_process_required_operands_modes_and_opaque_values() {
    for (name, prefix) in [
        ("execute_process", vec!["COMMAND", ""]),
        ("exec_program", vec![""]),
        ("cmake_host_system_information", vec!["RESULT", ""]),
        (
            "cmake_host_system_information",
            vec!["RESULT", "res", "QUERY", "WINDOWS_REGISTRY", ""],
        ),
        ("cmake_file_api", vec!["QUERY", "API_VERSION", ""]),
        ("cmake_instrumentation", vec!["API_VERSION", ""]),
        ("cmake_diagnostic", vec!["SET", ""]),
        ("cmake_language", vec!["CALL", ""]),
        ("cmake_language", vec!["CALL", "user_command", ""]),
        ("cmake_language", vec!["EVAL", "CODE", "script", ""]),
        ("cmake_pkg_config", vec!["EXTRACT", ""]),
        ("discover_tests", vec!["COMMAND", ""]),
    ] {
        assert!(
            keywords((4, 4), name, &prefix).is_empty(),
            "{name} {prefix:?}"
        );
    }
    assert_eq!(keywords((4, 4), "cmake_language", &["EVAL", ""]), ["CODE"]);
    assert_eq!(
        keywords((4, 4), "cmake_language", &["TRACE", "OFF", ""]),
        Vec::<String>::new()
    );
    assert_eq!(
        keywords((4, 4), "cmake_language", &["TRACE", "ON", ""]),
        ["EXPAND"]
    );
    assert!(!keywords((3, 11), "ctest_start", &["APPEND"]).contains(&"APPEND".to_owned()));
    assert!(keywords((3, 12), "ctest_start", &["APPEND"]).contains(&"APPEND".to_owned()));
    assert!(
        !keywords((3, 28), "ctest_test", &["PARALLEL_LEVEL", "QUIET"])
            .contains(&"QUIET".to_owned())
    );
    assert!(
        keywords((3, 29), "ctest_test", &["PARALLEL_LEVEL", "QUIET"]).contains(&"QUIET".to_owned())
    );
    assert!(
        !keywords((4, 4), "cmake_pkg_config", &["EXTRACT", "package", "NAME"])
            .contains(&"NAME".to_owned())
    );
    assert!(
        keywords((4, 4), "cmake_pkg_config", &["IMPORT", "package", "NAME"])
            .contains(&"NAME".to_owned())
    );
    assert!(
        !keywords(
            (4, 4),
            "cmake_pkg_config",
            &["POPULATE", "package", "SYSTEM_INCLUDE_DIRS"]
        )
        .contains(&"SYSTEM_INCLUDE_DIRS".to_owned())
    );
    assert!(
        !keywords((4, 4), "ctest_submit", &["CDASH_UPLOAD", "file", "PARTS"])
            .contains(&"PARTS".to_owned())
    );
    assert_eq!(
        completion_roles(&schema((4, 4), "cmake_language"), &["DEFER", "ID_VAR", ""]),
        [NameRole::Variable]
    );
    assert_eq!(
        completion_roles(&schema((4, 4), "discover_tests"), &["COMMAND", ""]),
        [NameRole::Executable]
    );
    assert_eq!(
        completion_roles(&schema((4, 4), "ctest_build"), &["TARGET", ""]),
        [NameRole::Target]
    );
}

/// Parse a two-command fixture with an explicit required version and injected process schema.
/// Creates local lexer/parser/semantic state only; does not run the named command.
///
/// # Arguments
/// - `version` `[in]`: CMake (major, minor) written into the header and used for schema lookup.
/// - `name` `[in]`: Exact process-family command name inserted after the header.
/// - `args` `[in]`: Raw CMake argument text inserted inside the command parentheses, not shell
///   arguments.
///
/// # Returns
/// Semantic analysis for file 1; command index 1 is the supplied command, including diagnostics,
/// output writes and path roles.
///
/// # Panics
/// If the requested command has no schema at that version.
///
/// # Algorithm
/// Format the version header and one command, lex/parse it, then inject this family schema into a
/// fresh semantic analyzer before parsing as file 1.
///
/// # Flow
/// ```text
/// [version/name/args]
///             |
///     [two-command source]
///             |
///        [lex/parse] -------> [semantic analyzer]
///                                   ^       |
///        [injected schema] ---------+       v
///                                     [file-1 buffer]
/// ```
///
/// # Examples
/// Local test-module context (await inside an async test):
/// ```text
/// let buffer = analyze((4, 4), "execute_process", "COMMAND tool OUTPUT_VARIABLE result").await;
/// assert_eq!(buffer.commands[1].name, "execute_process");
/// ```
async fn analyze(
    version: (u32, u32),
    name: &str,
    args: &str,
) -> crate::parser::semanticer::model::SemanticedBuffer {
    let source = format!(
        "cmake_minimum_required(VERSION {}.{})\n{name}({args})\n",
        version.0, version.1
    );
    let lexed = Lexer::new(RawBuffer::new(&source)).parse();
    let parsed = NParser::new(lexed).parse(&source);
    Semanticer::new(
        parsed,
        HashMap::from([(name.to_owned(), schema(version, name))]),
    )
    .parse(&source, FileId { id: 1 })
    .await
}

/// Parse complete 4.4 process, CTest, host-query, deferred-language and diagnostic fixtures with
/// injected schemas.
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
/// Compare exactly the names with recorded writes against expected result/output variables; no
/// fixture tool is run.
///
/// # Flow
/// ```text
///   [complete process call table]
///    |
///   [lex/parse injected schemas]-----------> [variable changing records]
///                                           |
///                                          [compare exact output-name set]
///                                           |
///                                          < checks hold? >
///                                           /         \
///                                         [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// keyword_process_preserves_full_call_output_roles();
/// ```
#[tokio::test]
async fn keyword_process_preserves_full_call_output_roles() {
    for (name, args, outputs) in [
        (
            "execute_process",
            "COMMAND tool arg COMMAND other RESULT_VARIABLE result RESULTS_VARIABLE results OUTPUT_VARIABLE output ERROR_VARIABLE error",
            vec!["result", "results", "output", "error"],
        ),
        (
            "exec_program",
            "tool work ARGS user_args OUTPUT_VARIABLE output RETURN_VALUE result",
            vec!["output", "result"],
        ),
        (
            "ctest_build",
            "NUMBER_ERRORS errors NUMBER_WARNINGS warnings RETURN_VALUE result CAPTURE_CMAKE_ERROR captured",
            vec!["errors", "warnings", "result", "captured"],
        ),
        (
            "ctest_configure",
            "RETURN_VALUE result CAPTURE_CMAKE_ERROR captured",
            vec!["result", "captured"],
        ),
        (
            "ctest_coverage",
            "LABELS one two RETURN_VALUE result CAPTURE_CMAKE_ERROR captured",
            vec!["result", "captured"],
        ),
        (
            "ctest_memcheck",
            "DEFECT_COUNT defects RETURN_VALUE result CAPTURE_CMAKE_ERROR captured",
            vec!["defects", "result", "captured"],
        ),
        (
            "ctest_test",
            "PARALLEL_LEVEL RETURN_VALUE result CAPTURE_CMAKE_ERROR captured",
            vec!["result", "captured"],
        ),
        (
            "ctest_run_script",
            "NEW_PROCESS one.cmake two.cmake RETURN_VALUE result",
            vec!["result"],
        ),
        (
            "ctest_submit",
            "PARTS Test Done BUILD_ID id RETURN_VALUE result CAPTURE_CMAKE_ERROR captured",
            vec!["id", "result", "captured"],
        ),
        (
            "ctest_submit",
            "CDASH_UPLOAD file CDASH_UPLOAD_TYPE type BUILD_ID id RETURN_VALUE result",
            vec!["id", "result"],
        ),
        (
            "ctest_update",
            "VERSION_ONLY VERSION_OVERRIDE revision RETURN_VALUE result CAPTURE_CMAKE_ERROR captured",
            vec!["result", "captured"],
        ),
        (
            "ctest_upload",
            "FILES one two CAPTURE_CMAKE_ERROR captured",
            vec!["captured"],
        ),
        (
            "cmake_host_system_information",
            "RESULT result QUERY WINDOWS_REGISTRY HKLM VALUE name VIEW HOST ERROR_VARIABLE error",
            vec!["result", "error"],
        ),
        (
            "cmake_host_system_information",
            "RESULT result QUERY DISTRIB_PRETTY_NAME",
            vec!["result"],
        ),
        (
            "cmake_language",
            "DEFER DIRECTORY dir ID_VAR id CALL tool user_args",
            vec!["id"],
        ),
        (
            "cmake_language",
            "DEFER DIRECTORY dir GET_CALL_IDS ids",
            vec!["ids"],
        ),
        ("cmake_language", "DEFER GET_CALL id call", vec!["call"]),
        (
            "cmake_language",
            "GET_MESSAGE_LOG_LEVEL level",
            vec!["level"],
        ),
        (
            "cmake_diagnostic",
            "GET CMD_CATEGORY action",
            vec!["action"],
        ),
    ] {
        let result = analyze((4, 4), name, args).await;
        assert!(
            result.diagnostics.is_empty(),
            "{name}: {:?}",
            result.diagnostics
        );
        let actual: BTreeSet<_> = result
            .variables
            .iter()
            .filter(|var| !var.changing.is_empty())
            .map(|var| var.name.as_str())
            .collect();
        assert_eq!(actual, outputs.into_iter().collect(), "{name}({args})");
    }
}

/// Parse complete process/CTest/package calls and compare indexed file, directory and output-path
/// positions.
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
/// Search hints must not become required input paths; invalid mixed signatures must record no
/// variable writes.
///
/// # Flow
/// ```text
///   [complete process/path calls]
///    |
///   [analyze typed paths]------------------> [invalid signature cases]
///                                           |
///                                          [compare paths; require no writes]
///                                           |
///                                          < checks hold? >
///                                           /         \
///                                         [pass]     [panic]
/// ```
///
/// # Examples
/// Direct local call outside any Tokio runtime (the test attribute creates its runtime):
/// ```text
/// keyword_process_preserves_paths_and_does_not_require_search_hints_to_exist();
/// ```
#[tokio::test]
async fn keyword_process_preserves_paths_and_does_not_require_search_hints_to_exist() {
    for (name, args, expected) in [
        (
            "execute_process",
            "COMMAND tool INPUT_FILE input OUTPUT_FILE output ERROR_FILE error WORKING_DIRECTORY work",
            vec![
                (3, PathKind::File),
                (5, PathKind::Output),
                (7, PathKind::Output),
                (9, PathKind::Directory),
            ],
        ),
        (
            "ctest_configure",
            "BUILD build SOURCE source PRESETS_FILE presets.json",
            vec![
                (1, PathKind::Directory),
                (3, PathKind::Directory),
                (5, PathKind::File),
            ],
        ),
        (
            "ctest_test",
            "INCLUDE_FROM_FILE names.txt RESOURCE_SPEC_FILE resource.json OUTPUT_JUNIT junit.xml",
            vec![
                (1, PathKind::File),
                (3, PathKind::File),
                (5, PathKind::Output),
            ],
        ),
        (
            "ctest_empty_binary_directory",
            "build",
            vec![(0, PathKind::Directory)],
        ),
        (
            "ctest_read_custom_files",
            "one two",
            vec![(0, PathKind::Directory), (1, PathKind::Directory)],
        ),
        (
            "ctest_start",
            "GROUP group Experimental QUIET source APPEND binary",
            vec![(4, PathKind::Directory), (6, PathKind::Directory)],
        ),
        (
            "cmake_pkg_config",
            "EXTRACT package PC_LIBDIR hint PC_PATH hint2 PC_SYSROOT_DIR root TOP_BUILD_DIR build SYSTEM_INCLUDE_DIRS includes",
            vec![],
        ),
    ] {
        let result = analyze((4, 4), name, args).await;
        let mut actual: Vec<_> = result.commands[1]
            .paths
            .iter()
            .map(|path| (path.argument, path.kind))
            .collect();
        actual.sort_by_key(|(index, _)| *index);
        assert_eq!(actual, expected, "{name}({args})");
    }
    for (name, args) in [
        ("cmake_language", "DEFER ID_VAR id GET_CALL_IDS ids"),
        ("cmake_diagnostic", "GET CMD_CATEGORY action RECURSE"),
        (
            "ctest_submit",
            "CDASH_UPLOAD file PARTS Test RETURN_VALUE result",
        ),
        ("execute_process", "OUTPUT_VARIABLE output"),
    ] {
        let result = analyze((4, 4), name, args).await;
        assert!(
            result.variables.iter().all(|var| var.changing.is_empty()),
            "invalid signature: {name}({args})"
        );
    }
}

/// Probe execute_process option values after an opaque COMMAND payload.
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
/// Check versioned fatal-error/encoding/echo choices and no invented keywords for output-variable
/// or working-directory scalars.
///
/// # Flow
/// ```text
///   [versioned execute options]
///    |
///   [probe after COMMAND payload]----------> [scalar output/path options]
///                                           |
///                                          [compare choices or empty result]
///                                           |
///                                          < checks hold? >
///                                           /         \
///                                         [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_process_opaque_command_options_keep_required_value_slots();
/// ```
#[test]
fn keyword_process_opaque_command_options_keep_required_value_slots() {
    for (version, option, expected) in [
        ((3, 19), "COMMAND_ERROR_IS_FATAL", vec!["ANY", "LAST"]),
        (
            (4, 0),
            "COMMAND_ERROR_IS_FATAL",
            vec!["ANY", "LAST", "NONE"],
        ),
        (
            (3, 8),
            "ENCODING",
            vec!["ANSI", "AUTO", "NONE", "OEM", "UTF8"],
        ),
        (
            (3, 11),
            "ENCODING",
            vec!["ANSI", "AUTO", "NONE", "OEM", "UTF-8", "UTF8"],
        ),
        ((3, 15), "COMMAND_ECHO", vec!["NONE", "STDERR", "STDOUT"]),
        ((4, 4), "OUTPUT_VARIABLE", vec![]),
        ((4, 4), "WORKING_DIRECTORY", vec![]),
    ] {
        assert_eq!(
            keywords(version, "execute_process", &["COMMAND", "tool", option, ""]),
            expected,
            "{version:?} {option}"
        );
    }
}

/// Compare the 4.4 process schema names with the explicit 22-command set and walk every grammar to
/// reject unmodeled tails.
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
/// Compare the 4.4 process schema names with the explicit 22-command set and walk every grammar to
/// reject unmodeled tails.
///
/// # Flow
/// ```text
///   [4.4 process catalog]
///    |
///   [compare exact 22-name set]----------> [walk argument literals]
///                                         |
///                                        [reject unmodeled tails]
///                                         |
///                                        < checks hold? >
///                                         /         \
///                                       [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_process_owns_all_assigned_commands_without_unmodeled_tails();
/// ```
#[test]
fn keyword_process_owns_all_assigned_commands_without_unmodeled_tails() {
    let commands = schemas((4, 4));
    let names: BTreeSet<_> = commands.iter().map(|(name, _)| *name).collect();
    assert_eq!(
        names,
        BTreeSet::from([
            "execute_process",
            "exec_program",
            "ctest_build",
            "ctest_configure",
            "ctest_coverage",
            "ctest_empty_binary_directory",
            "ctest_memcheck",
            "ctest_read_custom_files",
            "ctest_run_script",
            "ctest_sleep",
            "ctest_start",
            "ctest_submit",
            "ctest_test",
            "ctest_update",
            "ctest_upload",
            "cmake_host_system_information",
            "cmake_instrumentation",
            "cmake_language",
            "cmake_file_api",
            "cmake_pkg_config",
            "discover_tests",
            "cmake_diagnostic",
        ])
    );
    for (_, args) in commands {
        literal_words(&args);
    }
}

/// Probe CDASH_UPLOAD option introductions and reject unsupported capture/TLS spellings.
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
/// Compare minimal ctest_run_script operands before/after 4.0, when at least one script becomes
/// required.
///
/// # Flow
/// ```text
///   [legacy CDASH_UPLOAD prefixes]
///    |
///   [check option introductions]------------> [ctest_run_script minima]
///                                            |
///                                           [compare pre/post 4.0]
///                                            |
///                                           < checks hold? >
///                                            /         \
///                                          [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_process_legacy_submission_and_script_boundaries();
/// ```
#[test]
fn keyword_process_legacy_submission_and_script_boundaries() {
    for (word, introduced) in [
        ("RETRY_COUNT", 8),
        ("RETRY_DELAY", 8),
        ("QUIET", 8),
        ("HTTPHEADER", 9),
        ("RETURN_VALUE", 13),
        ("SUBMIT_URL", 14),
        ("BUILD_ID", 15),
    ] {
        for (minor, expected) in [(introduced - 1, false), (introduced, true)] {
            assert_eq!(
                keywords((3, minor), "ctest_submit", &["CDASH_UPLOAD", "file", word])
                    .contains(&word.to_owned()),
                expected,
                "{word} at 3.{minor}"
            );
        }
    }
    assert!(
        keywords(
            (4, 4),
            "ctest_submit",
            &["CDASH_UPLOAD", "file", "CAPTURE_CMAKE_ERROR"]
        )
        .is_empty()
    );
    assert!(keywords((4, 4), "ctest_submit", &["TLSVersion"]).is_empty());
    assert!(keywords((4, 4), "ctest_submit", &["TLSVerify"]).is_empty());
    let old = schema((3, 31), "ctest_run_script");
    let new = schema((4, 0), "ctest_run_script");
    assert!(minimum(&old.args.into_iter().map(|arg| arg.kind).collect::<Vec<_>>()).is_empty());
    assert_eq!(
        minimum(&new.args.into_iter().map(|arg| arg.kind).collect::<Vec<_>>()).len(),
        1
    );
}
