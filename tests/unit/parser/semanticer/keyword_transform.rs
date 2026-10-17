//! Private file/string/list/path/math schema tests attached by `schema::builtin::all_transform`.
//! Synthetic commands and a sentinel path probe full matching without performing the represented
//! operations.

// Contains AI-generated documentation edits.
// AI-generated tests.
use super::schemas;
use crate::{
    model::TextSpan,
    parser::{
        nparser::model::NParsedBuffer,
        semanticer::{
            model::{
                ArgKind, BuiltinCommand, CommandId, CommandKind, PathKind, SemanticArg,
                SemanticArgForm, SemanticCommandRepresent, SemanticedBuffer, Semanticer,
            },
            parsing::outputs::{
                completion_keywords, completion_roles, path_arguments, target_arguments,
            },
            schema::{ArgSchema, ArgSchemaKind as K, CommandSchema, CommandSchemaFrom, NameRole},
        },
    },
};
use std::{collections::BTreeSet, sync::Arc};

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
/// let grammar = schema("file", (4, 4));
/// assert!(!grammar.args.is_empty());
/// ```
fn schema(command: &str, version: (u32, u32)) -> CommandSchema {
    let args = schemas(version)
        .into_iter()
        .find(|(name, _)| *name == command)
        .unwrap()
        .1;
    CommandSchema {
        command_name: command.to_owned(),
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
/// assert!(keywords("file", (4, 4), &["RE"]).contains(&"READ".to_owned()));
/// ```
fn keywords(command: &str, version: (u32, u32), prefix: &[&str]) -> Vec<String> {
    completion_keywords(&schema(command, version), prefix, || false)
        .into_iter()
        .map(str::to_owned)
        .collect()
}

/// Construct a closed synthetic semantic command for matcher/role tests without lexing source.
/// Assigns bracket form to every argument so values stay literal; only local allocation occurs.
///
/// # Arguments
/// - `schema` `[in]`: Owned schema supplying command name/kind and stored behind Arc.
/// - `arguments` `[in]`: Literal payload strings copied into arguments in order; each receives a
///   synthetic one-byte span at its index.
///
/// # Returns
/// A command with ID 0, no block/scope/documentation/references/paths and empty initial argument
/// roles. Its synthetic spans are not real source locations.
///
/// # Examples
/// Local test-module context:
/// ```text
/// let call = command(schema("file", (4, 4)), &["READ", "in", "out"]);
/// assert_eq!(call.args[1].content, "in");
/// ```
fn command(schema: CommandSchema, arguments: &[&str]) -> SemanticCommandRepresent {
    SemanticCommandRepresent {
        name: schema.command_name.clone(),
        kind: schema.kind.clone(),
        schema: Some(Arc::new(schema)),
        args: arguments
            .iter()
            .enumerate()
            .map(|(i, text)| SemanticArg {
                content: (*text).to_owned(),
                span: TextSpan {
                    start_byte: i,
                    end_byte: i + 1,
                    ..Default::default()
                },
                form: SemanticArgForm::Bracket,
                kind: vec![],
            })
            .collect(),
        id: CommandId { id: 0 },
        documentation: None,
        span: TextSpan::default(),
        full_span: TextSpan::default(),
        variable_references: vec![],
        paths: vec![],
        closed: true,
        block: None,
        scope: None,
    }
}

// A required final path probes full matching using the production matcher, not prefix matching.
/// Probe full-call grammar matching by appending a required input-file sentinel. Changes a local
/// schema and argument vector only; does not read the sentinel or execute the command.
///
/// # Arguments
/// - `name` `[in]`: Transform-family command to look up.
/// - `version` `[in]`: CMake (major, minor) selecting its grammar.
/// - `args` `[in]`: Complete candidate arguments before the sentinel is added.
///
/// # Returns
/// True when path extraction recognizes the final sentinel as a file at its appended index,
/// indicating full-call acceptance; false otherwise.
///
/// # Panics
/// If the command is absent at the version.
///
/// # Algorithm
/// Append InputFile to the schema and sentinel to the values, construct a bracket-form command and
/// ask production path extraction whether the final slot matches.
///
/// # Flow
/// ```text
///    [schema]                 [candidate args]
///        |                           |
/// [append InputFile]          [append sentinel]
///        |                           |
///        +-------------+-------------+
///                      v
///             [path_arguments]
///                      |
///             < final file slot? >
///                 /         \
///             [true]       [false]
/// ```
///
/// # Examples
/// Local test-module context:
/// ```text
/// assert!(accepts("file", (4, 4), &["READ", "input", "result"]));
/// assert!(!accepts("file", (4, 4), &["READ", "input"]));
/// ```
fn accepts(name: &str, version: (u32, u32), args: &[&str]) -> bool {
    let mut schema = schema(name, version);
    schema.args.push(ArgSchema { kind: K::InputFile });
    let mut args = args.to_vec();
    args.push("sentinel");
    path_arguments(&command(schema, &args))
        .iter()
        .any(|path| path.argument == args.len() - 1 && path.kind == PathKind::File)
}

/// Compare complete root keyword sets for file, string, list, cmake_path and math at 4.4.
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
/// Set comparison catches both missing and stray operations.
///
/// # Flow
/// ```text
///   [five transform families]
///    |
///   [probe root completion]------------> [explicit operation-name sets]
///                                       |
///                                      [compare full unique sets]
///                                       |
///                                      < checks hold? >
///                                       /         \
///                                     [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_transform_complete_root_subcommands();
/// ```
#[test]
fn keyword_transform_complete_root_subcommands() {
    for (name, expected) in [
        (
            "file",
            "APPEND ARCHIVE_CREATE ARCHIVE_EXTRACT CHMOD CHMOD_RECURSE CONFIGURE COPY COPY_FILE CREATE_LINK DOWNLOAD GENERATE GET_RUNTIME_DEPENDENCIES GLOB GLOB_RECURSE INSTALL LOCK MAKE_DIRECTORY MD5 READ READ_SYMLINK REAL_PATH RELATIVE_PATH REMOVE REMOVE_RECURSE RENAME SHA1 SHA224 SHA256 SHA384 SHA3_224 SHA3_256 SHA3_384 SHA3_512 SHA512 SIZE STRINGS TIMESTAMP TOUCH TOUCH_NOCREATE TO_CMAKE_PATH TO_NATIVE_PATH UPLOAD WRITE",
        ),
        (
            "string",
            "APPEND ASCII COMPARE CONCAT CONFIGURE FIND GENEX_STRIP HEX JOIN JSON LENGTH MAKE_C_IDENTIFIER MD5 PREPEND RANDOM REGEX REPEAT REPLACE SHA1 SHA224 SHA256 SHA384 SHA3_224 SHA3_256 SHA3_384 SHA3_512 SHA512 STRIP SUBSTRING TIMESTAMP TOLOWER TOUPPER UUID",
        ),
        (
            "list",
            "APPEND FILTER FIND GET INSERT JOIN LENGTH POP_BACK POP_FRONT PREPEND REMOVE_AT REMOVE_DUPLICATES REMOVE_ITEM REVERSE SORT SUBLIST TRANSFORM",
        ),
        (
            "cmake_path",
            "ABSOLUTE_PATH APPEND APPEND_STRING COMPARE CONVERT GET HASH HAS_EXTENSION HAS_FILENAME HAS_PARENT_PATH HAS_RELATIVE_PART HAS_ROOT_DIRECTORY HAS_ROOT_NAME HAS_ROOT_PATH HAS_STEM IS_ABSOLUTE IS_PREFIX IS_RELATIVE NATIVE_PATH NORMAL_PATH RELATIVE_PATH REMOVE_EXTENSION REMOVE_FILENAME REPLACE_EXTENSION REPLACE_FILENAME SET",
        ),
        ("math", "EXPR"),
    ] {
        let actual: BTreeSet<_> = keywords(name, (4, 4), &[""]).into_iter().collect();
        let expected: BTreeSet<_> = expected.split_whitespace().map(str::to_owned).collect();
        assert_eq!(actual, expected, "{name}");
    }
}

/// Probe file operation-specific options, encodings, permissions, archive and transfer forms.
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
/// Require download-only hash/range options to stay scoped, while a non-enumerated archive encoding
/// remains accepted by full matching.
///
/// # Flow
/// ```text
///   [file operation prefix table]
///    |
///   [probe contextual options]-------------> [download / archive variants]
///                                           |
///                                          [check choices + full acceptance]
///                                           |
///                                          < checks hold? >
///                                           /         \
///                                         [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_transform_file_option_slots();
/// ```
#[test]
fn keyword_transform_file_option_slots() {
    for (prefix, expected) in [
        (vec!["READ", "in", "out", ""], "HEX LIMIT OFFSET"),
        (
            vec!["STRINGS", "in", "out", ""],
            "ENCODING LENGTH_MAXIMUM LENGTH_MINIMUM LIMIT_COUNT LIMIT_INPUT LIMIT_OUTPUT NEWLINE_CONSUME NO_HEX_CONVERSION REGEX",
        ),
        (
            vec!["STRINGS", "in", "out", "ENCODING", ""],
            "UTF-16BE UTF-16LE UTF-32BE UTF-32LE UTF-8",
        ),
        (vec!["GENERATE", "OUTPUT", "out", ""], "CONTENT INPUT"),
        (
            vec!["GENERATE", "OUTPUT", "out", "INPUT", "in", ""],
            "CONDITION FILE_PERMISSIONS NEWLINE_STYLE NO_SOURCE_PERMISSIONS TARGET USE_SOURCE_PERMISSIONS",
        ),
        (
            vec!["GENERATE", "OUTPUT", "out", "CONTENT", "body", ""],
            "CONDITION FILE_PERMISSIONS NEWLINE_STYLE NO_SOURCE_PERMISSIONS TARGET",
        ),
        (
            vec!["CONFIGURE", "OUTPUT", "out", "CONTENT", "body", ""],
            "@ONLY ESCAPE_QUOTES NEWLINE_STYLE",
        ),
        (
            vec!["GLOB", "out", ""],
            "CONFIGURE_DEPENDS LIST_DIRECTORIES RELATIVE",
        ),
        (
            vec!["GLOB_RECURSE", "out", ""],
            "CONFIGURE_DEPENDS FOLLOW_SYMLINKS LIST_DIRECTORIES RELATIVE",
        ),
        (vec!["RENAME", "old", "new", ""], "NO_REPLACE RESULT"),
        (
            vec!["COPY_FILE", "old", "new", ""],
            "INPUT_MAY_BE_RECENT ONLY_IF_DIFFERENT RESULT",
        ),
        (vec!["READ_SYMLINK", "link", "out", ""], "RESULT"),
        (
            vec!["CREATE_LINK", "src", "link", ""],
            "COPY_ON_ERROR RESULT SYMBOLIC",
        ),
        (vec!["MAKE_DIRECTORY", "dir", ""], "RESULT"),
        (
            vec!["REAL_PATH", "path", "out", ""],
            "BASE_DIRECTORY EXPAND_TILDE",
        ),
        (
            vec!["LOCK", "path", ""],
            "DIRECTORY GUARD RELEASE RESULT_VARIABLE TIMEOUT",
        ),
        (vec!["LOCK", "path", "GUARD", ""], "FILE FUNCTION PROCESS"),
        (
            vec!["CHMOD", "path", ""],
            "DIRECTORY_PERMISSIONS FILE_PERMISSIONS PERMISSIONS",
        ),
        (
            vec!["ARCHIVE_CREATE", "OUTPUT", "a", "PATHS", "src", ""],
            "COMPRESSION ENCODING FORMAT MTIME THREADS VERBOSE WORKING_DIRECTORY",
        ),
        (
            vec!["ARCHIVE_EXTRACT", "INPUT", "a", ""],
            "DESTINATION ENCODING LIST_ONLY PATTERNS TOUCH VERBOSE",
        ),
        (
            vec!["GET_RUNTIME_DEPENDENCIES", ""],
            "BUNDLE_EXECUTABLE CONFLICTING_DEPENDENCIES_PREFIX DIRECTORIES EXECUTABLES LIBRARIES MODULES POST_EXCLUDE_FILES POST_EXCLUDE_REGEXES POST_INCLUDE_FILES POST_INCLUDE_REGEXES PRE_EXCLUDE_REGEXES PRE_INCLUDE_REGEXES RESOLVED_DEPENDENCIES_VAR UNRESOLVED_DEPENDENCIES_VAR",
        ),
    ] {
        let actual: BTreeSet<_> = keywords("file", (4, 4), &prefix).into_iter().collect();
        let expected: BTreeSet<_> = expected.split_whitespace().map(str::to_owned).collect();
        assert_eq!(actual, expected, "{prefix:?}");
    }
    let expected = "HTTPHEADER INACTIVITY_TIMEOUT LOG NETRC NETRC_FILE SHOW_PROGRESS STATUS TIMEOUT TLS_CAINFO TLS_VERIFY TLS_VERSION USERPWD";
    for mode in ["DOWNLOAD", "UPLOAD"] {
        let prefix = if mode == "DOWNLOAD" {
            vec![mode, "url", "out", ""]
        } else {
            vec![mode, "in", "url", ""]
        };
        let actual: BTreeSet<_> = keywords("file", (4, 4), &prefix).into_iter().collect();
        let mut expected: BTreeSet<_> = expected.split_whitespace().map(str::to_owned).collect();
        if mode == "DOWNLOAD" {
            expected.extend(
                ["EXPECTED_HASH", "EXPECTED_MD5", "RANGE_START", "RANGE_END"].map(str::to_owned),
            );
        }
        assert_eq!(actual, expected);
    }
    let without_file = keywords("file", (4, 4), &["DOWNLOAD", "url", ""]);
    assert!(
        !without_file
            .iter()
            .any(|word| word.starts_with("EXPECTED_"))
    );
    for prefix in [
        vec![
            "ARCHIVE_CREATE",
            "OUTPUT",
            "a",
            "PATHS",
            "in",
            "ENCODING",
            "",
        ],
        vec!["ARCHIVE_EXTRACT", "INPUT", "a", "ENCODING", ""],
    ] {
        let encodings = keywords("file", (4, 4), &prefix);
        for encoding in ["UTF-8", "OEM", "UTF-16LE", "UTF-16BE"] {
            assert!(encodings.iter().any(|word| word == encoding));
        }
    }
    assert!(accepts(
        "file",
        (4, 4),
        &["ARCHIVE_EXTRACT", "INPUT", "a", "ENCODING", "ISO-8859-1"]
    ));
}

/// Probe nested regex/JSON, list filter/transform/sort, path component/conversion and math
/// output-format branches.
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
/// Compare exact keyword sets so scalar and operation-specific slots stay distinct.
///
/// # Flow
/// ```text
///   [string/list/path/math prefixes]
///    |
///   [probe nested operation slots]------------> [expected word sets]
///                                              |
///                                             [compare exact sets]
///                                              |
///                                             < checks hold? >
///                                              /         \
///                                            [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_transform_string_list_path_modes();
/// ```
#[test]
fn keyword_transform_string_list_path_modes() {
    for (name, prefix, expected) in [
        ("string", vec!["REGEX", ""], "MATCH MATCHALL QUOTE REPLACE"),
        (
            "string",
            vec!["JSON", "out", ""],
            "EQUAL ERROR_VARIABLE GET GET_RAW LENGTH MEMBER PARTIAL_EQUAL REMOVE SET STRING_ENCODE TYPE",
        ),
        (
            "string",
            vec!["JSON", "out", "ERROR_VARIABLE", "err", ""],
            "EQUAL GET GET_RAW LENGTH MEMBER PARTIAL_EQUAL REMOVE SET STRING_ENCODE TYPE",
        ),
        ("string", vec!["RANDOM", ""], "ALPHABET LENGTH RANDOM_SEED"),
        (
            "string",
            vec!["COMPARE", ""],
            "EQUAL GREATER GREATER_EQUAL LESS LESS_EQUAL NOTEQUAL",
        ),
        ("list", vec!["FILTER", "items", ""], "EXCLUDE INCLUDE"),
        (
            "list",
            vec!["FILTER", "items", "INCLUDE", ""],
            "PREDICATE REGEX",
        ),
        (
            "list",
            vec!["TRANSFORM", "items", ""],
            "APPEND APPLY GENEX_STRIP PREPEND REPLACE STRIP TOLOWER TOUPPER",
        ),
        (
            "list",
            vec!["TRANSFORM", "items", "APPEND", "suffix", ""],
            "AT FOR OUTPUT_VARIABLE PREDICATE REGEX",
        ),
        (
            "list",
            vec!["SORT", "items", ""],
            "CASE COMPARATOR COMPARE ORDER",
        ),
        (
            "cmake_path",
            vec!["GET", "path", ""],
            "EXTENSION FILENAME PARENT_PATH RELATIVE_PART ROOT_DIRECTORY ROOT_NAME ROOT_PATH STEM",
        ),
        (
            "cmake_path",
            vec!["GET", "path", "EXTENSION", ""],
            "LAST_ONLY",
        ),
        ("cmake_path", vec!["GET", "path", "FILENAME", ""], ""),
        (
            "cmake_path",
            vec!["CONVERT", "path", ""],
            "TO_CMAKE_PATH_LIST TO_NATIVE_PATH_LIST",
        ),
        ("cmake_path", vec!["COMPARE", "a", ""], "EQUAL NOT_EQUAL"),
        (
            "cmake_path",
            vec!["ABSOLUTE_PATH", "p", ""],
            "BASE_DIRECTORY NORMALIZE OUTPUT_VARIABLE",
        ),
        (
            "math",
            vec!["EXPR", "out", "1+2", "OUTPUT_FORMAT", ""],
            "DECIMAL HEXADECIMAL",
        ),
    ] {
        let actual: BTreeSet<_> = keywords(name, (4, 4), &prefix).into_iter().collect();
        let expected: BTreeSet<_> = expected.split_whitespace().map(str::to_owned).collect();
        assert_eq!(actual, expected, "{name} {prefix:?}");
    }
}

/// Append a sentinel input-file operand to probe full-call acceptance for valid and malformed
/// file/string/list/path/math signatures.
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
/// Missing scalars, illegal option combinations and reordered operands must fail; partial required
/// slots must not invent keywords.
///
/// # Flow
/// ```text
///   [valid / invalid call tables]
///    |
///   [append file sentinel]-----------------> [production full matcher]
///                                           |
///                                          [accept valid; reject malformed]
///                                           |
///                                          < checks hold? >
///                                           /         \
///                                         [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_transform_required_scalars_and_ordering();
/// ```
#[test]
fn keyword_transform_required_scalars_and_ordering() {
    for (name, args) in [
        (
            "file",
            vec!["READ", "input", "result", "OFFSET", "2", "HEX"],
        ),
        (
            "file",
            vec!["GENERATE", "OUTPUT", "out", "INPUT", "in", "TARGET", "lib"],
        ),
        (
            "file",
            vec![
                "COPY",
                "in",
                "DESTINATION",
                "out",
                "FILES_MATCHING",
                "REGEX",
                "[.]h$",
                "EXCLUDE",
            ],
        ),
        ("file", vec!["DOWNLOAD", "url", "STATUS", "status"]),
        (
            "file",
            vec!["DOWNLOAD", "url", "out", "EXPECTED_HASH", "SHA256=abcd"],
        ),
        (
            "file",
            vec![
                "ARCHIVE_CREATE",
                "OUTPUT",
                "a",
                "PATHS",
                "in",
                "COMPRESSION",
                "Zstd",
                "COMPRESSION_LEVEL",
                "7",
            ],
        ),
        ("string", vec!["JSON", "out", "SET", "{}", "key", "0"]),
        ("string", vec!["JSON", "out", "MEMBER", "{}", "0"]),
        (
            "string",
            vec![
                "UUID",
                "out",
                "NAMESPACE",
                "uuid",
                "NAME",
                "n",
                "TYPE",
                "SHA1",
                "UPPER",
            ],
        ),
        ("list", vec!["GET", "items", "0", "-1", "out"]),
        (
            "list",
            vec![
                "TRANSFORM",
                "items",
                "APPLY",
                "f",
                "FOR",
                "0",
                "3",
                "2",
                "OUTPUT_VARIABLE",
                "out",
            ],
        ),
        (
            "list",
            vec![
                "SORT",
                "items",
                "CASE",
                "SENSITIVE",
                "COMPARATOR",
                "f",
                "ORDER",
                "DESCENDING",
            ],
        ),
        (
            "cmake_path",
            vec!["GET", "path", "STEM", "LAST_ONLY", "out"],
        ),
        (
            "cmake_path",
            vec!["CONVERT", "path", "TO_NATIVE_PATH_LIST", "out", "NORMALIZE"],
        ),
        (
            "math",
            vec!["EXPR", "out", "1 + 2", "OUTPUT_FORMAT", "DECIMAL"],
        ),
    ] {
        assert!(accepts(name, (4, 4), &args), "{name} {args:?}");
    }
    for (name, args) in [
        ("file", vec!["READ", "in"]),
        ("file", vec!["READ", "in", "out", "OFFSET"]),
        ("file", vec!["GENERATE", "OUTPUT", "out"]),
        ("file", vec!["GENERATE", "OUTPUT", "out", "CONTENT"]),
        (
            "file",
            vec![
                "GENERATE",
                "OUTPUT",
                "out",
                "CONTENT",
                "body",
                "USE_SOURCE_PERMISSIONS",
            ],
        ),
        ("file", vec!["COPY", "in", "DESTINATION"]),
        ("file", vec!["COPY", "in", "DESTINATION", "out", "EXCLUDE"]),
        (
            "file",
            vec!["UPLOAD", "in", "url", "EXPECTED_MD5", "digest"],
        ),
        ("file", vec!["ARCHIVE_EXTRACT", "INPUT"]),
        ("string", vec!["REGEX", "MATCH", ".*", "out"]),
        ("string", vec!["JSON", "out", "REMOVE", "{}"]),
        ("string", vec!["JSON", "out", "SET", "{}", "key"]),
        ("string", vec!["JSON", "out", "STRING_ENCODE"]),
        ("list", vec!["GET", "items", "out"]),
        ("list", vec!["INSERT", "items", "0"]),
        ("list", vec!["REMOVE_ITEM", "items"]),
        ("list", vec!["TRANSFORM", "items", "APPEND"]),
        ("list", vec!["TRANSFORM", "items", "STRIP", "FOR", "0"]),
        (
            "list",
            vec!["SORT", "items", "COMPARE", "NATURAL", "COMPARATOR", "f"],
        ),
        (
            "cmake_path",
            vec!["GET", "path", "FILENAME", "LAST_ONLY", "out"],
        ),
        ("cmake_path", vec!["COMPARE", "a", "b", "EQUAL", "out"]),
        (
            "cmake_path",
            vec!["CONVERT", "path", "TO_CMAKE_PATH_LIST", "NORMALIZE", "out"],
        ),
        ("math", vec!["EXPR", "out"]),
        ("math", vec!["EXPR", "out", "1+2", "OUTPUT_FORMAT"]),
    ] {
        assert!(!accepts(name, (4, 4), &args), "{name} {args:?}");
    }
    for (name, prefix) in [
        ("file", vec!["GENERATE", "OUTPUT", ""]),
        ("file", vec!["READ", ""]),
        ("string", vec!["REGEX", "MATCH", ""]),
        ("string", vec!["UUID", "out", "NAMESPACE", ""]),
        ("list", vec!["TRANSFORM", ""]),
        ("list", vec!["TRANSFORM", "items", "APPEND", ""]),
        ("math", vec!["EXPR", "out", ""]),
    ] {
        assert!(
            keywords(name, (4, 4), &prefix).is_empty(),
            "{name} {prefix:?}"
        );
    }
}

/// For every supported version, compare transform keyword availability with a table of
/// introductions.
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
/// Also check cmake_path's introduction, zero-operand file operations from 3.30 and
/// download-without-output support from 3.19 through full matching.
///
/// # Flow
/// ```text
///   [all versions + gate table]
///    |
///   [probe keyword availability]----------> [sentinel full-call cases]
///                                          |
///                                         [check operation minima + gates]
///                                          |
///                                         < checks hold? >
///                                          /         \
///                                        [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_transform_version_boundaries();
/// ```
#[test]
fn keyword_transform_version_boundaries() {
    let cases = [
        ("file", vec!["STRINGS", "in", "out", ""], "ENCODING", (3, 1)),
        (
            "file",
            vec!["STRINGS", "in", "out", "ENCODING", ""],
            "UTF-16LE",
            (3, 2),
        ),
        ("file", vec![""], "LOCK", (3, 2)),
        ("file", vec!["GLOB", "out", ""], "LIST_DIRECTORIES", (3, 3)),
        (
            "file",
            vec!["DOWNLOAD", "url", "out", ""],
            "HTTPHEADER",
            (3, 7),
        ),
        ("file", vec![""], "SHA3_256", (3, 8)),
        ("file", vec!["DOWNLOAD", "url", "out", ""], "NETRC", (3, 11)),
        (
            "file",
            vec!["GLOB_RECURSE", "out", ""],
            "CONFIGURE_DEPENDS",
            (3, 12),
        ),
        ("file", vec![""], "TOUCH_NOCREATE", (3, 12)),
        ("file", vec![""], "SIZE", (3, 14)),
        ("file", vec![""], "READ_SYMLINK", (3, 14)),
        ("file", vec![""], "CREATE_LINK", (3, 14)),
        (
            "file",
            vec!["COPY", "in", "DESTINATION", "out", ""],
            "FOLLOW_SYMLINK_CHAIN",
            (3, 15),
        ),
        ("file", vec![""], "GET_RUNTIME_DEPENDENCIES", (3, 16)),
        ("file", vec![""], "CONFIGURE", (3, 18)),
        ("file", vec![""], "ARCHIVE_CREATE", (3, 18)),
        ("file", vec![""], "ARCHIVE_EXTRACT", (3, 18)),
        (
            "file",
            vec!["UPLOAD", "in", "url", ""],
            "TLS_VERIFY",
            (3, 18),
        ),
        ("file", vec![""], "CHMOD_RECURSE", (3, 19)),
        ("file", vec![""], "REAL_PATH", (3, 19)),
        (
            "file",
            vec!["GENERATE", "OUTPUT", "out", "CONTENT", "x", ""],
            "TARGET",
            (3, 19),
        ),
        (
            "file",
            vec![
                "ARCHIVE_CREATE",
                "OUTPUT",
                "a",
                "PATHS",
                "in",
                "COMPRESSION",
                "Zstd",
                "",
            ],
            "COMPRESSION_LEVEL",
            (3, 19),
        ),
        (
            "file",
            vec!["GENERATE", "OUTPUT", "out", "INPUT", "in", ""],
            "NEWLINE_STYLE",
            (3, 20),
        ),
        (
            "file",
            vec!["GENERATE", "OUTPUT", "out", "INPUT", "in", ""],
            "USE_SOURCE_PERMISSIONS",
            (3, 20),
        ),
        ("file", vec![""], "COPY_FILE", (3, 21)),
        ("file", vec!["RENAME", "old", "new", ""], "RESULT", (3, 21)),
        (
            "file",
            vec!["RENAME", "old", "new", ""],
            "NO_REPLACE",
            (3, 21),
        ),
        (
            "file",
            vec!["REAL_PATH", "p", "out", ""],
            "EXPAND_TILDE",
            (3, 21),
        ),
        (
            "file",
            vec!["GET_RUNTIME_DEPENDENCIES", ""],
            "POST_INCLUDE_FILES",
            (3, 21),
        ),
        (
            "file",
            vec!["DOWNLOAD", "url", "out", ""],
            "RANGE_START",
            (3, 24),
        ),
        (
            "file",
            vec!["ARCHIVE_EXTRACT", "INPUT", "a", ""],
            "TOUCH",
            (3, 24),
        ),
        (
            "file",
            vec!["COPY_FILE", "in", "out", ""],
            "INPUT_MAY_BE_RECENT",
            (3, 26),
        ),
        (
            "file",
            vec!["UPLOAD", "in", "url", ""],
            "TLS_VERSION",
            (3, 30),
        ),
        ("file", vec!["MAKE_DIRECTORY", "dir", ""], "RESULT", (3, 31)),
        (
            "file",
            vec!["ARCHIVE_CREATE", "OUTPUT", "a", "PATHS", "in", ""],
            "WORKING_DIRECTORY",
            (3, 31),
        ),
        (
            "file",
            vec!["ARCHIVE_CREATE", "OUTPUT", "a", "PATHS", "in", ""],
            "THREADS",
            (4, 3),
        ),
        (
            "file",
            vec![
                "ARCHIVE_CREATE",
                "OUTPUT",
                "a",
                "PATHS",
                "in",
                "COMPRESSION",
                "",
            ],
            "LZMA",
            (4, 3),
        ),
        (
            "file",
            vec![
                "ARCHIVE_CREATE",
                "OUTPUT",
                "a",
                "PATHS",
                "in",
                "COMPRESSION",
                "",
            ],
            "Deflate",
            (4, 3),
        ),
        (
            "file",
            vec![
                "ARCHIVE_CREATE",
                "OUTPUT",
                "a",
                "PATHS",
                "in",
                "COMPRESSION",
                "",
            ],
            "LZMA2",
            (4, 3),
        ),
        (
            "file",
            vec![
                "ARCHIVE_CREATE",
                "OUTPUT",
                "a",
                "PATHS",
                "in",
                "COMPRESSION",
                "",
            ],
            "PPMd",
            (4, 3),
        ),
        (
            "file",
            vec!["ARCHIVE_CREATE", "OUTPUT", "a", "PATHS", "in", ""],
            "ENCODING",
            (4, 4),
        ),
        (
            "file",
            vec!["ARCHIVE_EXTRACT", "INPUT", "a", ""],
            "ENCODING",
            (4, 4),
        ),
        (
            "file",
            vec!["READ_SYMLINK", "in", "out", ""],
            "RESULT",
            (4, 4),
        ),
        ("string", vec![""], "GENEX_STRIP", (3, 1)),
        ("string", vec![""], "UUID", (3, 1)),
        ("string", vec![""], "APPEND", (3, 4)),
        ("string", vec!["COMPARE", ""], "LESS_EQUAL", (3, 7)),
        ("string", vec![""], "SHA3_512", (3, 8)),
        ("string", vec![""], "PREPEND", (3, 10)),
        ("string", vec![""], "JOIN", (3, 12)),
        ("string", vec![""], "REPEAT", (3, 15)),
        ("string", vec![""], "HEX", (3, 18)),
        ("string", vec![""], "JSON", (3, 19)),
        ("string", vec!["REGEX", ""], "QUOTE", (4, 2)),
        ("string", vec!["JSON", "out", ""], "GET_RAW", (4, 3)),
        ("string", vec!["JSON", "out", ""], "STRING_ENCODE", (4, 3)),
        ("string", vec!["JSON", "out", ""], "PARTIAL_EQUAL", (4, 4)),
        ("list", vec![""], "FILTER", (3, 6)),
        ("list", vec![""], "JOIN", (3, 12)),
        ("list", vec![""], "SUBLIST", (3, 12)),
        ("list", vec![""], "TRANSFORM", (3, 12)),
        ("list", vec!["SORT", "items", ""], "COMPARE", (3, 13)),
        ("list", vec![""], "PREPEND", (3, 15)),
        ("list", vec![""], "POP_FRONT", (3, 15)),
        ("list", vec![""], "POP_BACK", (3, 15)),
        (
            "list",
            vec!["SORT", "items", "COMPARE", ""],
            "NATURAL",
            (3, 18),
        ),
        (
            "list",
            vec!["FILTER", "items", "INCLUDE", ""],
            "PREDICATE",
            (4, 4),
        ),
        ("list", vec!["SORT", "items", ""], "COMPARATOR", (4, 4)),
        ("list", vec!["TRANSFORM", "items", ""], "APPLY", (4, 4)),
        (
            "list",
            vec!["TRANSFORM", "items", "STRIP", ""],
            "PREDICATE",
            (4, 4),
        ),
        (
            "math",
            vec!["EXPR", "out", "1+2", ""],
            "OUTPUT_FORMAT",
            (3, 13),
        ),
    ];
    for version in (0..=31)
        .map(|minor| (3, minor))
        .chain((0..=4).map(|minor| (4, minor)))
    {
        for (name, prefix, keyword, introduced) in &cases {
            assert_eq!(
                keywords(name, version, prefix)
                    .iter()
                    .any(|word| word == keyword),
                version >= *introduced,
                "{name} {prefix:?} {keyword} at {version:?}, introduced {introduced:?}",
            );
        }
        assert_eq!(
            schemas(version)
                .iter()
                .any(|(name, _)| *name == "cmake_path"),
            version >= (3, 20)
        );
        for mode in ["MAKE_DIRECTORY", "TOUCH", "TOUCH_NOCREATE"] {
            if mode == "MAKE_DIRECTORY" || version >= (3, 12) {
                assert_eq!(
                    accepts("file", version, &[mode]),
                    version >= (3, 30),
                    "{mode} {version:?}"
                );
            }
        }
        assert_eq!(
            accepts("file", version, &["DOWNLOAD", "url", "STATUS", "out"]),
            version >= (3, 19)
        );
    }
}

/// Construct bracket-form synthetic calls and track variable writes to compare exact
/// output-argument indices.
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
/// Separately verify target/name roles, file GENERATE target binding, generic dependency hints and
/// empty completion inside opaque payloads.
///
/// # Flow
/// ```text
///   [synthetic bracket-form calls]
///    |
///   [track output roles]--------------------> [target/path/opaque probes]
///                                            |
///                                           [compare indices + excluded hints]
///                                            |
///                                           < checks hold? >
///                                            /         \
///                                          [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_transform_roles_and_opaque_operands();
/// ```
#[test]
fn keyword_transform_roles_and_opaque_operands() {
    for (name, args, expected) in [
        ("file", vec!["READ", "in", "out", "LIMIT", "2"], vec![2]),
        (
            "file",
            vec!["STRINGS", "in", "out", "ENCODING", "UTF-8"],
            vec![2],
        ),
        (
            "file",
            vec!["DOWNLOAD", "url", "out", "STATUS", "status", "LOG", "log"],
            vec![4, 6],
        ),
        (
            "file",
            vec!["READ_SYMLINK", "in", "out", "RESULT", "status"],
            vec![2, 4],
        ),
        (
            "file",
            vec![
                "GET_RUNTIME_DEPENDENCIES",
                "RESOLVED_DEPENDENCIES_VAR",
                "deps",
                "UNRESOLVED_DEPENDENCIES_VAR",
                "missing",
            ],
            vec![2, 4],
        ),
        (
            "string",
            vec!["REGEX", "REPLACE", ".*", "replacement", "out", "input"],
            vec![4],
        ),
        (
            "string",
            vec!["JSON", "out", "ERROR_VARIABLE", "error", "GET_RAW", "{}"],
            vec![1, 3],
        ),
        (
            "string",
            vec!["JSON", "out", "PARTIAL_EQUAL", "{}", "{}"],
            vec![1],
        ),
        (
            "string",
            vec!["RANDOM", "LENGTH", "8", "ALPHABET", "abc", "out"],
            vec![5],
        ),
        ("list", vec!["GET", "items", "0", "2", "out"], vec![4]),
        (
            "list",
            vec!["POP_FRONT", "items", "first", "second"],
            vec![2, 3],
        ),
        (
            "list",
            vec!["TRANSFORM", "items", "APPLY", "f", "OUTPUT_VARIABLE", "out"],
            vec![5],
        ),
        (
            "cmake_path",
            vec!["GET", "p", "EXTENSION", "LAST_ONLY", "out"],
            vec![4],
        ),
        ("cmake_path", vec!["SET", "out", "NORMALIZE", "p"], vec![1]),
        (
            "cmake_path",
            vec!["APPEND", "p", "a", "b", "OUTPUT_VARIABLE", "out"],
            vec![5],
        ),
        (
            "math",
            vec!["EXPR", "out", "1 + 2", "OUTPUT_FORMAT", "DECIMAL"],
            vec![1],
        ),
    ] {
        let mut buffer = SemanticedBuffer::default();
        buffer.commands.push(command(schema(name, (4, 4)), &args));
        let analyzer = Semanticer::new(
            NParsedBuffer {
                nodes: vec![],
                comments: vec![],
            },
            Default::default(),
        );
        analyzer.track_variable_changes(&mut buffer);
        let actual: Vec<_> = buffer.commands[0]
            .args
            .iter()
            .enumerate()
            .filter_map(|(index, arg)| {
                arg.kind
                    .iter()
                    .any(|kind| matches!(kind, ArgKind::Out))
                    .then_some(index)
            })
            .collect();
        assert_eq!(actual, expected, "{name} {args:?}");
    }
    for (name, prefix, role) in [
        (
            "file",
            vec!["GENERATE", "OUTPUT", "out", "INPUT", "in", "TARGET", ""],
            NameRole::Target,
        ),
        ("list", vec!["TRANSFORM", ""], NameRole::Variable),
        ("cmake_path", vec!["GET", ""], NameRole::Variable),
    ] {
        assert!(completion_roles(&schema(name, (4, 4)), &prefix).contains(&role));
    }
    let target = command(
        schema("file", (4, 4)),
        &[
            "GENERATE", "OUTPUT", "out", "CONTENT", "body", "TARGET", "lib",
        ],
    );
    assert_eq!(
        target_arguments(
            &target,
            &[
                "GENERATE", "OUTPUT", "out", "CONTENT", "body", "TARGET", "lib"
            ]
        ),
        vec![(6, NameRole::Target)]
    );
    let hints = command(
        schema("file", (4, 4)),
        &[
            "GET_RUNTIME_DEPENDENCIES",
            "EXECUTABLES",
            "program",
            "DIRECTORIES",
            "missing-dir",
            "POST_INCLUDE_FILES",
            "missing-filter",
        ],
    );
    let paths = path_arguments(&hints);
    assert_eq!(paths.len(), 1);
    assert_eq!(paths[0].argument, 2);
    assert_eq!(paths[0].kind, PathKind::File);
    for (name, prefix) in [
        ("file", vec!["WRITE", "out", "body", ""]),
        ("file", vec!["CONFIGURE", "OUTPUT", "out", "CONTENT", ""]),
        ("string", vec!["JSON", "out", "GET", "{}", ""]),
        ("string", vec!["REGEX", "MATCH", ""]),
        ("list", vec!["TRANSFORM", "items", "REPLACE", ""]),
        ("math", vec!["EXPR", "out", ""]),
    ] {
        assert!(
            keywords(name, (4, 4), &prefix).is_empty(),
            "{name} {prefix:?}"
        );
    }
}

/// Append active-slot witnesses and return a representative continuation for transform grammars.
/// Consumes a prefix locally, appends to the supplied vector and performs no I/O.
///
/// # Arguments
/// - `kinds` `[in]`: Structured argument sequence; alternatives must contain at least one branch.
/// - `prefix` `[in]`: Owned preceding arguments extended locally for the returned continuation.
/// - `result` `[in,out]`: Witness vector; append (prefix ending in an empty active slot, keyword)
///   pairs without clearing prior entries.
///
/// # Returns
/// The representative prefix after the sequence: uses the first choice continuation, omits optional
/// groups and adopts one repeated continuation when min is positive. Other branch witnesses still
/// remain in result.
///
/// # Panics
/// On empty choices or Unmodeled, Action or AllOf nodes; these are outside the expected transform
/// grammar.
///
/// # Algorithm
/// Record literals at an empty active slot, traverse every choice for witnesses but keep its first
/// continuation, visit optionals without committing them and commit one repeated group only when
/// required.
///
/// # Flow
/// ```text
///              [schema node]
///                     |
///          +----------+----------+
///          |                     |
///      [literal]              [group]
///          |               /     |      \
/// [append active probe] choice optional repeat
/// [extend prefix]         |      |       |
///          |         [first]   [omit] [min > 0?]
///          |              |      |       |
///          +--------------+------+-------+
///                         v
///               [return continuation]
/// ```
///
/// # Examples
/// Local test-module context:
/// ```text
/// let mut probes = vec![];
/// let after = keyword_witnesses(&[K::Keyword("READ".into())], vec![], &mut probes);
/// assert_eq!(after, ["READ"]);
/// assert_eq!(probes[0], (vec!["".into()], "READ".into()));
/// ```
fn keyword_witnesses(
    kinds: &[K],
    mut prefix: Vec<String>,
    result: &mut Vec<(Vec<String>, String)>,
) -> Vec<String> {
    for kind in kinds {
        match kind {
            K::Keyword(word) => {
                let mut active = prefix.clone();
                active.push(String::new());
                result.push((active, word.clone()));
                prefix.push(word.clone());
            }
            K::Choice(branches) => {
                let mut first = None;
                for branch in branches {
                    let continuation = keyword_witnesses(branch, prefix.clone(), result);
                    first.get_or_insert(continuation);
                }
                prefix = first.unwrap();
            }
            K::Optional(args) => {
                keyword_witnesses(args, prefix.clone(), result);
            }
            K::Repeat { args, min } => {
                let continuation = keyword_witnesses(args, prefix.clone(), result);
                if *min > 0 {
                    prefix = continuation;
                }
            }
            K::Unmodeled | K::Action | K::AllOf(_) => {
                panic!("unexpected fallback or intersection in transform grammar")
            }
            _ => prefix.push("operand".to_owned()),
        }
    }
    prefix
}

/// Recursively create active-slot witnesses for every transform literal across all supported
/// versions, then require the real prefix matcher to offer each word at its generated position.
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
/// Recursively create active-slot witnesses for every transform literal across all supported
/// versions, then require the real prefix matcher to offer each word at its generated position.
///
/// # Flow
/// ```text
///   [all supported transform schemas]
///    |
///   [generate active-slot witnesses]-----------> [production prefix matcher]
///                                               |
///                                              [require every literal reachable]
///                                               |
///                                              < checks hold? >
///                                               /         \
///                                             [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_transform_every_literal_is_reachable_in_all_supported_versions();
/// ```
#[test]
fn keyword_transform_every_literal_is_reachable_in_all_supported_versions() {
    for version in (0..=31)
        .map(|minor| (3, minor))
        .chain((0..=4).map(|minor| (4, minor)))
    {
        for (name, args) in schemas(version) {
            let mut witnesses = vec![];
            keyword_witnesses(&args, vec![], &mut witnesses);
            let schema = schema(name, version);
            for (prefix, word) in witnesses {
                let prefix: Vec<_> = prefix.iter().map(String::as_str).collect();
                assert!(
                    completion_keywords(&schema, &prefix, || false).contains(&word.as_str()),
                    "{version:?} {name} {prefix:?}: {word}"
                );
            }
        }
    }
}

/// Probe COPY after a completed destination and after a PATTERN. Compare exact allowed option sets
/// and require DESTINATION to disappear once consumed.
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
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_transform_copy_destination_not_repeated();
/// ```
#[test]
fn keyword_transform_copy_destination_not_repeated() {
    for (prefix, expected) in [
        (
            vec!["COPY", "src", "DESTINATION", "dest", ""],
            "DIRECTORY_PERMISSIONS FILES_MATCHING FILE_PERMISSIONS FOLLOW_SYMLINK_CHAIN NO_SOURCE_PERMISSIONS PATTERN REGEX USE_SOURCE_PERMISSIONS",
        ),
        (
            vec!["COPY", "src", "DESTINATION", "dest", "PATTERN", "*.h", ""],
            "EXCLUDE PATTERN PERMISSIONS REGEX",
        ),
    ] {
        let actual: BTreeSet<_> = keywords("file", (4, 4), &prefix).into_iter().collect();
        assert!(!actual.contains("DESTINATION"), "{prefix:?}");
        let expected: BTreeSet<_> = expected.split_whitespace().map(str::to_owned).collect();
        assert_eq!(actual, expected, "{prefix:?}");
    }
}

/// Probe ARCHIVE_CREATE before and after `COMPRESSION Zstd`. COMPRESSION_LEVEL must appear only
/// after a compression selection.
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
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_transform_compression_level_is_scoped_to_compression();
/// ```
#[test]
fn keyword_transform_compression_level_is_scoped_to_compression() {
    assert!(
        !keywords(
            "file",
            (4, 4),
            &["ARCHIVE_CREATE", "OUTPUT", "a", "PATHS", "in", ""]
        )
        .contains(&"COMPRESSION_LEVEL".to_owned())
    );
    assert!(
        keywords(
            "file",
            (4, 4),
            &[
                "ARCHIVE_CREATE",
                "OUTPUT",
                "a",
                "PATHS",
                "in",
                "COMPRESSION",
                "Zstd",
                ""
            ]
        )
        .contains(&"COMPRESSION_LEVEL".to_owned())
    );
}

/// Construct file/path calls and compare every returned path index/kind, including symbolic-link
/// source exclusion, archive paths and generic cmake_path operands that must not require existing
/// files.
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
/// Construct file/path calls and compare every returned path index/kind, including symbolic-link
/// source exclusion, archive paths and generic cmake_path operands that must not require existing
/// files.
///
/// # Flow
/// ```text
///   [synthetic file/path call table]
///    |
///   [extract typed path arguments]------------> [expected ordered index/kind pairs]
///                                              |
///                                             [compare; generic paths excluded]
///                                              |
///                                             < checks hold? >
///                                              /         \
///                                            [pass]     [panic]
/// ```
///
/// # Examples
/// Local test-module call:
/// ```text
/// keyword_transform_input_output_path_roles();
/// ```
#[test]
fn keyword_transform_input_output_path_roles() {
    for (name, args, expected) in [
        (
            "file",
            vec!["READ", "input", "out"],
            vec![(1, PathKind::File)],
        ),
        (
            "file",
            vec!["COPY", "src", "DESTINATION", "dest"],
            vec![(1, PathKind::Input), (3, PathKind::Output)],
        ),
        (
            "file",
            vec!["RENAME", "old", "new"],
            vec![(1, PathKind::Input), (2, PathKind::Output)],
        ),
        (
            "file",
            vec!["CREATE_LINK", "nonexistent", "link", "SYMBOLIC"],
            vec![(2, PathKind::Output)],
        ),
        (
            "file",
            vec!["DOWNLOAD", "url", "downloaded", "LOG", "log"],
            vec![(2, PathKind::Output)],
        ),
        (
            "file",
            vec!["GENERATE", "OUTPUT", "generated", "INPUT", "input"],
            vec![(4, PathKind::File), (2, PathKind::Output)],
        ),
        (
            "file",
            vec!["CHMOD", "input", "PERMISSIONS", "OWNER_READ"],
            vec![(1, PathKind::Input)],
        ),
        (
            "file",
            vec![
                "ARCHIVE_CREATE",
                "OUTPUT",
                "archive",
                "PATHS",
                "input",
                "WORKING_DIRECTORY",
                "work",
            ],
            vec![(4, PathKind::Input), (2, PathKind::Output)],
        ),
        (
            "file",
            vec!["ARCHIVE_EXTRACT", "INPUT", "archive", "DESTINATION", "dest"],
            vec![(2, PathKind::File), (4, PathKind::Output)],
        ),
        (
            "cmake_path",
            vec!["SET", "path", "NORMALIZE", "nonexistent"],
            vec![],
        ),
        (
            "cmake_path",
            vec![
                "ABSOLUTE_PATH",
                "path",
                "BASE_DIRECTORY",
                "nonexistent",
                "OUTPUT_VARIABLE",
                "out",
            ],
            vec![],
        ),
    ] {
        let actual: Vec<_> = path_arguments(&command(schema(name, (4, 4)), &args))
            .into_iter()
            .map(|path| (path.argument, path.kind))
            .collect();
        assert_eq!(actual, expected, "{name} {args:?}");
    }
}
