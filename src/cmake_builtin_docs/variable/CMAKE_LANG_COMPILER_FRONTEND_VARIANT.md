CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_FRONTEND_VARIANT.html)

# CMAKE_\<LANG\>_COMPILER_FRONTEND_VARIANT

Added in version 3.14.

Identification string of the compiler frontend variant.

Some compilers have multiple, different frontends for accepting command line options. (For example `Clang` originally only had a frontend compatible with the `GNU` compiler but since its port to Windows (`Clang-Cl`) it now also supports a frontend compatible with `MSVC`.) When CMake detects such a compiler it sets this variable to what would have been the [`CMAKE_<LANG>_COMPILER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_ID) for the compiler whose frontend it resembles.

Note

In other words, this variable describes what command line options and language extensions the compiler frontend expects.

Changed in version 3.26: This variable is set for `GNU`, `MSVC`, and `AppleClang` compilers that have only one frontend variant.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
