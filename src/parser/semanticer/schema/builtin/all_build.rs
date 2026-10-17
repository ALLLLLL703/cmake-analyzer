//! Structured build, install, export and compile-probe grammars.
//! These builders allocate [`ArgSchemaKind`] trees, not targets or files, and
//! never run a program, compiler or CMake project. `schemas` feeds the final
//! version cache; only names already registered by the native layers survive.
//! Output paths deliberately differ from existing-source inputs: for example,
//! `add_custom_command(OUTPUT generated.c COMMAND tool)` produces a path role,
//! while `try_compile(ok SOURCES probe.c)` exposes an output-variable role and
//! a source-file role. No compile probe is actually performed.
//! Experimental export/install syntax is represented by introduction version;
//! its runtime feature gates, duplicate options and policy constraints are not evaluated.

// AI-generated implementation.
use super::{
    ArgSchemaKind, Boolean, Command, Expression, InputDirectory, InputFile, InputPath, Integer,
    LanguageType, Out, OutputPath, PackageName, Path, Target, TargetExe, TargetLib, TestName, Text,
    Url, Value, Variable, Version, choice, flags, k, many, one_of, optional, options, some,
};

/// CMake major/minor comparison key; patch releases do not select grammar changes.
/// Example: `let version: VersionPair = (3, 25);`.
type VersionPair = (u32, u32);

/// Collect structured construction, installation and probe overrides.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// Owned (command name, ordered root kinds) pairs, including version-eligible target_sources and
/// precompiled-header forms.
///
/// # Algorithm
/// Assemble core command trees and legacy forms; append target_sources from 3.1 and
/// target_precompile_headers from 3.16. Native-name filtering happens in the cache caller.
///
/// # Flow
/// ```text
///                             [schemas inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |      Core/legacy commands      |    |     Version-gated targets      |
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
/// assert!(overrides.iter().any(|(name, _)| *name == "install"));
/// ```
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

/// Describe a target visibility keyword.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// None.
///
/// # Returns
/// A single-keyword choice of INTERFACE, PUBLIC and PRIVATE.
///
/// # Examples
/// ```text
/// let visibility = scope(); // For target_sources(app PRIVATE main.c).
/// ```
fn scope() -> ArgSchemaKind {
    one_of(&["INTERFACE", "PUBLIC", "PRIVATE"])
}

/// Describe a dependency operand that may be a target or existing input path.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// None.
///
/// # Returns
/// A choice between Target and InputPath; equally plausible full-call bindings remain conservative.
///
/// # Examples
/// ```text
/// let operand = dependency(); // For DEPENDS core or DEPENDS input.idl.
/// ```
fn dependency() -> ArgSchemaKind {
    choice(vec![vec![Target], vec![InputPath]])
}

/// Describe a custom command program/target and its uninterpreted arguments.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `legacy_args` `[in]`: permit an optional literal ARGS between program and values.
///
/// # Returns
/// Sequence choosing an executable target or command name, optional ARGS when requested, then
/// arbitrary values.
///
/// # Algorithm
/// Keep target/program alternatives in one slot; optionally add ARGS and finish with values whose
/// meaning belongs to the program.
///
/// # Flow
/// ```text
///                           [command_body inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |  Executable target / command   |    |     Optional ARGS + values     |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let body = command_body(true); // Describes tool ARGS --verbose, without running tool.
/// ```
fn command_body(legacy_args: bool) -> Vec<ArgSchemaKind> {
    let mut args = vec![choice(vec![vec![TargetExe], vec![Command]])];
    if legacy_args {
        args.push(optional(vec![k("ARGS")]));
    }
    // The executed program owns the meaning of its arguments.
    args.push(many(Value));
    args
}

/// Build option-group alternatives shared by custom command and target signatures.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
/// - `output` `[in]`: enable output-form dependencies, MAIN_DEPENDENCY, IMPLICIT_DEPENDS and
///   output-only additions.
/// - `target` `[in]`: enable custom-target DEPENDS, SOURCES, job settings and disable legacy ARGS
///   in nested COMMAND groups.
///
/// # Returns
/// Alternative option sequences, not yet wrapped in repetition; false/false is the
/// target-attached add_custom_command mode.
///
/// # Algorithm
/// Build COMMAND and common groups, gate additions by version, then add output-only or
/// custom-target-only groups. DEPFILE and BYPRODUCTS are generated outputs, not required inputs.
///
/// # Flow
/// ```text
///                          [custom_options inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |    Common + version options    |    | Output / custom-target groups  |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let opts = custom_options((3, 28), true, false); // Includes DEPFILE as OutputPath.
/// ```
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

