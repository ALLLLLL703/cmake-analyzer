CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_TARGET.html)

# CMAKE_\<LANG\>_COMPILER_TARGET

The target for cross-compiling, if supported.

Some compiler drivers are inherently cross-compilers, such as clang and QNX qcc. These compiler drivers support a command-line argument to specify the target to cross-compile for.

This variable may only be set in a toolchain file specified by the [`CMAKE_TOOLCHAIN_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
