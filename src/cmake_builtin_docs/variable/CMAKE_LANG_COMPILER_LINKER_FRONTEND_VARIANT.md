CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_LINKER_FRONTEND_VARIANT.html)

# CMAKE_\<LANG\>_COMPILER_LINKER_FRONTEND_VARIANT

Added in version 3.29.

Identification string of the linker frontend variant.

Some linkers have multiple, different frontends for accepting command line options. For example, `LLVM LLD` originally only had a frontend compatible with the `GNU` compiler, but since its port to Windows (`lld-link`), it now also supports a frontend compatible with `MSVC`. When CMake detects such a linker, it sets this variable to what would have been the [`CMAKE_<LANG>_COMPILER_LINKER_ID`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_COMPILER_LINKER_ID.html#variable:CMAKE_%3CLANG%3E_COMPILER_LINKER_ID) for the linker whose frontend it resembles.

Note

In other words, this variable describes what command line options and language extensions the linker frontend expects.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
