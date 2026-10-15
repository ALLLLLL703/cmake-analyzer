CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_Swift_SEPARATE_MODULE_EMISSION.html)

# CMAKE_Swift_SEPARATE_MODULE_EMISSION

Added in version 4.4.

Specify whether Swift targets emit `.swiftmodule` from a dedicated build edge. This variable is used to initialize the [`Swift_SEPARATE_MODULE_EMISSION`](https://cmake.org/cmake/help/v4.4/prop_tgt/Swift_SEPARATE_MODULE_EMISSION.html#prop_tgt:Swift_SEPARATE_MODULE_EMISSION) property on targets as they are created.

When set to `ON`, importable Swift targets built by [Ninja Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#ninja-generators) produce the `.swiftmodule` from a separate `-emit-module` build edge, allowing downstream Swift targets to compile in parallel with upstream object compilation.

If this variable is not set, separate module emission is enabled by default in targets that do not set [`Swift_SEPARATE_MODULE_EMISSION`](https://cmake.org/cmake/help/v4.4/prop_tgt/Swift_SEPARATE_MODULE_EMISSION.html#prop_tgt:Swift_SEPARATE_MODULE_EMISSION). See policy [`CMP0215`](https://cmake.org/cmake/help/v4.4/policy/CMP0215.html#policy:CMP0215).

Note

Separate module emission takes effect only when policy [`CMP0157`](https://cmake.org/cmake/help/v4.4/policy/CMP0157.html#policy:CMP0157) is set to `NEW` prior to the first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) command that enables the Swift language.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
