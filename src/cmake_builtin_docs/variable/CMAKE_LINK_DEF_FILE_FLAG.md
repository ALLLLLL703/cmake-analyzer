CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_DEF_FILE_FLAG.html)

# CMAKE_LINK_DEF_FILE_FLAG

Linker flag to be used to specify a `.def` file for dll creation.

The flag will be used to add a `.def` file when creating a dll on Windows; this is only defined on Windows.

CMake sets this variable automatically during toolchain inspection by calls to the [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) commands.

If the per-language [`CMAKE_<LANG>_LINK_DEF_FILE_FLAG`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_DEF_FILE_FLAG.html#variable:CMAKE_%3CLANG%3E_LINK_DEF_FILE_FLAG) variable is defined, it takes precedence over `CMAKE_LINK_DEF_FILE_FLAG`.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
