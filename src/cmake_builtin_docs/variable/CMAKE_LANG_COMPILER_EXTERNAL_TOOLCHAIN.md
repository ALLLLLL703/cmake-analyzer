CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_EXTERNAL_TOOLCHAIN.html)

# CMAKE_\<LANG\>_COMPILER_EXTERNAL_TOOLCHAIN

The external toolchain for cross-compiling, if supported.

Some compiler toolchains do not ship their own auxiliary utilities such as archivers and linkers. The compiler driver may support a command-line argument to specify the location of such tools. `CMAKE_<LANG>_COMPILER_EXTERNAL_TOOLCHAIN` may be set to a path to the external toolchain and will be passed to the compiler driver if supported.

This variable may only be set in a toolchain file specified by the [`CMAKE_TOOLCHAIN_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
