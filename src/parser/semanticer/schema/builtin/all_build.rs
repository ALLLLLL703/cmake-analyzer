// AI-generated implementation.
use super::{
    ArgSchemaKind, Boolean, Command, Expression, InputDirectory, InputFile, InputPath, Integer,
    LanguageType, Out, OutputPath, PackageName, Path, Target, TargetExe, TargetLib, TestName, Text,
    Url, Value, Variable, Version, choice, flags, k, many, one_of, optional, options, some,
};

type VersionPair = (u32, u32);

pub(super) fn schemas(version: VersionPair) -> Vec<(&'static str, Vec<ArgSchemaKind>)> {
    let mut schemas = vec![
        ("add_custom_command", custom_command(version)),
        ("add_custom_target", custom_target(version)),
        ("add_executable", executable(version)),
        ("add_library", library(version)),
        ("add_test", test(version)),
        ("add_subdirectory", subdirectory(version)),
        ("build_command", build_command(version)),
        ("configure_file", configure_file(version)),
        ("export", export(version)),
        ("install", install(version)),
        ("try_compile", try_compile(version)),
        ("try_run", try_run(version)),
        (
            "include_external_msproject",
            vec![
                Target,
                InputFile,
                options(vec![
                    vec![k("TYPE"), Text],
                    vec![k("GUID"), Text],
                    vec![k("PLATFORM"), Text],
                ]),
                many(Target),
            ],
        ),
        (
            "install_files",
            vec![
                OutputPath,
                choice(vec![
                    vec![k("FILES"), some(InputFile)],
                    vec![Text, some(InputFile)],
                    vec![Expression],
                ]),
            ],
        ),
        (
            "install_programs",
            vec![
                OutputPath,
                choice(vec![
                    vec![k("FILES"), some(InputFile)],
                    vec![some(InputFile)],
                    vec![Expression],
                ]),
            ],
        ),
        (
            "install_targets",
            vec![
                OutputPath,
                optional(vec![k("RUNTIME_DIRECTORY"), OutputPath]),
                some(Target),
            ],
        ),
        (
            "subdirs",
            vec![
                some(InputDirectory),
                options(vec![
                    vec![k("EXCLUDE_FROM_ALL"), many(InputDirectory)],
                    vec![k("PREORDER"), many(InputDirectory)],
                ]),
            ],
        ),
    ];
    if version >= (3, 1) {
        schemas.push(("target_sources", target_sources(version)));
    }
    if version >= (3, 16) {
        schemas.push((
            "target_precompile_headers",
            vec![
                Target,
                choice(vec![
                    vec![k("REUSE_FROM"), Target],
                    // Headers may be angle-bracket includes or quoted include tokens.
                    vec![some(choice(vec![vec![scope(), many(Value)]]))],
                ]),
            ],
        ));
    }
    schemas
}

fn scope() -> ArgSchemaKind {
    one_of(&["INTERFACE", "PUBLIC", "PRIVATE"])
}

fn dependency() -> ArgSchemaKind {
    choice(vec![vec![Target], vec![InputPath]])
}

fn command_body(legacy_args: bool) -> Vec<ArgSchemaKind> {
    let mut args = vec![choice(vec![vec![TargetExe], vec![Command]])];
    if legacy_args {
        args.push(optional(vec![k("ARGS")]));
    }
    // The executed program owns the meaning of its arguments.
    args.push(many(Value));
    args
}

