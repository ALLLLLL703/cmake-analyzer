CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INTERMEDIATE_DIR_STRATEGY.html)

# CMAKE_INTERMEDIATE_DIR_STRATEGY

Added in version 4.2.

`CMAKE_INTERMEDIATE_DIR_STRATEGY` is a string cache variable specifying the strategy to use for target intermediate directories and their contents. The supported values are:

- `FULL`: Intermediate directories are named based on a `<TARGET_NAME>.dir` pattern (with some slight deviations and sanitizations applied in various places). Object file names are based on the filename of the source file being compiled or, if set, its [`OBJECT_NAME`](https://cmake.org/cmake/help/v4.4/prop_sf/OBJECT_NAME.html#prop_sf:OBJECT_NAME) property.

- `SHORT`: Intermediate directories are named from the hash of the target name and the build directory location. Object file names are based on hashes of the source file name to reduce path lengths. This may help with projects that generate long paths in the build directory to support building in directories other than those near a root path.

When unset or the named strategy is not supported, the `FULL` strategy is used.

Note

This only works as a cache variable, not a locally-scoped variable.

This is supported by the following generators:

- [Ninja Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#ninja-generators)

- [Makefile Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#makefile-generators)

- [Visual Studio Generators](https://cmake.org/cmake/help/v4.4/manual/cmake-generators.7.html#visual-studio-generators)

Note

Not all generators support all strategies and paths may differ between generators.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
