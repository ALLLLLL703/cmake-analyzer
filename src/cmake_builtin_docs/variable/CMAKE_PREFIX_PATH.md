CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PREFIX_PATH.html)

# CMAKE_PREFIX_PATH

[Semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of directories specifying installation *prefixes* to be searched by the [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package), [`find_program()`](https://cmake.org/cmake/help/v4.4/command/find_program.html#command:find_program), [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library), [`find_file()`](https://cmake.org/cmake/help/v4.4/command/find_file.html#command:find_file), and [`find_path()`](https://cmake.org/cmake/help/v4.4/command/find_path.html#command:find_path) commands. Each command will add appropriate subdirectories (like `bin`, `lib`, or `include`) as specified in its own documentation.

By default this is empty. It is intended to be set by the project.

There is also an environment variable [`CMAKE_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/envvar/CMAKE_PREFIX_PATH.html#envvar:CMAKE_PREFIX_PATH), which is used as an additional list of search prefixes.

See also [`CMAKE_SYSTEM_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PREFIX_PATH.html#variable:CMAKE_SYSTEM_PREFIX_PATH), [`CMAKE_INCLUDE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INCLUDE_PATH.html#variable:CMAKE_INCLUDE_PATH), [`CMAKE_LIBRARY_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LIBRARY_PATH.html#variable:CMAKE_LIBRARY_PATH), [`CMAKE_PROGRAM_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PROGRAM_PATH.html#variable:CMAKE_PROGRAM_PATH), and [`CMAKE_IGNORE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_IGNORE_PATH.html#variable:CMAKE_IGNORE_PATH).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
