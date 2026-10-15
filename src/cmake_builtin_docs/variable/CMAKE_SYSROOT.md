CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSROOT.html)

# CMAKE_SYSROOT

Path to pass to the compiler in the `--sysroot` flag.

The `CMAKE_SYSROOT` content is passed to the compiler in the `--sysroot` flag, if supported. The path is also stripped from the `RPATH`/`RUNPATH` if necessary on installation. The `CMAKE_SYSROOT` is also used to prefix paths searched by the `find_*` commands.

This variable may only be set in a toolchain file specified by the [`CMAKE_TOOLCHAIN_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE) variable.

See also the [`CMAKE_SYSROOT_COMPILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSROOT_COMPILE.html#variable:CMAKE_SYSROOT_COMPILE) and [`CMAKE_SYSROOT_LINK`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_SYSROOT_LINK.html#variable:CMAKE_SYSROOT_LINK) variables.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
