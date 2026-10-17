//! Shared file/library/path/program search signatures.
//! Trees expose output-variable roles and explicit search options, not search
//! results; construction never probes a path, reads an environment variable,
//! invokes a validator or executes CMake. For `find_library(lib NAMES z PATHS
//! hint)`, `lib` is an output name and `hint` is a generic path, not an input
//! that must exist. The cache caller filters these overrides by native registration.

use super::super::*;

/// Collect the four native find-file/library/path/program overrides.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `v` `[in]`: requested CMake major/minor controlling syntax additions.
///
/// # Returns
/// Four owned (command name, roots) pairs using the shared command-sensitive builder.
///
/// # Examples
/// ```text
/// let overrides = schemas((3, 25));
/// assert_eq!(overrides.len(), 4);
/// ```
pub(super) fn schemas(v: (u32, u32)) -> Vec<(&'static str, Vec<ArgSchemaKind>)> {
    ["find_file", "find_library", "find_path", "find_program"]
        .into_iter()
        .map(|name| (name, args(v, name)))
        .collect()
}
/// Describe one find command with search hints, literal flags and a result variable.
/// Allocates grammar data only; no external side effects.
///
/// # Arguments
/// - `v` `[in]`: requested CMake major/minor controlling syntax additions.
/// - `command` `[in]`: native command spelling; only find_library/find_program enable NAMES_PER_DIR.
///
/// # Returns
/// Out followed by repeated option groups, including an open text branch for shorthand names/paths.
///
/// # Algorithm
/// Create generic path or ENV/name hint lists; add common flags and versioned
/// required/cache/registry/validator options. Keep NAMES_PER_DIR command-specific and an open
/// text alternative for legacy shorthand.
///
/// # Flow
/// ```text
///                               [args inputs]
///                                     |
///                  +------------------+------------------+
///                  v                                     v
/// +--------------------------------+    +--------------------------------+
/// |  NAMES / HINTS / PATHS groups  |    |  Flags / versioned controls /  |
/// |                                |    |           shorthand            |
/// +----------------+---------------+    +----------------+---------------+
///                  |                                     |
///                  +------------------+------------------+
///                                     v
///                              [Owned grammar]
/// ```
///
/// # Examples
/// ```text
/// let args = args((3, 25), "find_library"); // VALIDATOR is a Command slot, not invoked.
/// ```
fn args(v: (u32, u32), command: &str) -> Vec<ArgSchemaKind> {
    let paths = many(choice(vec![vec![k("ENV"), Text], vec![Path]]));
    let mut opts = vec![
        vec![k("NAMES"), many(Text)],
        vec![k("HINTS"), paths.clone()],
        vec![k("PATHS"), paths],
        vec![k("PATH_SUFFIXES"), many(Text)],
        vec![k("DOC"), Text],
    ];
    for flag in [
        "NO_DEFAULT_PATH",
        "NO_CMAKE_PATH",
        "NO_CMAKE_ENVIRONMENT_PATH",
        "NO_SYSTEM_ENVIRONMENT_PATH",
        "NO_CMAKE_SYSTEM_PATH",
        "CMAKE_FIND_ROOT_PATH_BOTH",
        "ONLY_CMAKE_FIND_ROOT_PATH",
        "NO_CMAKE_FIND_ROOT_PATH",
    ] {
        opts.push(vec![k(flag)]);
    }
    if matches!(command, "find_library" | "find_program") {
        opts.push(vec![k("NAMES_PER_DIR")]);
    }
    if v >= (3, 12) {
        opts.push(vec![k("NO_PACKAGE_ROOT_PATH")]);
    }
    if v >= (3, 18) {
        opts.push(vec![k("REQUIRED")]);
    }
    if v >= (3, 21) {
        opts.push(vec![k("NO_CACHE")]);
    }
    if v >= (3, 24) {
        opts.extend([
            vec![k("NO_CMAKE_INSTALL_PREFIX")],
            vec![
                k("REGISTRY_VIEW"),
                one_of(&["64", "32", "64_32", "32_64", "HOST", "TARGET", "BOTH"]),
            ],
        ]);
    }
    if v >= (3, 25) {
        opts.push(vec![k("VALIDATOR"), Command]);
    }
    if v >= (4, 1) {
        opts.push(vec![k("OPTIONAL")]);
    }
    // Both shorthand names/paths and keyword groups are valid; search paths need not exist.
    opts.push(vec![Text]);
    vec![Out, options(opts)]
}