fn custom_options(version: VersionPair, output: bool, target: bool) -> Vec<Vec<ArgSchemaKind>> {
    let mut command = vec![k("COMMAND")];
    command.extend(command_body(!target));
    let mut args = vec![
        command,
        vec![k("WORKING_DIRECTORY"), Path],
        vec![k("COMMENT"), Text],
        vec![k("VERBATIM")],
    ];
    if output || target {
        args.push(vec![k("DEPENDS"), many(dependency())]);
    }
    if version >= (3, 2) {
        args.extend([
            vec![k("BYPRODUCTS"), many(OutputPath)],
            vec![k("USES_TERMINAL")],
        ]);
    }
    if version >= (3, 8) {
        args.push(vec![k("COMMAND_EXPAND_LISTS")]);
    }
    if output {
        args.extend([
            vec![k("MAIN_DEPENDENCY"), InputFile],
            vec![
                k("IMPLICIT_DEPENDS"),
                some(choice(vec![vec![LanguageType, InputFile]])),
            ],
            vec![k("APPEND")],
        ]);
        if version >= (3, 7) {
            // The depfile is produced by the command, not a configure-time input.
            args.push(vec![k("DEPFILE"), OutputPath]);
        }
        if version >= (3, 27) {
            args.push(vec![k("DEPENDS_EXPLICIT_ONLY")]);
        }
        if version >= (3, 31) {
            args.push(vec![k("CODEGEN")]);
        }
    }
    if output || target {
        if version >= (3, 15) {
            args.push(vec![k("JOB_POOL"), Text]);
        }
        if version >= (3, 28) {
            args.push(vec![k("JOB_SERVER_AWARE"), Boolean]);
        }
    }
    if target {
        args.push(vec![k("SOURCES"), many(InputFile)]);
    }
    args
}

fn custom_command(version: VersionPair) -> Vec<ArgSchemaKind> {
    vec![choice(vec![
        vec![
            k("OUTPUT"),
            some(OutputPath),
            options(custom_options(version, true, false)),
        ],
        vec![
            k("TARGET"),
            Target,
            one_of(&["PRE_BUILD", "PRE_LINK", "POST_BUILD"]),
            k("COMMAND"),
            choice(vec![command_body(true)]),
            options(custom_options(version, false, false)),
        ],
    ])]
}

fn custom_target(version: VersionPair) -> Vec<ArgSchemaKind> {
    vec![
        Target,
        optional(vec![k("ALL")]),
        optional(command_body(false)),
        options(custom_options(version, false, true)),
    ]
}

fn executable(version: VersionPair) -> Vec<ArgSchemaKind> {
    vec![
        TargetExe,
        choice(vec![
            vec![
                flags(&["WIN32", "MACOSX_BUNDLE", "EXCLUDE_FROM_ALL"]),
                if version >= (3, 11) {
                    many(InputFile)
                } else {
                    some(InputFile)
                },
            ],
            vec![k("IMPORTED"), optional(vec![k("GLOBAL")])],
            vec![k("ALIAS"), TargetExe],
        ]),
    ]
}

fn library(version: VersionPair) -> Vec<ArgSchemaKind> {
    let mut imported = vec!["STATIC", "SHARED", "MODULE", "UNKNOWN", "INTERFACE"];
    if version >= (3, 9) {
        imported.push("OBJECT");
    }
    let mut forms = vec![
        vec![
            optional(vec![one_of(&["STATIC", "SHARED", "MODULE", "OBJECT"])]),
            optional(vec![k("EXCLUDE_FROM_ALL")]),
            if version >= (3, 11) {
                many(InputFile)
            } else {
                some(InputFile)
            },
        ],
        vec![
            one_of(&imported),
            k("IMPORTED"),
            optional(vec![k("GLOBAL")]),
        ],
        vec![k("ALIAS"), TargetLib],
    ];
    forms.push(if version >= (3, 19) {
        vec![
            k("INTERFACE"),
            optional(vec![k("EXCLUDE_FROM_ALL")]),
            many(InputFile),
        ]
    } else {
        vec![k("INTERFACE")]
    });
    if version >= (4, 2) {
        forms.push(vec![k("INTERFACE"), k("SYMBOLIC")]);
    }
    vec![TargetLib, choice(forms)]
}

fn test(version: VersionPair) -> Vec<ArgSchemaKind> {
    let mut args = vec![
        vec![k("CONFIGURATIONS"), some(Text)],
        vec![k("WORKING_DIRECTORY"), Path],
    ];
    if version >= (3, 16) {
        args.push(vec![k("COMMAND_EXPAND_LISTS")]);
    }
    if version >= (4, 4) {
        args.push(vec![k("BUILD_DEPENDS"), many(dependency())]);
    }
    vec![choice(vec![
        vec![
            k("NAME"),
            TestName,
            k("COMMAND"),
            choice(vec![command_body(false)]),
            options(args),
        ],
        vec![TestName, Command, many(Value)],
    ])]
}

