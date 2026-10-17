//! Process, CTest and host-query syntax as immutable argument metadata.
//! [`ArgSchemaKind`] trees describe programs, files, outputs and literal modes;
//! they never spawn processes, inspect the host, submit test results or configure
//! projects. For example, `execute_process(COMMAND tool OUTPUT_VARIABLE result)`
//! gives `result` an output-variable role; calling the builder does not run tool.
//! `schemas` supplies the final cache with version-filtered overrides, including
//! language dispatch, package configuration and newer diagnostic/test commands.

// AI-generated implementation.
use super::{
    ArgSchemaKind, Boolean, Command, Expression, InputDirectory, InputFile, Integer, Number, Out,
    OutputPath, PackageName, Path, Target, TargetExe, Text, Url, Value, Version, choice, k, many,
    one_of, optional, options, pairs, some,
};

/// Collect process, CTest, host and newer command overrides.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
///
/// # Returns
/// Owned (command name, ordered roots) pairs, with newer commands included only at their known
/// introduction versions.
///
/// # Algorithm
/// Collect execution/CTest/host trees, then append cmake_language, file API, pkg-config,
/// instrumentation and 4.4 discovery/diagnostic forms at their thresholds. Native registration
/// filtering remains the cache caller's job.
///
/// # Flow
/// ```text
///                             [schemas inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |     Process / CTest / host     |    |   Versioned language / API /   |
/// |                                |    |          diagnostics           |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let overrides = schemas((4, 4));
/// assert!(overrides.iter().any(|(name, _)| *name == "discover_tests"));
/// ```
pub(super) fn schemas(version: (u32, u32)) -> Vec<(&'static str, Vec<ArgSchemaKind>)> {
    let mut schemas = vec![
        ("execute_process", execute_process(version)),
        (
            "exec_program",
            vec![
                Command,
                optional(vec![InputDirectory]),
                options(vec![
                    vec![k("ARGS"), some(Value)],
                    vec![k("OUTPUT_VARIABLE"), Out],
                    vec![k("RETURN_VALUE"), Out],
                ]),
            ],
        ),
        ("ctest_build", ctest_build(version)),
        ("ctest_configure", ctest_configure(version)),
        ("ctest_coverage", ctest_coverage(version)),
        ("ctest_empty_binary_directory", vec![InputDirectory]),
        ("ctest_memcheck", ctest_test(version, true)),
        ("ctest_read_custom_files", vec![some(InputDirectory)]),
        (
            "ctest_run_script",
            vec![
                optional(vec![k("NEW_PROCESS")]),
                if version >= (4, 0) {
                    some(InputFile)
                } else {
                    many(InputFile)
                },
                optional(vec![k("RETURN_VALUE"), Out]),
            ],
        ),
        (
            "ctest_sleep",
            vec![choice(vec![vec![Number], vec![Number, Number, Number]])],
        ),
        ("ctest_start", ctest_start(version)),
        ("ctest_submit", ctest_submit(version)),
        ("ctest_test", ctest_test(version, false)),
        ("ctest_update", ctest_update(version)),
        (
            "ctest_upload",
            vec![
                k("FILES"),
                some(InputFile),
                options(ctest_status(version, (3, 7))),
            ],
        ),
        (
            "cmake_host_system_information",
            host_system_information(version),
        ),
    ];
    if version >= (3, 18) {
        schemas.push(("cmake_language", cmake_language(version)));
    }
    if version >= (3, 27) {
        schemas.push((
            "cmake_file_api",
            vec![
                k("QUERY"),
                k("API_VERSION"),
                Integer,
                options(
                    ["CODEMODEL", "CACHE", "CMAKEFILES", "TOOLCHAINS"]
                        .map(|word| vec![k(word), some(Version)])
                        .to_vec(),
                ),
            ],
        ));
    }
    if version >= (3, 31) {
        schemas.push(("cmake_pkg_config", pkg_config(version)));
    }
    if version >= (4, 3) {
        schemas.push(("cmake_instrumentation", instrumentation(version)));
    }
    if version >= (4, 4) {
        schemas.extend([
            (
                "discover_tests",
                vec![
                    k("COMMAND"),
                    TargetExe,
                    many(Value),
                    optional(vec![k("COMMAND_EXPAND_LISTS")]),
                    optional(vec![k("CONFIGURATIONS"), some(Text)]),
                    k("DISCOVERY_ARGS"),
                    some(Value),
                    k("DISCOVERY_MATCH"),
                    Expression,
                    optional(vec![k("DISCOVERY_PROPERTIES"), pairs()]),
                    k("TEST_NAME"),
                    Text,
                    k("TEST_ARGS"),
                    some(Value),
                    optional(vec![k("TEST_PROPERTIES"), pairs()]),
                ],
            ),
            (
                "cmake_diagnostic",
                vec![choice(vec![
                    vec![
                        k("SET"),
                        Text,
                        diagnostic_action(),
                        optional(vec![k("RECURSE")]),
                    ],
                    vec![
                        one_of(&["PROMOTE", "DEMOTE"]),
                        Text,
                        diagnostic_action(),
                        optional(vec![k("NO_RECURSE")]),
                    ],
                    vec![k("GET"), Text, Out],
                    vec![one_of(&["PUSH", "POP"])],
                ])],
            ),
        ]);
    }
    schemas
}

