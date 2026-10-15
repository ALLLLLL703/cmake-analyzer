CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INCLUDE_CURRENT_DIR_IN_INTERFACE.html)

# CMAKE_INCLUDE_CURRENT_DIR_IN_INTERFACE

Automatically add the current source and build directories to the [`INTERFACE_INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_INCLUDE_DIRECTORIES.html#prop_tgt:INTERFACE_INCLUDE_DIRECTORIES) target property.

If this variable is enabled, CMake automatically adds for each shared library target, static library target, module target and executable target, [`CMAKE_CURRENT_SOURCE_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_SOURCE_DIR.html#variable:CMAKE_CURRENT_SOURCE_DIR) and [`CMAKE_CURRENT_BINARY_DIR`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_CURRENT_BINARY_DIR.html#variable:CMAKE_CURRENT_BINARY_DIR) to the [`INTERFACE_INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/prop_tgt/INTERFACE_INCLUDE_DIRECTORIES.html#prop_tgt:INTERFACE_INCLUDE_DIRECTORIES) target property. By default `CMAKE_INCLUDE_CURRENT_DIR_IN_INTERFACE` is `OFF`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
