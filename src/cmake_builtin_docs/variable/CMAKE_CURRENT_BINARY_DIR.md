CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_BINARY_DIR.html)

# CMAKE_CURRENT_BINARY_DIR

The path to the binary directory currently being processed.

This is the full path to the build directory that is currently being processed by cmake. Each directory added by [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) will create a binary directory in the build tree, and as it is being processed this variable will be set. For in-source builds this is the current source directory being processed.

When run in [`cmake -P`](https://cmake.org/cmake/help/v4.4/manual/cmake.1.html#cmdoption-cmake-P) script mode, CMake sets the variables [`CMAKE_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_BINARY_DIR.html#variable:CMAKE_BINARY_DIR), [`CMAKE_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SOURCE_DIR.html#variable:CMAKE_SOURCE_DIR), `CMAKE_CURRENT_BINARY_DIR` and [`CMAKE_CURRENT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_SOURCE_DIR.html#variable:CMAKE_CURRENT_SOURCE_DIR) to the current working directory.

Modifying `CMAKE_CURRENT_BINARY_DIR` has undefined behavior.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