/// Describe required first COMMAND and repeated process/control options.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
///
/// # Returns
/// COMMAND/program/values followed by repeated option groups, including additional COMMAND stages
/// and explicit output variables/files.
///
/// # Algorithm
/// Keep one required command before options. Add encoding, result lists, echo/fatal controls and
/// environment groups by version. Program arguments stay arbitrary values, not inferred
/// filesystem inputs.
///
/// # Flow
/// ```text
///                         [execute_process inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |   Required COMMAND + values    |    |Versioned process/control groups|
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = execute_process((4, 4)); // Includes ENVIRONMENT_MODIFICATION values.
/// ```
fn execute_process(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut opts = vec![
        vec![k("COMMAND"), Command, many(Value)],
        vec![k("WORKING_DIRECTORY"), InputDirectory],
        vec![k("TIMEOUT"), Number],
        vec![k("RESULT_VARIABLE"), Out],
        vec![k("OUTPUT_VARIABLE"), Out],
        vec![k("ERROR_VARIABLE"), Out],
        vec![k("INPUT_FILE"), InputFile],
        vec![k("OUTPUT_FILE"), OutputPath],
        vec![k("ERROR_FILE"), OutputPath],
        vec![one_of(&[
            "OUTPUT_QUIET",
            "ERROR_QUIET",
            "OUTPUT_STRIP_TRAILING_WHITESPACE",
            "ERROR_STRIP_TRAILING_WHITESPACE",
        ])],
    ];
    if version >= (3, 8) {
        let mut encodings = vec!["NONE", "AUTO", "ANSI", "OEM", "UTF8"];
        if version >= (3, 11) {
            encodings.push("UTF-8");
        }
        opts.push(vec![k("ENCODING"), one_of(&encodings)]);
    }
    if version >= (3, 10) {
        opts.push(vec![k("RESULTS_VARIABLE"), Out]);
    }
    if version >= (3, 15) {
        opts.push(vec![
            k("COMMAND_ECHO"),
            one_of(&["STDERR", "STDOUT", "NONE"]),
        ]);
    }
    if version >= (3, 18) {
        opts.push(vec![one_of(&[
            "ECHO_OUTPUT_VARIABLE",
            "ECHO_ERROR_VARIABLE",
        ])]);
    }
    if version >= (3, 19) {
        opts.push(vec![
            k("COMMAND_ERROR_IS_FATAL"),
            if version >= (4, 0) {
                one_of(&["ANY", "LAST", "NONE"])
            } else {
                one_of(&["ANY", "LAST"])
            },
        ]);
    }
    if version >= (4, 4) {
        opts.push(vec![
            one_of(&["ENVIRONMENT", "ENVIRONMENT_MODIFICATION"]),
            some(Value),
        ]);
    }
    vec![k("COMMAND"), Command, many(Value), options(opts)]
}

