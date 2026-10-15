CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_IGNORE_PATH.html)

# CMAKE_SYSTEM_IGNORE_PATH

[Semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of directories to be ignored by the various `find...()` commands.

For [`find_program()`](https://cmake.org/cmake/help/v4.4/command/find_program.html#command:find_program), [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library), [`find_file()`](https://cmake.org/cmake/help/v4.4/command/find_file.html#command:find_file), and [`find_path()`](https://cmake.org/cmake/help/v4.4/command/find_path.html#command:find_path), any file found in one of the listed directories will be ignored. The listed directories do not apply recursively, so any subdirectories to be ignored must also be explicitly listed. `CMAKE_SYSTEM_IGNORE_PATH` does not affect the search *prefixes* used by these four commands. To ignore individual paths under a search prefix (e.g. `bin`, `include`, `lib`, etc.), each path must be listed in `CMAKE_SYSTEM_IGNORE_PATH` as a full absolute path. [`CMAKE_SYSTEM_IGNORE_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_IGNORE_PREFIX_PATH.html#variable:CMAKE_SYSTEM_IGNORE_PREFIX_PATH) provides a more appropriate way to ignore a whole search prefix.

[`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) is also affected by `CMAKE_SYSTEM_IGNORE_PATH`, but only for *Config mode* searches. Any `<Name>Config.cmake` or `<name>-config.cmake` file found in one of the specified directories will be ignored. In addition, any search *prefix* found in `CMAKE_SYSTEM_IGNORE_PATH` will be skipped for backward compatibility reasons, but new code should prefer to use [`CMAKE_SYSTEM_IGNORE_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_IGNORE_PREFIX_PATH.html#variable:CMAKE_SYSTEM_IGNORE_PREFIX_PATH) to ignore prefixes instead.

Ignoring search locations can be useful in cross-compiling environments where some system directories contain incompatible but possibly linkable libraries. For example, on cross-compiled cluster environments, this allows a user to ignore directories containing libraries meant for the front-end machine.

`CMAKE_SYSTEM_IGNORE_PATH` is populated by CMake as part of its platform and toolchain setup. Its purpose is to ignore locations containing incompatible binaries meant for the host rather than the target platform. The project or end user should not modify this variable, they should use [`CMAKE_IGNORE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_IGNORE_PATH.html#variable:CMAKE_IGNORE_PATH) instead.

See also the following variables:

- [`CMAKE_SYSTEM_IGNORE_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_IGNORE_PREFIX_PATH.html#variable:CMAKE_SYSTEM_IGNORE_PREFIX_PATH)

- [`CMAKE_SYSTEM_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PREFIX_PATH.html#variable:CMAKE_SYSTEM_PREFIX_PATH)

- [`CMAKE_SYSTEM_LIBRARY_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_LIBRARY_PATH.html#variable:CMAKE_SYSTEM_LIBRARY_PATH)

- [`CMAKE_SYSTEM_INCLUDE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_INCLUDE_PATH.html#variable:CMAKE_SYSTEM_INCLUDE_PATH)

- [`CMAKE_SYSTEM_PROGRAM_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PROGRAM_PATH.html#variable:CMAKE_SYSTEM_PROGRAM_PATH)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