/// Describe output-generating and target-attached add_custom_command forms.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// One choice of OUTPUT paths/options or TARGET name/build phase/required COMMAND/options.
///
/// # Algorithm
/// Keep the two mode sequences separate. OUTPUT uses output-specific options; TARGET requires a
/// phase and command before shared options.
///
/// # Flow
/// ```text
///                          [custom_command inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |     OUTPUT paths + options     |    |     TARGET phase + COMMAND     |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = custom_command((3, 31)); // Includes output-form CODEGEN.
/// ```
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

/// Describe add_custom_target with optional ALL, initial program and repeated options.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// Target name followed by optional flags/body and custom-target option groups; no target is
/// created.
///
/// # Examples
/// ```text
/// let args = custom_target((3, 28)); // Can describe SOURCES and JOB_SERVER_AWARE.
/// ```
fn custom_target(version: VersionPair) -> Vec<ArgSchemaKind> {
    vec![
        Target,
        optional(vec![k("ALL")]),
        optional(command_body(false)),
        options(custom_options(version, false, true)),
    ]
}

/// Describe normal, imported and alias add_executable signatures.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// Executable target followed by a mode choice; source lists may be empty from 3.11. The alias
/// branch is present in this override tree.
///
/// # Algorithm
/// Choose source cardinality by version, retaining imported GLOBAL and alias executable-target
/// operands as distinct branches.
///
/// # Flow
/// ```text
///                            [executable inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |   Sources and platform flags   |    |        IMPORTED / ALIAS        |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = executable((3, 11)); // Empty source list is structurally accepted.
/// ```
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

/// Describe normal, imported, interface and alias add_library forms.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// Library-target roots with versioned source cardinality, imported kinds and interface variants.
///
/// # Algorithm
/// Add OBJECT imports from 3.9, empty sources from 3.11, interface sources from 3.19 and symbolic
/// interfaces from 4.2. Keep each mode as a full alternative.
///
/// # Flow
/// ```text
///                             [library inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |      Sources / interface       |    |        Imported / alias        |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = library((4, 2)); // Includes INTERFACE SYMBOLIC.
/// ```
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

/// Describe named add_test and its legacy positional form.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// A mode choice; named mode has executable/program arguments and repeated test options.
///
/// # Algorithm
/// Build named-only configuration/directory options, add list expansion and build dependencies at
/// their thresholds, then retain the legacy name/program/value sequence.
///
/// # Flow
/// ```text
///                               [test inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |    NAME + COMMAND + options    |    |     Legacy name + program      |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = test((4, 4)); // Named form includes BUILD_DEPENDS.
/// ```
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

/// Describe source directory, optional binary directory and add_subdirectory flags.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// InputDirectory followed by optional OutputPath and repeated flags; SYSTEM is present from 3.25.
///
/// # Examples
/// ```text
/// let args = subdirectory((3, 25)); // Can describe src build SYSTEM.
/// ```
fn subdirectory(version: VersionPair) -> Vec<ArgSchemaKind> {
    let mut args = vec![vec![k("EXCLUDE_FROM_ALL")]];
    if version >= (3, 25) {
        args.push(vec![k("SYSTEM")]);
    }
    vec![InputDirectory, optional(vec![OutputPath]), options(args)]
}

/// Describe build_command output variable and modern options or legacy make path.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// Out slot followed by options or a single generic path; PARALLEL_LEVEL is offered from 3.21.
///
/// # Examples
/// ```text
/// let args = build_command((3, 21)); // Describes a command string result, not its execution.
/// ```
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

/// Describe one or more literal filesystem permission names.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// None.
///
/// # Returns
/// A nonempty repetition of documented owner/group/world and special-bit keywords; no permissions
/// are changed.
///
/// # Examples
/// ```text
/// let mode = permissions(); // For FILE_PERMISSIONS OWNER_READ OWNER_WRITE.
/// ```
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

/// Describe configure_file source, destination and formatting/permission options.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// InputFile and OutputPath followed by repeated options; permission controls are version-filtered.
///
/// # Examples
/// ```text
/// let args = configure_file((3, 20)); // Includes FILE_PERMISSIONS.
/// ```
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

