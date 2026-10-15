CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_EXECUTABLE_ENABLE_EXPORTS.html)

# CMAKE_EXECUTABLE_ENABLE_EXPORTS

Added in version 3.27.

Specify whether executables export symbols for loadable modules.

This variable is used to initialize the [`ENABLE_EXPORTS`](https://cmake.org/cmake/help/v4.4/prop_tgt/ENABLE_EXPORTS.html#prop_tgt:ENABLE_EXPORTS) target property for executable targets when they are created by calls to the [`add_executable()`](https://cmake.org/cmake/help/v4.4/command/add_executable.html#command:add_executable) command. See the property documentation for details.

This variable supersedes the [`CMAKE_ENABLE_EXPORTS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_ENABLE_EXPORTS.html#variable:CMAKE_ENABLE_EXPORTS) variable.

## See Also

- The [`CMAKE_SHARED_LIBRARY_ENABLE_EXPORTS`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SHARED_LIBRARY_ENABLE_EXPORTS.html#variable:CMAKE_SHARED_LIBRARY_ENABLE_EXPORTS) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
