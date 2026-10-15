CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_DEPENDS_IN_PROJECT_ONLY.html)

# CMAKE_DEPENDS_IN_PROJECT_ONLY

Added in version 3.6.

When set to `TRUE` in a directory, the build system produced by the [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators) is set up to only consider dependencies on source files that appear either in the source or in the binary directories. Changes to source files outside of these directories will not cause rebuilds.

This should be used carefully in cases where some source files are picked up through external headers during the build.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