/// Describe visibility-scoped source lists and versioned file-set groups.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// Target plus one-or-more visibility groups with ordinary input files or named/typed file-set
/// contents.
///
/// # Algorithm
/// Start with ordinary InputFile contents. From 3.23 add FILE_SET groups; extend types with
/// CXX_MODULES in 3.28 and SOURCES in 4.4. BASE_DIRS are generic path hints, not required-file
/// checks.
///
/// # Flow
/// ```text
///                          [target_sources inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |     Ordinary source tokens     |    |   Versioned FILE_SET groups    |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = target_sources((3, 28)); // FILE_SET can use CXX_MODULES.
/// ```
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

/// Describe target/package export, setup, package metadata and software-bill-of-materials modes.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// One choice of eligible export forms; a software bill of materials (SBOM) is represented as
/// syntax only.
///
/// # Algorithm
/// Keep TARGETS/EXPORT/PACKAGE modes; add Android, dependency setup and metadata forms by
/// version. PACKAGE_INFO uses EXPORT first in 4.1/4.2, then PACKAGE_INFO first from 4.3; SBOM
/// also starts in 4.3.
///
/// # Flow
/// ```text
///                              [export inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |   TARGETS / EXPORT / PACKAGE   |    |  Versioned SETUP / metadata /  |
/// |                                |    |              SBOM              |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = export((4, 3)); // PACKAGE_INFO starts with the package name in this version.
/// ```
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

/// Describe package metadata while separating APPENDIX from basic package identity.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
/// - `installing` `[in]`: include installation common options and DESTINATION.
///
/// # Returns
/// Choice of an APPENDIX sequence with common options only, or ordinary common/basic metadata
/// options.
///
/// # Algorithm
/// Collect shared versioned fields, optionally extend installation fields, then isolate APPENDIX
/// so it cannot take VERSION/default-target metadata through the ordinary branch.
///
/// # Flow
/// ```text
///                           [package_info inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |    APPENDIX + common fields    |    |    Common + basic metadata     |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let metadata = package_info((4, 3), true); // Includes install DESTINATION and CXX_MODULES_DIRECTORY.
/// ```
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

/// Describe software-bill-of-materials metadata option groups.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `installing` `[in]`: include an output DESTINATION group for installation.
///
/// # Returns
/// Alternative metadata sequences for FORMAT, project identity, version/license and URLs, plus an
/// optional installation destination group.
///
/// # Examples
/// ```text
/// let opts = sbom_options(true); // For install(SBOM ... DESTINATION share).
/// ```
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

/// Collect permission, configuration and component groups shared by installation modes.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// Option alternatives, including EXCLUDE_FROM_ALL from 3.6; not a complete install signature.
///
/// # Examples
/// ```text
/// let common = install_common((3, 6)); // Adds EXCLUDE_FROM_ALL.
/// ```
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

/// Extend shared installation options with artifact destinations and optional namelink controls.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
/// - `namelink` `[in]`: include NAMELINK_ONLY/SKIP and, from 3.12, NAMELINK_COMPONENT.
///
/// # Returns
/// Option alternatives including DESTINATION and OPTIONAL, with requested namelink groups.
///
/// # Examples
/// ```text
/// let opts = artifact_options((3, 12), true); // Includes NAMELINK_COMPONENT.
/// ```
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

/// Describe repeated artifact-kind and optional named file-set installation groups.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
/// - `kinds` `[in]`: borrowed artifact keyword vocabulary, such as ARCHIVE and LIBRARY.
/// - `filesets` `[in]`: allow FILE_SET groups from 3.23.
/// - `namelink` `[in]`: pass namelink controls to ordinary artifact options; file sets always
///   disable them.
///
/// # Returns
/// Zero-or-more grouped artifact choices; FILE_SET consumes a name before its own options.
///
/// # Algorithm
/// Build ordinary kind/options groups and, when enabled and supported, a separate FILE_SET
/// name/options group. Wrap the alternatives in repetition to retain group boundaries.
///
/// # Flow
/// ```text
///                         [artifact_groups inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |    Kind + artifact options     |    |   Eligible FILE_SET + name +   |
/// |                                |    |            options             |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let groups = artifact_groups((3, 23), &["ARCHIVE", "LIBRARY"], true, true);
/// ```
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