/// Collect quiet and captured-CMake-error options at caller-selected thresholds.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
/// - `capture_since` `[in]`: first major/minor where this CTest command accepts CAPTURE_CMAKE_ERROR.
///
/// # Returns
/// Zero, one or two option alternatives: QUIET from 3.3 and CAPTURE_CMAKE_ERROR Out from
/// capture_since.
///
/// # Examples
/// ```text
/// let opts = ctest_status((3, 7), (3, 7)); // Contains QUIET and captured error output.
/// ```
fn ctest_status(version: (u32, u32), capture_since: (u32, u32)) -> Vec<Vec<ArgSchemaKind>> {
    let mut opts = Vec::new();
    if version >= (3, 3) {
        opts.push(vec![k("QUIET")]);
    }
    if version >= capture_since {
        opts.push(vec![k("CAPTURE_CMAKE_ERROR"), Out]);
    }
    opts
}

/// Append CTest preset option groups for 4.4 and newer.
/// Mutates only the supplied option vector; no project commands or I/O.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
/// - `opts` `[in,out]`: existing option alternatives; new PRESET and PRESETS_FILE groups are
///   appended, never overwritten.
///
/// # Returns
/// Unit; opts is unchanged before 4.4 and extended by two groups from 4.4.
///
/// # Examples
/// ```text
/// let mut opts = Vec::new();
/// ctest_presets((4, 4), &mut opts);
/// assert_eq!(opts.len(), 2);
/// ```
fn ctest_presets(version: (u32, u32), opts: &mut Vec<Vec<ArgSchemaKind>>) {
    if version >= (4, 4) {
        opts.extend([vec![k("PRESET"), Text], vec![k("PRESETS_FILE"), InputFile]]);
    }
}

/// Describe CTest build options and build-status outputs without building.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
///
/// # Returns
/// One repeated-options root with input build directory, target and counter/result output roles.
///
/// # Algorithm
/// Collect base build/control/result groups, extend status controls, add parallelism from 3.21
/// and append presets through the shared helper.
///
/// # Flow
/// ```text
///                           [ctest_build inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |Build settings + result counters|    | Status / parallelism / presets |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = ctest_build((4, 4)); // Includes preset options and PARALLEL_LEVEL.
/// ```
fn ctest_build(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut opts = vec![
        vec![k("BUILD"), InputDirectory],
        vec![k("APPEND")],
        vec![k("CONFIGURATION"), Text],
        vec![k("FLAGS"), Text],
        vec![k("PROJECT_NAME"), Text],
        vec![k("TARGET"), Target],
        vec![k("NUMBER_ERRORS"), Out],
        vec![k("NUMBER_WARNINGS"), Out],
        vec![k("RETURN_VALUE"), Out],
    ];
    opts.extend(ctest_status(version, (3, 7)));
    if version >= (3, 21) {
        opts.push(vec![k("PARALLEL_LEVEL"), Integer]);
    }
    ctest_presets(version, &mut opts);
    vec![options(opts)]
}

/// Describe CTest configure options without configuring a project.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
///
/// # Returns
/// One repeated-options root with build/source input directories and return/error output roles.
///
/// # Examples
/// ```text
/// let args = ctest_configure((3, 7)); // RETURN_VALUE and CAPTURE_CMAKE_ERROR are outputs.
/// ```
fn ctest_configure(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut opts = vec![
        vec![k("BUILD"), InputDirectory],
        vec![k("SOURCE"), InputDirectory],
        vec![k("APPEND")],
        vec![k("OPTIONS"), Text],
        vec![k("RETURN_VALUE"), Out],
    ];
    opts.extend(ctest_status(version, (3, 7)));
    ctest_presets(version, &mut opts);
    vec![options(opts)]
}

