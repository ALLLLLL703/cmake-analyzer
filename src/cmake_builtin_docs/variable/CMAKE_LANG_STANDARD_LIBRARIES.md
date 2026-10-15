CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_STANDARD_LIBRARIES.html)

# CMAKE_\<LANG\>_STANDARD_LIBRARIES

Added in version 3.6.

Libraries linked into every executable and shared library linked for language `<LANG>`. This is meant for specification of system libraries needed by the language for the current platform.

This variable should not be set by project code. It is meant to be set by CMake's platform information modules for the current toolchain, or by a toolchain file when used with [`CMAKE_TOOLCHAIN_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE).

See also [`CMAKE_<LANG>_STANDARD_INCLUDE_DIRECTORIES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_STANDARD_INCLUDE_DIRECTORIES.html#variable:CMAKE_%3CLANG%3E_STANDARD_INCLUDE_DIRECTORIES).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
