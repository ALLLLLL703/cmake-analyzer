CMake 4.4.4 · [Official documentation](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LANG_LINK_DEF_FILE_FLAG.html)

# CMAKE_\<LANG\>_LINK_DEF_FILE_FLAG

Added in version 4.1.

Linker flag to be used to specify a `.def` file for dll creation with the toolchain for language `<LANG>`.

CMake sets this variable automatically during toolchain inspection by calls to the [`project()`](https://cmake.org/cmake/help/v4.4/command/project.html#command:project) or [`enable_language()`](https://cmake.org/cmake/help/v4.4/command/enable_language.html#command:enable_language) commands.

If the `CMAKE_<LANG>_LINK_DEF_FILE_FLAG` variable is defined, it takes precedence over the language-agnostic [`CMAKE_LINK_DEF_FILE_FLAG`](https://cmake.org/cmake/help/v4.4/variable/CMAKE_LINK_DEF_FILE_FLAG.html#variable:CMAKE_LINK_DEF_FILE_FLAG) variable.

<!-- Script-generated content. Generator: scripts/gen/cmake_builtin_docs.py. -->