/// Describe CTest coverage options without collecting coverage.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
///
/// # Returns
/// One repeated-options root for build directory, labels, append and status/result controls.
///
/// # Examples
/// ```text
/// let args = ctest_coverage((3, 7)); // LABELS consumes one or more text values.
/// ```
fn ctest_coverage(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut opts = vec![
        vec![k("BUILD"), InputDirectory],
        vec![k("APPEND")],
        vec![k("LABELS"), some(Text)],
        vec![k("RETURN_VALUE"), Out],
    ];
    opts.extend(ctest_status(version, (3, 7)));
    vec![options(opts)]
}

/// Describe shared CTest test/memory-check options without running tests.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
/// - `memcheck` `[in]`: include DEFECT_COUNT output for memory checking from 3.8.
///
/// # Returns
/// One repeated-options root covering selection, scheduling, resources, outputs and
/// version-supported presets.
///
/// # Algorithm
/// Collect base test filters and results, then add versioned fixtures/resources/repeat/JUnit/file
/// filters. DEFECT_COUNT is memory-check-only; REPEAT remains one mode:count text argument.
/// Append presets last.
///
/// # Flow
/// ```text
///                            [ctest_test inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |  Test selection / scheduling   |    | Versioned resources / results  |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = ctest_test((3, 29), true); // PARALLEL_LEVEL may omit its integer.
/// ```
fn ctest_test(version: (u32, u32), memcheck: bool) -> Vec<ArgSchemaKind> {
    let mut opts = vec![
        vec![k("BUILD"), InputDirectory],
        vec![k("APPEND")],
        vec![one_of(&["START", "END", "STRIDE"]), Integer],
        vec![
            one_of(&["EXCLUDE", "INCLUDE", "EXCLUDE_LABEL", "INCLUDE_LABEL"]),
            Expression,
        ],
        vec![
            k("PARALLEL_LEVEL"),
            if version >= (3, 29) {
                optional(vec![Integer])
            } else {
                Integer
            },
        ],
        vec![k("SCHEDULE_RANDOM"), one_of(&["ON", "OFF"])],
        vec![k("STOP_TIME"), Text],
        vec![k("RETURN_VALUE"), Out],
    ];
    opts.extend(ctest_status(version, (3, 7)));
    if version >= (3, 4) {
        opts.push(vec![k("TEST_LOAD"), Number]);
    }
    if version >= (3, 7) {
        opts.push(vec![
            one_of(&[
                "EXCLUDE_FIXTURE",
                "EXCLUDE_FIXTURE_SETUP",
                "EXCLUDE_FIXTURE_CLEANUP",
            ]),
            Expression,
        ]);
    }
    if memcheck && version >= (3, 8) {
        opts.push(vec![k("DEFECT_COUNT"), Out]);
    }
    if version >= (3, 16) {
        opts.push(vec![k("RESOURCE_SPEC_FILE"), InputFile]);
    }
    // REPEAT is a single mode:count operand, not two CMake arguments.
    if version >= (3, 17) {
        opts.push(vec![k("REPEAT"), Text]);
    }
    if version >= (3, 18) {
        opts.push(vec![k("STOP_ON_FAILURE")]);
    }
    if version >= (3, 21) {
        opts.push(vec![k("OUTPUT_JUNIT"), OutputPath]);
    }
    if version >= (3, 29) {
        opts.push(vec![
            one_of(&["EXCLUDE_FROM_FILE", "INCLUDE_FROM_FILE"]),
            InputFile,
        ]);
    }
    if version >= (4, 4) {
        opts.push(vec![k("COVERAGE_TOOL"), Text]);
    }
    ctest_presets(version, &mut opts);
    vec![options(opts)]
}

