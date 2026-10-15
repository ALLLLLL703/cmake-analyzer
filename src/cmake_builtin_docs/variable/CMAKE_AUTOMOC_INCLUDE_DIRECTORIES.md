CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_AUTOMOC_INCLUDE_DIRECTORIES.html)

# CMAKE_AUTOMOC_INCLUDE_DIRECTORIES

Added in version 4.1.

Specifies zero or more include directories for AUTOMOC to pass explicitly to the Qt Meta-Object Compiler (`moc`) instead of automatically discovering each target's include directories.

The directories listed here will replace any include paths discovered from target properties such as [`INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INCLUDE_DIRECTORIES.html#prop_tgt:INCLUDE_DIRECTORIES).

This variable is used to initialize the [`AUTOMOC_INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/AUTOMOC_INCLUDE_DIRECTORIES.html#prop_tgt:AUTOMOC_INCLUDE_DIRECTORIES) property on all the targets. See that target property for additional information.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
