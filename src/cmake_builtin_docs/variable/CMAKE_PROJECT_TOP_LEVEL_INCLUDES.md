CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_TOP_LEVEL_INCLUDES.html)

# CMAKE_PROJECT_TOP_LEVEL_INCLUDES

Added in version 3.24.

[Semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of CMake language files to include as part of the very first [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) call. The files will be included immediately after the toolchain file has been read (if one is specified) and platform variables have been set, but before any languages have been enabled. Therefore, language-specific variables, including things like [`CMAKE_<LANG>_COMPILER`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER.html#variable:CMAKE_%3CLANG%3E_COMPILER), might not be set. See [Code Injection](https://cmake.org/cmake/help/v4.4/command/project.html#code-injection) for a more detailed discussion of files potentially included during a [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) call.

Added in version 3.29: This variable can also now refer to module names to be found in [`CMAKE_MODULE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_MODULE_PATH.html#variable:CMAKE_MODULE_PATH) or builtin to CMake.

This variable is intended for specifying files that perform one-time setup for the build. It provides an injection point for things like configuring package managers, adding logic the user shares between projects (e.g. defining their own custom build types), and so on. It is primarily for users to add things specific to their environment, but not for specifying the toolchain details (use [`CMAKE_TOOLCHAIN_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE) for that).

By default, this variable is empty. It is intended to be set by the user.

See also:

- [`CMAKE_PROJECT_INCLUDE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_INCLUDE.html#variable:CMAKE_PROJECT_INCLUDE)

- [`CMAKE_PROJECT_INCLUDE_BEFORE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_INCLUDE_BEFORE.html#variable:CMAKE_PROJECT_INCLUDE_BEFORE)

- [`CMAKE_PROJECT_<PROJECT-NAME>_INCLUDE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_PROJECT-NAME_INCLUDE.html#variable:CMAKE_PROJECT_%3CPROJECT-NAME%3E_INCLUDE)

- [`CMAKE_PROJECT_<PROJECT-NAME>_INCLUDE_BEFORE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROJECT_PROJECT-NAME_INCLUDE_BEFORE.html#variable:CMAKE_PROJECT_%3CPROJECT-NAME%3E_INCLUDE_BEFORE)

- [`PROPAGATE_TOP_LEVEL_INCLUDES_TO_TRY_COMPILE`](https://cmake.org/cmake/help/v4.4/prop_gbl/PROPAGATE_TOP_LEVEL_INCLUDES_TO_TRY_COMPILE.html#prop_gbl:PROPAGATE_TOP_LEVEL_INCLUDES_TO_TRY_COMPILE)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