/// Describe CTest dashboard starts and the later append-only signature.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
///
/// # Returns
/// Legacy model/directories/options before 3.12; afterward a choice of model mode or a required
/// APPEND-only mode.
///
/// # Algorithm
/// Build TRACK/GROUP/QUIET options. Before 3.12 keep model first. Later allow options between
/// ordered model/source/build operands and separately require APPEND for the model-free branch.
///
/// # Flow
/// ```text
///                           [ctest_start inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |    Before 3.12: model first    |    |  Later: model or APPEND-only   |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = ctest_start((3, 12)); // Allows APPEND without a model.
/// ```
fn ctest_start(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut opts = vec![vec![k("TRACK"), Text]];
    if version >= (3, 16) {
        opts.push(vec![k("GROUP"), Text]);
    }
    if version >= (3, 3) {
        opts.push(vec![k("QUIET")]);
    }
    let model = one_of(&["Experimental", "Continuous", "Nightly"]);
    if version < (3, 12) {
        opts.push(vec![k("APPEND")]);
        return vec![
            model,
            optional(vec![InputDirectory, optional(vec![InputDirectory])]),
            options(opts),
        ];
    }
    let append_only = vec![options(opts.clone()), k("APPEND"), options(opts.clone())];
    opts.push(vec![k("APPEND")]);
    // Options can separate positional operands, whose relative order is fixed.
    vec![choice(vec![
        vec![
            options(opts.clone()),
            model,
            options(opts.clone()),
            optional(vec![
                InputDirectory,
                options(opts.clone()),
                optional(vec![InputDirectory, options(opts)]),
            ]),
        ],
        append_only,
    ])]
}

/// Describe CTest result submission and optional CDash upload syntax without networking.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
///
/// # Returns
/// Choice of ordinary repeated submission options and CDASH_UPLOAD InputFile form from 3.2.
///
/// # Algorithm
/// Build versioned PARTS and retry/status/connection groups. Keep CDASH_UPLOAD as a distinct
/// sequence with its own retry/result thresholds; shared connection additions append to both.
///
/// # Flow
/// ```text
///                           [ctest_submit inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |    PARTS / FILES submission    |    |       CDASH_UPLOAD file        |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = ctest_submit((3, 15)); // BUILD_ID is an output variable.
/// ```
fn ctest_submit(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut parts = vec![
        "Start",
        "Update",
        "Configure",
        "Build",
        "Test",
        "Coverage",
        "MemCheck",
        "Notes",
        "ExtraFiles",
        "Upload",
        "Submit",
    ];
    if version >= (3, 14) {
        parts.push("Done");
    }
    let mut opts = vec![
        vec![k("PARTS"), some(one_of(&parts))],
        vec![k("FILES"), some(InputFile)],
        vec![k("RETRY_COUNT"), Integer],
        vec![k("RETRY_DELAY"), Number],
        vec![k("RETURN_VALUE"), Out],
    ];
    opts.extend(ctest_status(version, (3, 13)));
    submit_connection(version, &mut opts);
    let mut forms = vec![vec![options(opts)]];
    if version >= (3, 2) {
        let mut upload = vec![vec![k("CDASH_UPLOAD_TYPE"), Text]];
        if version >= (3, 8) {
            upload.extend([
                vec![k("RETRY_COUNT"), Integer],
                vec![k("RETRY_DELAY"), Number],
                vec![k("QUIET")],
            ]);
        }
        if version >= (3, 13) {
            upload.push(vec![k("RETURN_VALUE"), Out]);
        }
        submit_connection(version, &mut upload);
        forms.push(vec![k("CDASH_UPLOAD"), InputFile, options(upload)]);
    }
    vec![choice(forms)]
}

/// Append HTTP header, submission URL and build-ID output groups by version.
/// Mutates only the supplied option vector; no project commands or I/O.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
/// - `opts` `[in,out]`: existing alternatives receiving appended connection groups; never cleared
///   or overwritten.
///
/// # Returns
/// Unit; opts retains its previous groups and gains the supported connection groups.
///
/// # Examples
/// ```text
/// let mut opts = Vec::new();
/// submit_connection((3, 15), &mut opts);
/// assert_eq!(opts.len(), 3);
/// ```
fn submit_connection(version: (u32, u32), opts: &mut Vec<Vec<ArgSchemaKind>>) {
    if version >= (3, 9) {
        opts.push(vec![k("HTTPHEADER"), Text]);
    }
    if version >= (3, 14) {
        opts.push(vec![k("SUBMIT_URL"), Url]);
    }
    if version >= (3, 15) {
        opts.push(vec![k("BUILD_ID"), Out]);
    }
}

