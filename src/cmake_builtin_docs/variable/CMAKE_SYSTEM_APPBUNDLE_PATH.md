CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSTEM_APPBUNDLE_PATH.html)

# CMAKE_SYSTEM_APPBUNDLE_PATH

Added in version 3.4.

Search path for macOS application bundles used by the [`find_program()`](https://cmake.org/cmake/help/v4.4/command/find_program.html#command:find_program), and [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) commands. By default it contains the standard directories for the current system. It is *not* intended to be modified by the project, use [`CMAKE_APPBUNDLE_PATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_APPBUNDLE_PATH.html#variable:CMAKE_APPBUNDLE_PATH) for this.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