/// Describe runtime-dependency include/exclude filters and directory hints.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// None.
///
/// # Returns
/// Option alternatives for regex lists, generic path lists and directories; these hints are not
/// required existing inputs.
///
/// # Examples
/// ```text
/// let filters = runtime_filters(); // For RUNTIME_DEPENDENCIES DIRECTORIES hint.
/// ```
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

/// Describe explicit destinations or version-supported standard destination types.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// Choice requiring DESTINATION OutputPath, or TYPE from 3.14; LIBEXEC joins the type vocabulary
/// in 3.31.
///
/// # Examples
/// ```text
/// let destination = install_destination((3, 31)); // Allows TYPE LIBEXEC.
/// ```
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

/// Describe installation modes without copying files or running embedded scripts.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// One choice spanning targets, files/programs, directories, script/code, exports and
/// version-eligible runtime/package/SBOM forms.
///
/// # Algorithm
/// Build mode-specific common/options groups, preserving required destinations where encoded. Add
/// object/module artifacts, file sets, runtime dependency forms, PACKAGE_INFO and SBOM by
/// version. Script/code operands are data only.
///
/// # Flow
/// ```text
///                             [install inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |  Targets + artifact subgroups  |    |Files / directories / scripts / |
/// |                                |    |            exports             |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = install((4, 3)); // Can describe install(PACKAGE_INFO ...).
/// ```
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

/// Collect source-probe options shared by try_compile and try_run.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
/// - `running` `[in]`: select run-result outputs and run-specific version thresholds.
/// - `modern` `[in]`: enable modern signature NO_CACHE/log options and separate run stdout/stderr
///   outputs.
///
/// # Returns
/// Option alternatives for compiler/linker settings and output variables; no compiler or
/// executable is invoked.
///
/// # Algorithm
/// Start with CMake flags, definitions and copy output; gate linking/language-standard groups by
/// version. Choose compile-only or run output names, then modern logging/cache controls.
///
/// # Flow
/// ```text
///                         [compile_options inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |     Compile/link settings      |    |   Compile-only / run outputs   |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let opts = compile_options((3, 29), true, true); // Includes LINKER_LANGUAGE and run outputs.
/// ```
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

/// Describe one or more modern source input/content/variable specifications.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// Nonempty repetition of optional SOURCES_TYPE followed by one source specification; type
/// selection exists from 3.28.
///
/// # Algorithm
/// Keep SOURCES, SOURCE_FROM_CONTENT, SOURCE_FROM_VAR and SOURCE_FROM_FILE as separate sequences.
/// Attach the optional normal/module type prefix per repeated source group.
///
/// # Flow
/// ```text
///                      [source_specifications inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |      Optional source type      |    | Sources / content / variable / |
/// |                                |    |              file              |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let sources = source_specifications((3, 28)); // SOURCE_FROM_VAR uses a Variable role.
/// ```
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

/// Describe legacy and modern try_compile signatures without performing a probe.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// Out result variable followed by source-file/project alternatives; modern forms start in 3.25.
///
/// # Algorithm
/// Keep legacy binary-directory/file and project sequences. From 3.25 append source-specification
/// and explicit PROJECT/SOURCE_DIR forms, with versioned log options.
///
/// # Flow
/// ```text
///                           [try_compile inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |   Legacy binary/source forms   |    | Modern sources / PROJECT forms |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = try_compile((3, 25)); // Describes ok PROJECT demo SOURCE_DIR src.
/// ```
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

/// Describe compile-and-run probe syntax without compiling or running anything.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `version` `[in]`: requested CMake major/minor; filters syntax additions, not runtime feature
///   activation.
///
/// # Returns
/// Run-result Out, compile-result Out, then a legacy or eligible modern source form, each with
/// optional ARGS.
///
/// # Algorithm
/// Retain legacy binary-directory/source mode; from 3.25 add source-specification mode with
/// modern run output controls. Both modes keep program ARGS as uninterpreted values.
///
/// # Flow
/// ```text
///                             [try_run inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// | Legacy binary-directory source |    |  Modern source specifications  |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = try_run((3, 25)); // First two slots are possible output variables.
/// ```
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

// Contains AI-generated test-module documentation edits.
/// Build grammar regression tests loaded from the package test tree.
#[cfg(test)]
#[path = "../../../../../tests/unit/parser/semanticer/keyword_build.rs"]
mod keyword_build;