/// Describe CTest source-update options without contacting version control.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
///
/// # Returns
/// One repeated-options root with source directory/result/status controls and 4.4
/// version-only/override groups.
///
/// # Examples
/// ```text
/// let args = ctest_update((4, 4)); // Includes VERSION_ONLY.
/// ```
fn ctest_update(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut opts = vec![
        vec![k("SOURCE"), InputDirectory],
        vec![k("RETURN_VALUE"), Out],
    ];
    opts.extend(ctest_status(version, (3, 13)));
    if version >= (4, 4) {
        opts.extend([vec![k("VERSION_ONLY")], vec![k("VERSION_OVERRIDE"), Text]]);
    }
    vec![options(opts)]
}

/// Describe host queries without inspecting processors, operating systems or registries.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
///
/// # Returns
/// RESULT Out QUERY followed by query-key lists or the Windows-registry form from 3.24.
///
/// # Algorithm
/// Build documented literal query keys by version; from 3.22 add an open value branch for
/// `DISTRIB_<name>`. Keep registry selectors/options separate. Only explicit literal keys become
/// keyword suggestions.
///
/// # Flow
/// ```text
///                     [host_system_information inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |    Versioned query-key list    |    |   Windows registry selectors   |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = host_system_information((3, 24)); // Registry ERROR_VARIABLE is an output.
/// ```
fn host_system_information(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut keys = vec![
        "NUMBER_OF_LOGICAL_CORES",
        "NUMBER_OF_PHYSICAL_CORES",
        "HOSTNAME",
        "FQDN",
        "TOTAL_VIRTUAL_MEMORY",
        "AVAILABLE_VIRTUAL_MEMORY",
        "TOTAL_PHYSICAL_MEMORY",
        "AVAILABLE_PHYSICAL_MEMORY",
    ];
    if version >= (3, 10) {
        keys.extend([
            "IS_64BIT",
            "HAS_FPU",
            "HAS_MMX",
            "HAS_MMX_PLUS",
            "HAS_SSE",
            "HAS_SSE2",
            "HAS_SSE_FP",
            "HAS_SSE_MMX",
            "HAS_AMD_3DNOW",
            "HAS_AMD_3DNOW_PLUS",
            "HAS_IA64",
            "HAS_SERIAL_NUMBER",
            "PROCESSOR_SERIAL_NUMBER",
            "PROCESSOR_NAME",
            "PROCESSOR_DESCRIPTION",
            "OS_NAME",
            "OS_RELEASE",
            "OS_VERSION",
            "OS_PLATFORM",
        ]);
    }
    if version >= (3, 22) {
        keys.push("DISTRIB_INFO");
    }
    if version >= (3, 28) {
        keys.push("MSYSTEM_PREFIX");
    }
    if version >= (4, 3) {
        keys.extend([
            "HAS_APIC",
            "HAS_L1_CACHE",
            "LOCALE_CHARSET",
            "FAMILY_ID",
            "MODEL_ID",
            "MODEL_NAME",
            "PROCESSOR_APIC_ID",
            "PROCESSOR_CACHE_SIZE",
            "PROCESSOR_CLOCK_FREQUENCY",
            "VENDOR_ID",
            "VENDOR_STRING",
        ]);
    }
    let mut key_forms = keys.iter().map(|word| vec![k(word)]).collect::<Vec<_>>();
    // DISTRIB_<name> is open-ended; the AST has no prefix-pattern scalar.
    if version >= (3, 22) {
        key_forms.push(vec![Value]);
    }
    let mut forms = vec![vec![some(choice(key_forms))]];
    if version >= (3, 24) {
        forms.push(vec![
            k("WINDOWS_REGISTRY"),
            Text,
            optional(vec![choice(vec![
                vec![one_of(&["VALUE_NAMES", "SUBKEYS"])],
                vec![k("VALUE"), Text],
            ])]),
            options(vec![
                vec![
                    k("VIEW"),
                    one_of(&["64", "32", "64_32", "32_64", "HOST", "TARGET", "BOTH"]),
                ],
                vec![k("SEPARATOR"), Text],
                vec![k("ERROR_VARIABLE"), Out],
            ]),
        ]);
    }
    vec![k("RESULT"), Out, k("QUERY"), choice(forms)]
}

