CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_LIBRARY_CUSTOM_LIB_SUFFIX.html)

# CMAKE_FIND_LIBRARY_CUSTOM_LIB_SUFFIX

Added in version 3.9.

Specify a `<suffix>` to tell the [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library) command to search in a `lib<suffix>` directory before each `lib` directory that would normally be searched.

This overrides the behavior of related global properties:

- [`FIND_LIBRARY_USE_LIB32_PATHS`](https://cmake.org/cmake/help/v4.4/prop_gbl/FIND_LIBRARY_USE_LIB32_PATHS.html#prop_gbl:FIND_LIBRARY_USE_LIB32_PATHS)

- [`FIND_LIBRARY_USE_LIB64_PATHS`](https://cmake.org/cmake/help/v4.4/prop_gbl/FIND_LIBRARY_USE_LIB64_PATHS.html#prop_gbl:FIND_LIBRARY_USE_LIB64_PATHS)

- [`FIND_LIBRARY_USE_LIBX32_PATHS`](https://cmake.org/cmake/help/v4.4/prop_gbl/FIND_LIBRARY_USE_LIBX32_PATHS.html#prop_gbl:FIND_LIBRARY_USE_LIBX32_PATHS)

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