fn subdirectory(version: VersionPair) -> Vec<ArgSchemaKind> {
    let mut args = vec![vec![k("EXCLUDE_FROM_ALL")]];
    if version >= (3, 25) {
        args.push(vec![k("SYSTEM")]);
    }
    vec![InputDirectory, optional(vec![OutputPath]), options(args)]
}

fn build_command(version: VersionPair) -> Vec<ArgSchemaKind> {
    let mut args = vec![
        vec![k("CONFIGURATION"), Text],
        vec![k("TARGET"), Target],
        vec![k("PROJECT_NAME"), Text],
    ];
    if version >= (3, 21) {
        args.push(vec![k("PARALLEL_LEVEL"), Integer]);
    }
    vec![Out, choice(vec![vec![options(args)], vec![Path]])]
}

fn permissions() -> ArgSchemaKind {
    some(one_of(&[
        "OWNER_READ",
        "OWNER_WRITE",
        "OWNER_EXECUTE",
        "GROUP_READ",
        "GROUP_WRITE",
        "GROUP_EXECUTE",
        "WORLD_READ",
        "WORLD_WRITE",
        "WORLD_EXECUTE",
        "SETUID",
        "SETGID",
    ]))
}

fn configure_file(version: VersionPair) -> Vec<ArgSchemaKind> {
    let mut args = vec![
        vec![k("COPYONLY")],
        vec![k("ESCAPE_QUOTES")],
        vec![k("@ONLY")],
        vec![
            k("NEWLINE_STYLE"),
            one_of(&["UNIX", "DOS", "WIN32", "LF", "CRLF"]),
        ],
    ];
    if version >= (3, 19) {
        args.push(vec![k("NO_SOURCE_PERMISSIONS")]);
    }
    if version >= (3, 20) {
        args.extend([
            vec![k("USE_SOURCE_PERMISSIONS")],
            vec![k("FILE_PERMISSIONS"), permissions()],
        ]);
    }
    vec![InputFile, OutputPath, options(args)]
}

fn target_sources(version: VersionPair) -> Vec<ArgSchemaKind> {
    let mut contents = vec![vec![InputFile]];
    if version >= (3, 23) {
        let mut types = vec!["HEADERS"];
        if version >= (3, 28) {
            types.push("CXX_MODULES");
        }
        if version >= (4, 4) {
            types.push("SOURCES");
        }
        contents.push(vec![
            k("FILE_SET"),
            choice(vec![vec![one_of(&types)], vec![Text]]),
            options(vec![
                vec![k("TYPE"), one_of(&types)],
                vec![k("BASE_DIRS"), many(Path)],
                vec![k("FILES"), many(InputFile)],
            ]),
        ]);
    }
    vec![
        Target,
        some(choice(vec![vec![scope(), many(choice(contents))]])),
    ]
}

fn export(version: VersionPair) -> Vec<ArgSchemaKind> {
    let mut target_options = vec![
        vec![k("NAMESPACE"), Text],
        vec![k("APPEND")],
        vec![k("EXPORT_LINK_INTERFACE_LIBRARIES")],
    ];
    if version >= (3, 28) {
        target_options.push(vec![k("CXX_MODULES_DIRECTORY"), OutputPath]);
    }
    let mut export_options = vec![vec![k("NAMESPACE"), Text], vec![k("FILE"), OutputPath]];
    if version >= (3, 28) {
        export_options.push(vec![k("CXX_MODULES_DIRECTORY"), OutputPath]);
    }
    if version >= (3, 29) {
        // Requires CMAKE_EXPERIMENTAL_EXPORT_PACKAGE_DEPENDENCIES.
        export_options.push(vec![k("EXPORT_PACKAGE_DEPENDENCIES")]);
    }
    let mut forms = vec![
        vec![
            k("TARGETS"),
            many(Target),
            options(target_options.clone()),
            k("FILE"),
            OutputPath,
            options(target_options),
        ],
        vec![k("EXPORT"), Text, options(export_options)],
        vec![k("PACKAGE"), PackageName],
    ];
    if version >= (3, 7) {
        forms.push(vec![
            k("TARGETS"),
            many(Target),
            k("ANDROID_MK"),
            OutputPath,
        ]);
    }
    if version >= (3, 29) {
        forms.push(vec![
            k("SETUP"),
            Text,
            options(vec![
                vec![
                    k("PACKAGE_DEPENDENCY"),
                    PackageName,
                    options(vec![
                        vec![k("ENABLED"), choice(vec![vec![k("AUTO")], vec![Boolean]])],
                        vec![k("EXTRA_ARGS"), many(Value)],
                    ]),
                ],
                vec![
                    k("TARGET"),
                    Target,
                    optional(vec![k("XCFRAMEWORK_LOCATION"), Path]),
                ],
            ]),
        ]);
    }
    // Before stabilization in 4.3, v4.1/v4.2 used EXPORT <set> PACKAGE_INFO <name>.
    // https://cmake.org/cmake/help/v4.1/command/export.html
    if version >= (4, 3) {
        forms.push(vec![
            k("PACKAGE_INFO"),
            PackageName,
            k("EXPORT"),
            Text,
            package_info(version, false),
        ]);
    } else if version >= (4, 1) {
        forms.push(vec![
            k("EXPORT"),
            Text,
            k("PACKAGE_INFO"),
            PackageName,
            package_info(version, false),
        ]);
    }
    if version >= (4, 3) {
        forms.push(vec![
            k("SBOM"),
            Text,
            k("EXPORTS"),
            some(Text),
            options(sbom_options(false)),
        ]);
    }
    vec![choice(forms)]
}