/// Describe dynamic language dispatch, deferred calls and later control modes as data.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
///
/// # Returns
/// One choice of eligible CALL/EVAL/DEFER/provider/log-level/exit/trace sequences; none is
/// executed.
///
/// # Algorithm
/// Keep CALL/EVAL base forms, then append DEFER with distinct call/query/cancel modes. Add
/// dependency provider, log-level, exit and tracing syntax by version; explicit query outputs
/// retain Out roles.
///
/// # Flow
/// ```text
///                          [cmake_language inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |      CALL / EVAL / DEFER       |    |Provider / query / exit / trace |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = cmake_language((4, 2)); // Describes TRACE ON EXPAND.
/// ```
fn cmake_language(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut forms = vec![
        vec![k("CALL"), Command, many(Value)],
        vec![k("EVAL"), k("CODE"), some(Text)],
    ];
    if version >= (3, 19) {
        forms.push(vec![
            k("DEFER"),
            choice(vec![
                vec![
                    options(vec![
                        vec![k("DIRECTORY"), InputDirectory],
                        vec![k("ID"), Text],
                        vec![k("ID_VAR"), Out],
                    ]),
                    k("CALL"),
                    Command,
                    many(Value),
                ],
                vec![
                    optional(vec![k("DIRECTORY"), InputDirectory]),
                    choice(vec![
                        vec![k("GET_CALL_IDS"), Out],
                        vec![k("GET_CALL"), Text, Out],
                        vec![k("CANCEL_CALL"), some(Text)],
                    ]),
                ],
            ]),
        ]);
    }
    if version >= (3, 24) {
        forms.push(vec![
            k("SET_DEPENDENCY_PROVIDER"),
            Command,
            k("SUPPORTED_METHODS"),
            many(one_of(&[
                "FIND_PACKAGE",
                "FETCHCONTENT_MAKEAVAILABLE_SERIAL",
            ])),
        ]);
    }
    if version >= (3, 25) {
        forms.push(vec![k("GET_MESSAGE_LOG_LEVEL"), Out]);
    }
    if version >= (3, 29) {
        forms.push(vec![k("EXIT"), Integer]);
    }
    if version >= (4, 2) {
        forms.push(vec![
            k("TRACE"),
            choice(vec![
                vec![k("ON"), optional(vec![k("EXPAND")])],
                vec![k("OFF")],
            ]),
        ]);
    }
    vec![choice(forms)]
}

