CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_INCLUDE_PATH.html)

# CMAKE_SYSTEM_INCLUDE_PATH

[Semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) of directories specifying a search path for the [`find_file()`](https://cmake.org/cmake/help/v4.4/command/find_file.html#command:find_file) and [`find_path()`](https://cmake.org/cmake/help/v4.4/command/find_path.html#command:find_path) commands. By default this contains the standard directories for the current system. It is *not* intended to be modified by the project; use [`CMAKE_INCLUDE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INCLUDE_PATH.html#variable:CMAKE_INCLUDE_PATH) for this. See also [`CMAKE_SYSTEM_PREFIX_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_PREFIX_PATH.html#variable:CMAKE_SYSTEM_PREFIX_PATH).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
