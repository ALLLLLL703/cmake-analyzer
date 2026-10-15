CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_STANDARD_LINK_DIRECTORIES.html)

# CMAKE_\<LANG\>_STANDARD_LINK_DIRECTORIES

Added in version 3.31.

Link directories specified for every executable and library linked for language `<LANG>`. This is meant for specification of system link directories needed by the language for the current platform.

This variable should not be set by project code. It is meant to be set by CMake's platform information modules for the current toolchain, or by a toolchain file when used with [`CMAKE_TOOLCHAIN_FILE`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_TOOLCHAIN_FILE.html#variable:CMAKE_TOOLCHAIN_FILE).

See also [`CMAKE_<LANG>_STANDARD_LIBRARIES`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_STANDARD_LIBRARIES.html#variable:CMAKE_%3CLANG%3E_STANDARD_LIBRARIES).

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
