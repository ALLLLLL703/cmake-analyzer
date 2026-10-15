CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_BUILD_WITH_INSTALL_RPATH.html)

# CMAKE_BUILD_WITH_INSTALL_RPATH

Use the install path for the `RPATH`.

Normally CMake uses the build tree for the `RPATH` when building executables etc on systems that use `RPATH`. When the software is installed the executables etc are relinked by CMake to have the install `RPATH`. If this variable is set to true then the software is always built with the install path for the `RPATH` and does not need to be relinked when installed.

This is used to initialize the [`BUILD_WITH_INSTALL_RPATH`](https://cmake.org/cmake/help/v4.4/prop_tgt/BUILD_WITH_INSTALL_RPATH.html#prop_tgt:BUILD_WITH_INSTALL_RPATH) target property for all targets.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