fn package_info(version: VersionPair, installing: bool) -> ArgSchemaKind {
    let mut common = vec![vec![k("LOWER_CASE_FILE")]];
    if version >= (4, 1) {
        common.extend([vec![k("DESCRIPTION"), Text], vec![k("HOMEPAGE_URL"), Url]]);
    }
    if version >= (4, 2) {
        common.extend([vec![k("LICENSE"), Text], vec![k("DEFAULT_LICENSE"), Text]]);
    }
    if installing {
        common.extend(install_common(version));
        common.push(vec![k("DESTINATION"), OutputPath]);
    }
    if version >= (4, 3) {
        common.push(vec![k("CXX_MODULES_DIRECTORY"), OutputPath]);
    }
    let mut metadata = vec![
        vec![
            k("VERSION"),
            Version,
            options(vec![
                vec![k("COMPAT_VERSION"), Version],
                vec![k("VERSION_SCHEMA"), Text],
            ]),
        ],
        vec![k("DEFAULT_TARGETS"), some(Target)],
        vec![k("DEFAULT_CONFIGURATIONS"), some(Text)],
    ];
    if version >= (4, 1) {
        metadata.push(vec![choice(vec![
            vec![k("PROJECT"), Text],
            vec![k("NO_PROJECT_METADATA")],
        ])]);
    }
    // APPENDIX cannot change basic package metadata.
    choice(vec![
        vec![
            options(common.clone()),
            k("APPENDIX"),
            Text,
            options(common.clone()),
        ],
        vec![options({
            common.extend(metadata);
            common
        })],
    ])
}

fn sbom_options(installing: bool) -> Vec<Vec<ArgSchemaKind>> {
    // Requires CMAKE_EXPERIMENTAL_GENERATE_SBOM (4.3+).
    let mut args = vec![
        vec![k("FORMAT"), Text],
        vec![choice(vec![
            vec![k("PROJECT"), Text],
            vec![k("NO_PROJECT_METADATA")],
        ])],
        vec![k("VERSION"), Version],
        vec![k("LICENSE"), Text],
        vec![k("DESCRIPTION"), Text],
        vec![k("HOMEPAGE_URL"), Url],
        vec![k("PACKAGE_URL"), Url],
    ];
    if installing {
        args.push(vec![k("DESTINATION"), OutputPath]);
    }
    args
}

fn install_common(version: VersionPair) -> Vec<Vec<ArgSchemaKind>> {
    let mut args = vec![
        vec![k("PERMISSIONS"), permissions()],
        vec![k("CONFIGURATIONS"), some(Text)],
        vec![k("COMPONENT"), Text],
    ];
    if version >= (3, 6) {
        args.push(vec![k("EXCLUDE_FROM_ALL")]);
    }
    args
}

