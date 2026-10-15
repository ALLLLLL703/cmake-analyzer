CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_PARENT_LIST_FILE.html)

# CMAKE_PARENT_LIST_FILE

Full path to the CMake file that included the current one.

While processing a CMake file loaded by [`include()`](https://cmake.org/cmake/help/v4.4/command/include.html#command:include) or [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) this variable contains the full path to the file including it.

While processing a `CMakeLists.txt` file, even in subdirectories, this variable is not defined. See policy [`CMP0198`](https://cmake.org/cmake/help/v4.4/policy/CMP0198.html#policy:CMP0198).

While processing a [`cmake -P`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-P) script, this variable is not defined in the outermost script.

See also [`CMAKE_CURRENT_LIST_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_LIST_FILE.html#variable:CMAKE_CURRENT_LIST_FILE).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
