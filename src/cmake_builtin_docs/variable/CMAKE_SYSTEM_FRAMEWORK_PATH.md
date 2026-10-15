CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_FRAMEWORK_PATH.html)

# CMAKE_SYSTEM_FRAMEWORK_PATH

Added in version 3.4.

Search path for macOS frameworks used by the [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library), [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package), [`find_path()`](https://cmake.org/cmake/help/v4.4/command/find_path.html#command:find_path), and [`find_file()`](https://cmake.org/cmake/help/v4.4/command/find_file.html#command:find_file) commands. By default it contains the standard directories for the current system. It is *not* intended to be modified by the project, use [`CMAKE_FRAMEWORK_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FRAMEWORK_PATH.html#variable:CMAKE_FRAMEWORK_PATH) for this.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
