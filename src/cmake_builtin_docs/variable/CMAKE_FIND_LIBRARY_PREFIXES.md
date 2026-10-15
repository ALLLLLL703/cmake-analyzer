CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_LIBRARY_PREFIXES.html)

# CMAKE_FIND_LIBRARY_PREFIXES

Prefixes to prepend when looking for libraries.

This specifies what prefixes to add to library names when the [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library) command looks for libraries. On UNIX systems this is typically `lib`, meaning that when trying to find the `foo` library it will look for `libfoo`.

CMake sets this variable automatically during toolchain inspection by calls to the [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) commands.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
