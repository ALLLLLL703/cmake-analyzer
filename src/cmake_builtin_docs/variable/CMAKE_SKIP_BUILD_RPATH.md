CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SKIP_BUILD_RPATH.html)

# CMAKE_SKIP_BUILD_RPATH

Do not include RPATHs in the build tree.

Normally CMake uses the build tree for the RPATH when building executables etc on systems that use RPATH. When the software is installed the executables etc are relinked by CMake to have the install RPATH. If this variable is set to `TRUE` then the software is always built with no RPATH.

This is used to initialize the [`SKIP_BUILD_RPATH`](https://cmake.org/cmake/help/v4.4/prop_tgt/SKIP_BUILD_RPATH.html#prop_tgt:SKIP_BUILD_RPATH) target property for all targets. For more information on RPATH handling see the [`INSTALL_RPATH`](https://cmake.org/cmake/help/v4.4/prop_tgt/INSTALL_RPATH.html#prop_tgt:INSTALL_RPATH) and [`BUILD_RPATH`](https://cmake.org/cmake/help/v4.4/prop_tgt/BUILD_RPATH.html#prop_tgt:BUILD_RPATH) target properties.

See also the [`CMAKE_SKIP_INSTALL_RPATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SKIP_INSTALL_RPATH.html#variable:CMAKE_SKIP_INSTALL_RPATH) variable. To omit RPATH in both the build and install steps, use [`CMAKE_SKIP_RPATH`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SKIP_RPATH.html#variable:CMAKE_SKIP_RPATH) instead.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