fn artifact_options(version: VersionPair, namelink: bool) -> Vec<Vec<ArgSchemaKind>> {
    let mut args = install_common(version);
    args.extend([vec![k("DESTINATION"), OutputPath], vec![k("OPTIONAL")]]);
    if namelink {
        args.push(vec![one_of(&["NAMELINK_ONLY", "NAMELINK_SKIP"])]);
        if version >= (3, 12) {
            args.push(vec![k("NAMELINK_COMPONENT"), Text]);
        }
    }
    args
}

fn artifact_groups(
    version: VersionPair,
    kinds: &[&str],
    filesets: bool,
    namelink: bool,
) -> ArgSchemaKind {
    let mut groups = vec![vec![
        one_of(kinds),
        options(artifact_options(version, namelink)),
    ]];
    if filesets && version >= (3, 23) {
        groups.push(vec![
            k("FILE_SET"),
            Text,
            options(artifact_options(version, false)),
        ]);
    }
    options(groups)
}

fn runtime_filters() -> Vec<Vec<ArgSchemaKind>> {
    vec![
        vec![
            one_of(&[
                "PRE_INCLUDE_REGEXES",
                "PRE_EXCLUDE_REGEXES",
                "POST_INCLUDE_REGEXES",
                "POST_EXCLUDE_REGEXES",
            ]),
            many(Expression),
        ],
        // These are dependency search/filter hints, not required existing inputs.
        vec![
            one_of(&["POST_INCLUDE_FILES", "POST_EXCLUDE_FILES"]),
            many(Path),
        ],
        vec![k("DIRECTORIES"), many(Path)],
    ]
}

fn install_destination(version: VersionPair) -> ArgSchemaKind {
    let mut forms = vec![vec![k("DESTINATION"), OutputPath]];
    if version >= (3, 14) {
        let mut types = vec![
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
        ];
        if version >= (3, 31) {
            types.push("LIBEXEC");
        }
        forms.push(vec![k("TYPE"), one_of(&types)]);
    }
    choice(forms)
}

