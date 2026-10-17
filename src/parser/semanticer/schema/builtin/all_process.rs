// AI-generated implementation.
use super::{
    ArgSchemaKind, Boolean, Command, Expression, InputDirectory, InputFile, Integer, Number, Out,
    OutputPath, PackageName, Path, Target, TargetExe, Text, Url, Value, Version, choice, k, many,
    one_of, optional, options, pairs, some,
};

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

fn ctest_presets(version: (u32, u32), opts: &mut Vec<Vec<ArgSchemaKind>>) {
    if version >= (4, 4) {
        opts.extend([vec![k("PRESET"), Text], vec![k("PRESETS_FILE"), InputFile]]);
    }
}

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

fn diagnostic_action() -> ArgSchemaKind {
    one_of(&["IGNORE", "WARN", "SEND_ERROR", "FATAL_ERROR"])
}

#[cfg(test)]
#[path = "../../../../../tests/unit/parser/semanticer/keyword_process.rs"]
mod keyword_process;