/// Describe native pkg-config extraction and newer population/import modes.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
///
/// # Returns
/// One choice beginning with EXTRACT, plus POPULATE and IMPORT from 4.1; package names and
/// optional versions lead each mode.
///
/// # Algorithm
/// Share requirement/environment/search settings. Add extraction-specific system filters, then
/// population bindings and import name options only for their eligible modes. Search paths remain
/// hints.
///
/// # Flow
/// ```text
///                            [pkg_config inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |    EXTRACT + system filters    |    |   POPULATE / IMPORT bindings   |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = pkg_config((4, 1)); // BIND_PC_REQUIRES consumes combined values, not Target roles.
/// ```
fn pkg_config(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let common = vec![
        vec![one_of(&["REQUIRED", "EXACT", "QUIET"])],
        vec![
            k("STRICTNESS"),
            one_of(&["STRICT", "PERMISSIVE", "BEST_EFFORT"]),
        ],
        vec![k("ENV_MODE"), one_of(&["FDO", "PKGCONF", "IGNORE"])],
        vec![one_of(&["PC_LIBDIR", "PC_PATH"]), some(Path)],
        vec![k("DISABLE_UNINSTALLED"), Boolean],
        vec![one_of(&["PC_SYSROOT_DIR", "TOP_BUILD_DIR"]), Path],
    ];
    let mut extract = common.clone();
    extract.extend([
        vec![
            one_of(&["SYSTEM_INCLUDE_DIRS", "SYSTEM_LIBRARY_DIRS"]),
            some(Path),
        ],
        vec![
            one_of(&["ALLOW_SYSTEM_INCLUDES", "ALLOW_SYSTEM_LIBS"]),
            Boolean,
        ],
    ]);
    let mut forms = vec![vec![
        k("EXTRACT"),
        PackageName,
        optional(vec![Version]),
        options(extract),
    ]];
    if version >= (4, 1) {
        let mut populate = common;
        // BIND_PC_REQUIRES contains combined name=target values, not bare targets.
        populate.extend([
            vec![k("PREFIX"), Text],
            vec![k("BIND_PC_REQUIRES"), some(Value)],
        ]);
        forms.push(vec![
            k("POPULATE"),
            PackageName,
            optional(vec![Version]),
            options(populate.clone()),
        ]);
        populate.push(vec![k("NAME"), Text]);
        forms.push(vec![
            k("IMPORT"),
            PackageName,
            optional(vec![Version]),
            options(populate),
        ]);
    }
    vec![choice(forms)]
}

/// Describe instrumentation hooks and metadata callbacks without invoking them.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor selecting syntax additions.
///
/// # Returns
/// Required API_VERSION integer and DATA_VERSION version followed by repeated
/// hook/options/callback/custom-content groups.
///
/// # Algorithm
/// Keep hook and content-type vocabularies explicit; add new instrumentation option literals from
/// 4.4. CALLBACK names a program and arbitrary values but is never run by this builder.
///
/// # Flow
/// ```text
///                         [instrumentation inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |    API/data version prefix     |    |  Hooks / options / callback /  |
/// |                                |    |            content             |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = instrumentation((4, 4)); // Adds captureOutput and compileTrace options.
/// ```
fn instrumentation(version: (u32, u32)) -> Vec<ArgSchemaKind> {
    let mut instrumentation_options = vec![
        "staticSystemInformation",
        "dynamicSystemInformation",
        "cdashSubmit",
        "cdashVerbose",
        "trace",
    ];
    if version >= (4, 4) {
        instrumentation_options.extend(["captureOutput", "compileTrace"]);
    }
    vec![
        k("API_VERSION"),
        Integer,
        k("DATA_VERSION"),
        Version,
        options(vec![
            vec![
                k("HOOKS"),
                some(one_of(&[
                    "postGenerate",
                    "preBuild",
                    "postBuild",
                    "preCMakeBuild",
                    "postCMakeBuild",
                    "postCMakeInstall",
                    "postCMakeWorkflow",
                    "postCTest",
                ])),
            ],
            vec![k("OPTIONS"), some(one_of(&instrumentation_options))],
            vec![k("CALLBACK"), Command, many(Value)],
            vec![
                k("CUSTOM_CONTENT"),
                Text,
                one_of(&["STRING", "BOOL", "LIST", "JSON"]),
                Value,
            ],
        ]),
    ]
}

/// Describe the literal severity-action vocabulary for cmake_diagnostic.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// None.
///
/// # Returns
/// One-keyword choice of IGNORE, WARN, SEND_ERROR and FATAL_ERROR; does not emit a diagnostic.
///
/// # Examples
/// ```text
/// let action = diagnostic_action(); // For cmake_diagnostic(SET group WARN).
/// ```
fn diagnostic_action() -> ArgSchemaKind {
    one_of(&["IGNORE", "WARN", "SEND_ERROR", "FATAL_ERROR"])
}

// Contains AI-generated test-module documentation edits.
/// Process grammar regression tests loaded from the package test tree.
#[cfg(test)]
#[path = "../../../../../tests/unit/parser/semanticer/keyword_process.rs"]
mod keyword_process;
