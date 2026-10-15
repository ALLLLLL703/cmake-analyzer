CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/PROJECT_SOURCE_DIR.html)

# PROJECT_SOURCE_DIR

This is the source directory of the last call to the [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) command made in the current directory scope or one of its parents. Note, it is not affected by calls to [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) made within a child directory scope (i.e. from within a call to [`add_subdirectory()`](https://cmake.org/cmake/help/v4.4/command/add_subdirectory.html#command:add_subdirectory) from the current scope).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
