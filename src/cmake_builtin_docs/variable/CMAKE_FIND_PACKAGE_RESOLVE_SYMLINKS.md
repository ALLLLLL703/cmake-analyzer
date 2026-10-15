CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_FIND_PACKAGE_RESOLVE_SYMLINKS.html)

# CMAKE_FIND_PACKAGE_RESOLVE_SYMLINKS

Added in version 3.14.

Set to `TRUE` to tell [`find_package()`](https://cmake.org/cmake/help/v4.4/command/find_package.html#command:find_package) calls to resolve symbolic links in the value of `<PackageName>_DIR`.

This is helpful in use cases where the package search path points at a proxy directory in which symlinks to the real package locations appear. This is not enabled by default because there are also common use cases in which the symlinks should be preserved.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
