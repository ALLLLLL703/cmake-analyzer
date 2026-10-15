CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_BUILD_RPATH.html)

# CMAKE_BUILD_RPATH

Added in version 3.8.

[Semicolon-separated list](https://cmake.org/cmake/help/v4.4/manual/cmake-language.7.html#cmake-language-lists) specifying runtime path (`RPATH`) entries to add to binaries linked in the build tree (for platforms that support it). The entries will *not* be used for binaries in the install tree. See also the [`CMAKE_INSTALL_RPATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_INSTALL_RPATH.html#variable:CMAKE_INSTALL_RPATH) variable.

This is used to initialize the [`BUILD_RPATH`](https://cmake.org/cmake/help/v4.4/prop_tgt/BUILD_RPATH.html#prop_tgt:BUILD_RPATH) target property for all targets.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
