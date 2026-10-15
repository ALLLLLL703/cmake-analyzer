CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSROOT_LINK.html)

# CMAKE_SYSROOT_LINK

Added in version 3.9.

Path to pass to the compiler in the `--sysroot` flag when linking. This is the same as [`CMAKE_SYSROOT`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSROOT.html#variable:CMAKE_SYSROOT) but is used only for linking and not compiling sources.

This variable may only be set in a toolchain file specified by the [`CMAKE_TOOLCHAIN_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