fn install(version: VersionPair) -> Vec<ArgSchemaKind> {
    let mut kinds = vec![
        "ARCHIVE",
        "LIBRARY",
        "RUNTIME",
        "FRAMEWORK",
        "BUNDLE",
        "PRIVATE_HEADER",
        "PUBLIC_HEADER",
        "RESOURCE",
    ];
    if version >= (3, 9) {
        kinds.push("OBJECTS");
    }
    if version >= (3, 28) {
        kinds.push("CXX_MODULES_BMI");
    }
    let mut targets = vec![
        k("TARGETS"),
        many(Target),
        optional(vec![k("EXPORT"), Text]),
    ];
    if version >= (3, 21) {
        targets.push(optional(vec![choice(vec![
            vec![k("RUNTIME_DEPENDENCY_SET"), Text],
            vec![k("RUNTIME_DEPENDENCIES"), options(runtime_filters())],
        ])]));
    }
    targets.extend([
        options(artifact_options(version, true)),
        artifact_groups(version, &kinds, true, true),
        optional(vec![k("INCLUDES"), k("DESTINATION"), many(OutputPath)]),
    ]);
    let mut files = install_common(version);
    files.extend([vec![k("RENAME"), Text], vec![k("OPTIONAL")]]);
    let mut directories = vec![
        vec![k("FILE_PERMISSIONS"), permissions()],
        vec![k("DIRECTORY_PERMISSIONS"), permissions()],
        vec![k("USE_SOURCE_PERMISSIONS")],
        vec![k("OPTIONAL")],
        vec![k("CONFIGURATIONS"), some(Text)],
        vec![k("COMPONENT"), Text],
    ];
    if version >= (3, 1) {
        directories.push(vec![k("MESSAGE_NEVER")]);
    }
    if version >= (3, 6) {
        directories.push(vec![k("EXCLUDE_FROM_ALL")]);
    }
    let mut components = vec![vec![k("COMPONENT"), Text]];
    if version >= (3, 21) {
        components.push(vec![k("ALL_COMPONENTS")]);
    }
    let mut scripts = vec![optional(vec![choice(components)])];
    if version >= (3, 6) {
        scripts.push(optional(vec![k("EXCLUDE_FROM_ALL")]));
    }
    let mut exports = install_common(version);
    exports.extend([
        vec![k("NAMESPACE"), Text],
        vec![k("FILE"), Text],
        vec![k("EXPORT_LINK_INTERFACE_LIBRARIES")],
    ]);
    if version >= (3, 28) {
        exports.push(vec![k("CXX_MODULES_DIRECTORY"), OutputPath]);
    }
    if version >= (3, 29) {
        exports.push(vec![k("EXPORT_PACKAGE_DEPENDENCIES")]);
    }
    let mut forms = vec![
        targets,
        vec![
            one_of(&["FILES", "PROGRAMS"]),
            many(InputFile),
            install_destination(version),
            options(files),
        ],
        vec![
            k("DIRECTORY"),
            many(InputDirectory),
            install_destination(version),
            options(directories),
            optional(vec![k("FILES_MATCHING")]),
            options(vec![vec![
                one_of(&["PATTERN", "REGEX"]),
                Expression,
                options(vec![
                    vec![k("EXCLUDE")],
                    vec![k("PERMISSIONS"), permissions()],
                ]),
            ]]),
        ],
        {
            let mut args = vec![some(choice(vec![
                vec![k("SCRIPT"), InputFile],
                vec![k("CODE"), Text],
            ]))];
            args.extend(scripts);
            args
        },
        vec![
            k("EXPORT"),
            Text,
            k("DESTINATION"),
            OutputPath,
            options(exports.clone()),
        ],
    ];
    if version >= (3, 7) {
        forms.push(vec![
            k("EXPORT_ANDROID_MK"),
            Text,
            k("DESTINATION"),
            OutputPath,
            options(exports),
        ]);
    }
    if version >= (3, 21) {
        forms.push(vec![
            k("IMPORTED_RUNTIME_ARTIFACTS"),
            many(Target),
            optional(vec![k("RUNTIME_DEPENDENCY_SET"), Text]),
            options(artifact_options(version, false)),
            artifact_groups(
                version,
                &["LIBRARY", "RUNTIME", "FRAMEWORK", "BUNDLE"],
                false,
                false,
            ),
        ]);
        let mut runtime_artifacts = artifact_options(version, false);
        runtime_artifacts.push(vec![k("NAMELINK_COMPONENT"), Text]);
        forms.push(vec![
            k("RUNTIME_DEPENDENCY_SET"),
            Text,
            options(runtime_artifacts.clone()),
            options(vec![vec![
                one_of(&["LIBRARY", "RUNTIME", "FRAMEWORK"]),
                options(runtime_artifacts),
            ]]),
            options(runtime_filters()),
        ]);
    }
    // Experimental in 3.31..4.2: CMAKE_EXPERIMENTAL_EXPORT_PACKAGE_INFO.
    // https://cmake.org/cmake/help/v4.1/command/install.html
    if version >= (3, 31) {
        forms.push(vec![
            k("PACKAGE_INFO"),
            PackageName,
            k("EXPORT"),
            Text,
            package_info(version, true),
        ]);
    }
    if version >= (4, 3) {
        forms.push(vec![
            k("SBOM"),
            Text,
            k("EXPORTS"),
            some(Text),
            options(sbom_options(true)),
        ]);
    }
    vec![choice(forms)]
}

