CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SOURCE_DIR.html)

# CMAKE_SOURCE_DIR

The path to the top level of the source tree.

This is the full path to the top level of the current CMake source tree. For an in-source build, this would be the same as [`CMAKE_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_BINARY_DIR.html#variable:CMAKE_BINARY_DIR).

When run in [`cmake -P`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-P) script mode, CMake sets the variables [`CMAKE_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_BINARY_DIR.html#variable:CMAKE_BINARY_DIR), `CMAKE_SOURCE_DIR`, [`CMAKE_CURRENT_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_BINARY_DIR.html#variable:CMAKE_CURRENT_BINARY_DIR) and [`CMAKE_CURRENT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_SOURCE_DIR.html#variable:CMAKE_CURRENT_SOURCE_DIR) to the current working directory.

Modifying `CMAKE_SOURCE_DIR` has undefined behavior.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
