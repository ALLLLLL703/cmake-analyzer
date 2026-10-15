CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_IGNORE_PREFIX_PATH.html)

# CMAKE_SYSTEM_IGNORE_PREFIX_PATH

Added in version 3.23.

[Semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of search *prefixes* to be ignored by the [`find_program()`](https://cmake.org/cmake/help/v4.4/command/find_program.html#command:find_program), [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library), [`find_file()`](https://cmake.org/cmake/help/v4.4/command/find_file.html#command:find_file), and [`find_path()`](https://cmake.org/cmake/help/v4.4/command/find_path.html#command:find_path) commands. The prefixes are also ignored by the *Config mode* of the [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) command (*Module mode* is unaffected). To ignore specific directories instead, see [`CMAKE_SYSTEM_IGNORE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_IGNORE_PATH.html#variable:CMAKE_SYSTEM_IGNORE_PATH).

Ignoring search locations can be useful in cross-compiling environments where some system directories contain incompatible but possibly linkable libraries. For example, on cross-compiled cluster environments, this allows a user to ignore directories containing libraries meant for the front-end machine.

`CMAKE_SYSTEM_IGNORE_PREFIX_PATH` is populated by CMake as part of its platform and toolchain setup. Its purpose is to ignore locations containing incompatible binaries meant for the host rather than the target platform. The project or end user should not modify this variable, they should use [`CMAKE_IGNORE_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_IGNORE_PREFIX_PATH.html#variable:CMAKE_IGNORE_PREFIX_PATH) instead.

See also the following variables:

- [`CMAKE_SYSTEM_IGNORE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_IGNORE_PATH.html#variable:CMAKE_SYSTEM_IGNORE_PATH)

- [`CMAKE_SYSTEM_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PREFIX_PATH.html#variable:CMAKE_SYSTEM_PREFIX_PATH)

- [`CMAKE_SYSTEM_LIBRARY_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_LIBRARY_PATH.html#variable:CMAKE_SYSTEM_LIBRARY_PATH)

- [`CMAKE_SYSTEM_INCLUDE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_INCLUDE_PATH.html#variable:CMAKE_SYSTEM_INCLUDE_PATH)

- [`CMAKE_SYSTEM_PROGRAM_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PROGRAM_PATH.html#variable:CMAKE_SYSTEM_PROGRAM_PATH)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