fn compile_options(version: VersionPair, running: bool, modern: bool) -> Vec<Vec<ArgSchemaKind>> {
    let mut args = vec![
        vec![k("CMAKE_FLAGS"), many(Value)],
        vec![k("COMPILE_DEFINITIONS"), many(Value)],
        vec![
            k("COPY_FILE"),
            OutputPath,
            optional(vec![k("COPY_FILE_ERROR"), Out]),
        ],
    ];
    // try_run gained LINK_LIBRARIES in 3.2 (release/3.2.rst).
    if !running || version >= (3, 2) {
        args.push(vec![
            k("LINK_LIBRARIES"),
            many(choice(vec![vec![TargetLib], vec![Value]])),
        ]);
    }
    if version >= (3, 14) {
        args.push(vec![k("LINK_OPTIONS"), many(Value)]);
    }
    if version >= (3, 29) {
        args.push(vec![k("LINKER_LANGUAGE"), LanguageType]);
    }
    if version >= (3, 8) {
        let mut languages = vec!["C", "CXX", "CUDA"];
        if version >= (3, 16) {
            languages.extend(["OBJC", "OBJCXX"]);
        }
        for lang in languages {
            args.extend([
                vec![k(&format!("{lang}_STANDARD")), Integer],
                vec![k(&format!("{lang}_STANDARD_REQUIRED")), Boolean],
                vec![k(&format!("{lang}_EXTENSIONS")), Boolean],
            ]);
        }
    }
    if modern {
        args.push(vec![k("NO_CACHE")]);
        if version >= (3, 26) {
            args.extend([vec![k("NO_LOG")], vec![k("LOG_DESCRIPTION"), Text]]);
        }
    }
    if running {
        args.extend([
            vec![k("COMPILE_OUTPUT_VARIABLE"), Out],
            vec![k("RUN_OUTPUT_VARIABLE"), Out],
        ]);
        if modern {
            args.extend([
                vec![k("RUN_OUTPUT_STDOUT_VARIABLE"), Out],
                vec![k("RUN_OUTPUT_STDERR_VARIABLE"), Out],
            ]);
        } else {
            args.push(vec![k("OUTPUT_VARIABLE"), Out]);
        }
        if version >= (3, 20) {
            args.push(vec![k("WORKING_DIRECTORY"), Path]);
        }
    } else {
        args.push(vec![k("OUTPUT_VARIABLE"), Out]);
    }
    args
}

fn source_specifications(version: VersionPair) -> ArgSchemaKind {
    let sources = choice(vec![
        vec![k("SOURCES"), some(InputFile)],
        vec![k("SOURCE_FROM_CONTENT"), Text, Text],
        vec![k("SOURCE_FROM_VAR"), Text, Variable],
        vec![k("SOURCE_FROM_FILE"), Text, InputFile],
    ]);
    let source_type = if version >= (3, 28) {
        optional(vec![k("SOURCES_TYPE"), one_of(&["NORMAL", "CXX_MODULE"])])
    } else {
        optional(vec![])
    };
    some(choice(vec![vec![source_type, sources]]))
}

fn try_compile(version: VersionPair) -> Vec<ArgSchemaKind> {
    let mut forms = vec![
        vec![
            OutputPath,
            choice(vec![vec![InputFile], vec![k("SOURCES"), some(InputFile)]]),
            options(compile_options(version, false, false)),
        ],
        vec![
            OutputPath,
            InputDirectory,
            Text,
            optional(vec![Target]),
            options(vec![
                vec![k("CMAKE_FLAGS"), many(Value)],
                vec![k("OUTPUT_VARIABLE"), Out],
            ]),
        ],
    ];
    if version >= (3, 25) {
        forms.push(vec![
            source_specifications(version),
            options(compile_options(version, false, true)),
        ]);
        let mut project = vec![
            vec![k("BINARY_DIR"), OutputPath],
            vec![k("TARGET"), Target],
            vec![k("NO_CACHE")],
            vec![k("CMAKE_FLAGS"), many(Value)],
            vec![k("OUTPUT_VARIABLE"), Out],
        ];
        if version >= (3, 26) {
            project.extend([vec![k("NO_LOG")], vec![k("LOG_DESCRIPTION"), Text]]);
        }
        forms.push(vec![
            k("PROJECT"),
            Text,
            k("SOURCE_DIR"),
            InputDirectory,
            options(project),
        ]);
    }
    vec![Out, choice(forms)]
}

fn try_run(version: VersionPair) -> Vec<ArgSchemaKind> {
    let mut forms = vec![vec![
        OutputPath,
        choice(vec![vec![InputFile], vec![k("SOURCES"), some(InputFile)]]),
        options(compile_options(version, true, false)),
        optional(vec![k("ARGS"), many(Value)]),
    ]];
    if version >= (3, 25) {
        forms.push(vec![
            source_specifications(version),
            options(compile_options(version, true, true)),
            optional(vec![k("ARGS"), many(Value)]),
        ]);
    }
    vec![Out, Out, choice(forms)]
}

#[cfg(test)]
#[path = "../../../../../tests/unit/parser/semanticer/keyword_build.rs"]
mod keyword_build;
