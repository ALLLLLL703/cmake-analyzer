> AI-generated content.

# Static package sources

find_package is a dependency, alongside include and add_subdirectory. It selects
source entrypoints, not an executed package result. No CMake configure, package
provider, registry script, download, compiler, or version-check script is run.

- `request.rs` reads literal MODULE/CONFIG/NO_MODULE, NAMES, CONFIGS, HINTS, PATHS,
  PATH_SUFFIXES, and default-path switches. Unknown dynamic operands and root-path/
  registry-view policies cannot be resolved statically.
- `model.rs` holds the parsed request without protocol/serialization envelopes.
- `candidates.rs` checks exact directories first, then conventional package
  directories under a prefix, lib/cmake, lib64/cmake, share, share/cmake, and
  lib/<architecture>/cmake. Directory enumeration has fixed conventional depth,
  deterministic ordering, and no recursive source-tree scan.
- `../packages.rs` supplies known <Package>_DIR/<Package>_ROOT, CMAKE_PREFIX_PATH,
  matching environment paths, and configured package prefixes. HINTS/PATHS can
  explicitly supply directories; NO_DEFAULT_PATH excludes default prefix sources.

Basic signatures try Find<Package>.cmake through module search first, then config
files; CMAKE_FIND_PACKAGE_PREFER_CONFIG reverses this. MODULE and CONFIG/NO_MODULE
restrict the modes. Config candidates use <Name>Config.cmake or <lower-name>-config.cmake,
with explicit CONFIGS filenames supported. Static simple argument expansions use
positioned graph values and current list/source directory context.

This is source discovery, not a complete CMake find algorithm. Version/EXACT,
component availability, target architecture, toolchain/sysroot policies, package
registries/providers, CPS files, and platform-specific layouts are not evaluated.
When several conventional installations are present, the first static candidate
is used; it is not proof that a real CMake invocation would choose that package.
Installed runtime observations remain supplemental navigation evidence.

Required unresolved packages produce dependency warnings; optional packages keep
unresolved edges without a missing-package warning. Existing Workspace traversal
limits and revision-qualified snapshots still apply.

## External imports

Project-external include and package entrypoints independently select full,
functions, or none. Defaults are full. Canonical saved paths determine whether a
file is outside the entry directory; external add_subdirectory sources retain
ordinary project analysis.

- full: existing conservative static dependency, function, variable, value, and
  target analysis; conditional/deferred code is not executed.
- functions: load source declarations and statically resolved dependency files,
  importing only unconditional top-level functions/macros in source order.
  Variable writes, values, targets, and body references are not exported. Child
  imports remain functions-only. Dependencies needing private external assignments
  may stay unresolved; current-list-directory literal expansions still work.
- none: retain the resolved file edge for navigation but do not load its symbols.

Opening an external file explicitly still permits local editor analysis. The
modes govern automatic static imports, not authorization to run project code.
No FetchContent/ExternalProject checkout discovery or download is added.
