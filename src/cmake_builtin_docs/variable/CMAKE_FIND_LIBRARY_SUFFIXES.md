CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_LIBRARY_SUFFIXES.html)

# CMAKE_FIND_LIBRARY_SUFFIXES

Suffixes to append when looking for libraries.

This specifies what suffixes to add to library names when the [`find_library()`](https://cmake.org/cmake/help/v4.4/command/find_library.html#command:find_library) command looks for libraries. On Windows systems this is typically `.lib` and, depending on the compiler, `.dll.lib`, `.dll.a`, `.a` (e.g. rustc, GCC, or Clang), so when it tries to find the `foo` library, it will look for `[<prefix>]foo[.dll].lib` and/or `[<prefix>]foo[.dll].a`, depending on the compiler used and the `<prefix>` specified in the [`CMAKE_FIND_LIBRARY_PREFIXES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_LIBRARY_PREFIXES.html#variable:CMAKE_FIND_LIBRARY_PREFIXES).

CMake sets this variable automatically during toolchain inspection by calls to the [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) commands.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
